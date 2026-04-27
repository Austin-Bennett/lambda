#![feature(try_trait_v2)]
#![feature(deref_pure_trait)]
#![feature(try_trait_v2_residual)]

use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::utils::runtime_static::RuntimeStatic;
use crate::compiler::Compiler;
use anyhow::Result;
use clap::Parser;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit};
use crate::build_args::{Arguments, BuildJSON, CompilerArgs};
use crate::intrinsics::add_intrinsics;

pub mod lexer;
pub mod common;
pub mod compiler;
pub mod ast;

pub mod typed_ast;
pub mod consteval;
pub mod intrinsics;
pub mod build_args;






fn main() -> Result<()> {
    let args = Arguments::parse();
    
    let args: Box<dyn CompilerArgs> = if let Some(bjson) = args.build {
        Box::new(BuildJSON::from_file(bjson))
    } else {
        Box::new(args)
    };




    let context = RuntimeStatic::new(inkwell::context::Context::create());
    let mut compiler = Compiler::new(RuntimeStatic::static_ref(&context));

    add_intrinsics(&mut compiler);




    for path in args.get_module_paths() {
        compiler.module_search_paths.push(std::path::PathBuf::from(path));
    }
    let search_paths = compiler.module_search_paths.clone();

    let mut extra_link_inputs: Vec<String> = Vec::new();

    for file in args.get_files() {
        let path = Path::new(file);
        match path.extension().and_then(|e| e.to_str()) {
            Some("lm") | None => {
                let resolved = if path.is_absolute() {
                    Some(path.to_path_buf())
                } else {
                    search_paths.iter().map(|sp| sp.join(path)).find(|p| p.exists())
                };

                match resolved {
                    Some(p) => match fs::read_to_string(&p) {
                        Ok(f) => compiler.add_module(
                            SourceOwner::new(SourceDescriptor::File, p.to_string_lossy().into_owned()),
                            f,
                        ),
                        Err(e) => eprintln!("Failed to read file: {} due to error: {:?}", file, e),
                    },
                    None => eprintln!("{}: file not found in any module search path", file),
                }
            }
            // Object files, archives, shared libs — pass directly to the linker
            _ => extra_link_inputs.push(file.clone()),
        }
    }



    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        exit(-1);
    }


    if args.get_debug_ast() {
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
        exit(-1);
    }

    if args.get_debug_ast() {
        println!("Typed AST:");
        for (mp, md) in compiler.get_typed_modules() {
            println!("Module {}:", mp);
            for f in &md.functions {
                println!("{} {}", f.signature.to_string(&compiler.type_context), f.to_string(&compiler.type_context));
            }
        }
    }

    if args.get_no_codegen() {
        return Ok(());
    }

    let llvm_mod = compiler.compile();




    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        exit(-1);
    }

    if let Some(ir) = args.get_output_ir() {
        llvm_mod.print_to_file(ir).unwrap();
    }

    if let Err(err) = llvm_mod.verify() {
        eprintln!("LLVM IR Verification Error: {}", err.to_string());
        // This will print exactly what is wrong with your GEP or types
        exit(-1);
    }


    // --- Codegen: LLVM IR -> native object file -> executable ---

    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow::anyhow!("Failed to initialize native target: {}", e))?;

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple)
        .map_err(|e| anyhow::anyhow!("Failed to get target: {}", e))?;

    let reloc_mode = if args.get_shared() { RelocMode::PIC } else { RelocMode::Default };

    let opt_level = match args.get_optimize().as_ref() {
        "1" => OptimizationLevel::Less,
        "2" => OptimizationLevel::Default,
        "3" => OptimizationLevel::Aggressive,
        _ => OptimizationLevel::None,
    };

    let machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            opt_level,
            reloc_mode,
            CodeModel::Default,
        )
        .ok_or_else(|| anyhow::anyhow!("Failed to create target machine"))?;

    // Write a temporary object file alongside the output.
    let obj_path = Path::new(&args.get_output()).with_extension("o");

    machine
        .write_to_file(&llvm_mod, FileType::Object, &obj_path)
        .map_err(|e| anyhow::anyhow!("Failed to write object file: {}", e))?;

    // Link with the system C compiler (handles libc, crt, etc. automatically).
    let mut link_cmd = std::process::Command::new("cc");
    if args.get_shared() {
        link_cmd.arg("-shared");
    }
    
    link_cmd.arg(obj_path.to_str().unwrap());
    for input in &extra_link_inputs {
        link_cmd.arg(input);
    }
    link_cmd.args(["-o", &args.get_output()]);
    for path in args.get_lib_paths() {
        link_cmd.args(["-L", path]);
    }
    if !args.get_shared() && !args.get_libs().is_empty() {
        link_cmd.arg("-Wl,-rpath,$ORIGIN");
    }
    
    for lib in args.get_libs() {
        link_cmd.args(["-l", lib]);
    }

    #[cfg(target_os = "linux")]
    link_cmd.args(["-l", "m"]);

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

    let artifact = if args.get_shared() { "shared library" } else { "executable" };
    println!("Successfully compiled {}: {} [optimization level: {}]", artifact, args.get_output(), args.get_optimize());

    Ok(())
}
