#![feature(try_trait_v2)]
#![feature(deref_pure_trait)]

use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::utils::runtime_static::RuntimeStatic;
use crate::compiler::Compiler;
use anyhow::Result;
use clap::Parser;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use std::fs;
use std::path::Path;
use std::process::abort;

pub mod lexer;
pub mod common;
pub mod compiler;
pub mod ast;
#[cfg(test)]
pub mod tests;
pub mod typed_ast;
pub mod consteval;

#[derive(Parser)]
pub struct Arguments {
    /// Output path
    #[arg(short, long, default_value = "a.out")]
    output: String,

    #[arg(long, default_value = "false")]
    debug_ast: bool,

    #[arg(long, default_value = "false")]
    no_codegen: bool,

    /// Compile as a shared library (.so / .dll)
    #[arg(long)]
    shared: bool,

    /// Link against a shared library (e.g. -l pthread)
    #[arg(short = 'l', long="link", value_name = "LIB", action = clap::ArgAction::Append)]
    libs: Vec<String>,

    /// Add a library search path (e.g. -L /usr/local/lib)
    #[arg(short = 'L', value_name = "PATH", action = clap::ArgAction::Append)]
    lib_paths: Vec<String>,

    #[arg(num_args = 1.., value_name = "FILE")]
    files: Vec<String>,
}




fn main() -> Result<()> {
    let args = Arguments::parse();

    let context = RuntimeStatic::new(inkwell::context::Context::create());
    let mut compiler = Compiler::new(RuntimeStatic::static_ref(&context));

    for file in &args.files {
        match fs::read_to_string(file) {
            Ok(f) => {
                compiler.add_module(
                    SourceOwner::new(SourceDescriptor::File, file.clone()),
                    f,
                )
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }



    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        abort();
    }


    if args.debug_ast {
        println!("Untyped AST:");
        for (mp, md) in compiler.get_untyped_modules() {
            println!("Module {}:", mp);
            for item in &md.ast {
                println!("{:?}", item)
            }
        }
        println!();
    }

    compiler.create_typed_ast();

    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        abort();
    }

    if args.debug_ast {
        println!("Typed AST:");
        for (mp, md) in compiler.get_typed_modules() {
            println!("Module {}:", mp);
            for f in &md.functions {
                println!("{} {}", f.signature.to_string(&compiler.type_context), f.to_string(&compiler.type_context));
            }
        }
    }

    if args.no_codegen {
        return Ok(());
    }

    let llvm_mod = compiler.compile();




    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        abort();
    }

    if let Err(err) = llvm_mod.verify() {
        eprintln!("LLVM IR Verification Error: {}", err.to_string());
        // This will print exactly what is wrong with your GEP or types
        abort();
    }

    llvm_mod.print_to_file("intermediate.llvm").unwrap();

    // --- Codegen: LLVM IR -> native object file -> executable ---

    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow::anyhow!("Failed to initialize native target: {}", e))?;

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple)
        .map_err(|e| anyhow::anyhow!("Failed to get target: {}", e))?;

    let reloc_mode = if args.shared { RelocMode::PIC } else { RelocMode::Default };

    let machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            reloc_mode,
            CodeModel::Default,
        )
        .ok_or_else(|| anyhow::anyhow!("Failed to create target machine"))?;

    // Write a temporary object file alongside the output.
    let obj_path = Path::new(&args.output).with_extension("o");

    machine
        .write_to_file(&llvm_mod, FileType::Object, &obj_path)
        .map_err(|e| anyhow::anyhow!("Failed to write object file: {}", e))?;

    // Link with the system C compiler (handles libc, crt, etc. automatically).
    let mut link_cmd = std::process::Command::new("cc");
    if args.shared {
        link_cmd.arg("-shared");
    }
    
    link_cmd.args([obj_path.to_str().unwrap(), "-o", &args.output]);
    for path in &args.lib_paths {
        link_cmd.args(["-L", path]);
    }
    if !args.shared && !args.libs.is_empty() {
        link_cmd.arg("-Wl,-rpath,$ORIGIN");
    }
    
    for lib in &args.libs {
        link_cmd.args(["-l", lib]);
    }

    let link_status = link_cmd
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to invoke linker: {}", e))?;

    fs::remove_file(&obj_path)?;

    if !link_status.success() {
        anyhow::bail!(
            "Linking failed (exit code {})",
            link_status.code().unwrap_or(-1)
        );
    }

    let artifact = if args.shared { "shared library" } else { "executable" };
    println!("Successfully compiled {} -> {}", artifact, args.output);

    Ok(())
}
