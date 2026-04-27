use std::fs;
use std::path::PathBuf;
use clap::Parser;

pub trait CompilerArgs {
    fn get_output(&self) -> &String;
    fn get_optimize(&self) -> &String;
    fn get_debug_ast(&self) -> bool;
    fn get_no_codegen(&self) -> bool;
    fn get_output_ir(&self) -> Option<&String>;
    fn get_shared(&self) -> bool;
    fn get_libs(&self) -> &Vec<String>;
    fn get_lib_paths(&self) -> &Vec<String>;
    fn get_module_paths(&self) -> &Vec<String>;
    fn get_files(&self) -> &Vec<String>;
}



#[derive(Parser)]
pub struct Arguments {
    /// Output path
    #[arg(short, long, default_value = "a.out")]
    output: String,

    /// Build json
    #[arg(long, short='B')]
    pub build: Option<PathBuf>,

    ///optimization level
    #[arg(long, short='O', value_parser = ["none", "1", "2", "3"], default_value = "2")]
    optimize: String,

    /// print the ast as debug output
    #[arg(long, default_value = "false")]
    debug_ast: bool,

    /// dont do codegen, only generate ast and ir
    #[arg(long, default_value = "false")]
    no_codegen: bool,

    /// generate ir in the specified output file
    #[arg(long)]
    output_ir: Option<String>,

    /// Compile as a shared library (.so / .dll)
    #[arg(long)]
    shared: bool,

    /// Link against a shared library (e.g. -l pthread)
    #[arg(short = 'l', long="link", value_name = "LIB", action = clap::ArgAction::Append)]
    libs: Vec<String>,

    /// Add a library search path (e.g. -L /usr/local/lib)
    #[arg(short = 'L', value_name = "PATH", action = clap::ArgAction::Append)]
    lib_paths: Vec<String>,

    /// Add a module search path (e.g. -M ./stdlib)
    #[arg(short = 'M', long = "modules", value_name = "PATH", action = clap::ArgAction::Append)]
    module_paths: Vec<String>,

    ///files to compile
    #[arg(num_args = 0.., required_unless_present = "build", value_name = "FILE")]
    files: Vec<String>,
}

impl CompilerArgs for Arguments {
    fn get_output(&self) -> &String {
        &self.output
    }

    fn get_optimize(&self) -> &String {
        &self.optimize
    }

    fn get_debug_ast(&self) -> bool {
        self.debug_ast
    }

    fn get_no_codegen(&self) -> bool {
        self.no_codegen
    }

    fn get_output_ir(&self) -> Option<&String> {
        self.output_ir.as_ref()
    }

    fn get_shared(&self) -> bool {
        self.shared
    }

    fn get_libs(&self) -> &Vec<String> {
        &self.libs
    }

    fn get_lib_paths(&self) -> &Vec<String> {
        &self.lib_paths
    }

    fn get_module_paths(&self) -> &Vec<String> {
        &self.module_paths
    }

    fn get_files(&self) -> &Vec<String> {
        &self.files
    }
}

pub struct BuildJSON {
    output: String,
    optimize: String,
    debug_ast: bool,
    no_codegen: bool,
    output_ir: Option<String>,
    shared: bool,
    libs: Vec<String>,
    lib_paths: Vec<String>,
    module_paths: Vec<String>,
    files: Vec<String>,
}

impl BuildJSON {
    pub fn from_file(f: PathBuf) -> Self {
        let Ok(txt) = fs::read_to_string(&f) else {
            panic!("Failed to read file: {:?}", f);
        };

        let json = match serde_json::from_str::<serde_json::Value>(&txt) {
            Ok(json) => {
                json
            },
            Err(e) => {
                panic!("Failed to parse json from file: {:?} due to error: {}", f, e)
            }
        };

        let str_field = |key: &str, default: &str| -> String {
            json.get(key).and_then(|v| v.as_str()).unwrap_or(default).to_string()
        };
        let bool_field = |key: &str| -> bool {
            json.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
        };
        let str_array = |key: &str| -> Vec<String> {
            json.get(key)
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect())
                .unwrap_or_default()
        };

        let output    = str_field("output", "a.out");
        let optimize  = str_field("optimize", "2");
        let debug_ast = bool_field("debug_ast");
        let no_codegen = bool_field("no_codegen");
        let output_ir = json.get("output_ir").and_then(|v| v.as_str()).map(str::to_owned);
        let shared    = bool_field("shared");
        let libs         = str_array("libs");
        let lib_paths    = str_array("lib_paths");
        let module_paths = str_array("module_paths");
        let files        = str_array("files");

        Self{
            output,
            optimize,
            debug_ast,
            no_codegen,
            output_ir,
            shared,
            libs,
            lib_paths,
            module_paths,
            files,
        }
    }
}

impl CompilerArgs for BuildJSON {
    fn get_output(&self) -> &String {
        &self.output
    }

    fn get_optimize(&self) -> &String {
        &self.optimize
    }

    fn get_debug_ast(&self) -> bool {
        self.debug_ast
    }

    fn get_no_codegen(&self) -> bool {
        self.no_codegen
    }

    fn get_output_ir(&self) -> Option<&String> {
        self.output_ir.as_ref()
    }

    fn get_shared(&self) -> bool {
        self.shared
    }

    fn get_libs(&self) -> &Vec<String> {
        &self.libs
    }

    fn get_lib_paths(&self) -> &Vec<String> {
        &self.lib_paths
    }

    fn get_module_paths(&self) -> &Vec<String> {
        &self.module_paths
    }

    fn get_files(&self) -> &Vec<String> {
        &self.files
    }
}