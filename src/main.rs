use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use doj::{build_java, run_java, BuildOptions, RunOptions};

#[derive(Parser, Debug)]
#[command(name = "doj", version, about = "do Java: a Rust port of JBang")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Script to run when no subcommand is given, JBang-style.
    script: Option<PathBuf>,

    /// Arguments passed to the script when no subcommand is given.
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile and run a Java source file.
    Run(RunCommand),
    /// Compile and store script in the cache without running it.
    Build(BuildCommand),
    /// Print parsed JBang directives.
    Info(InfoCommand),
}

#[derive(Parser, Debug)]
struct RunCommand {
    /// Additional dependency coordinates, same shape as //DEPS.
    #[arg(long = "deps")]
    deps: Vec<String>,

    /// Additional classpath entries.
    #[arg(long = "class-path", alias = "cp")]
    classpath: Vec<PathBuf>,

    /// Additional javac option.
    #[arg(long = "javac-option")]
    javac_options: Vec<String>,

    /// Additional java runtime option.
    #[arg(long = "runtime-option")]
    runtime_options: Vec<String>,

    /// Override //MAIN / inferred class name.
    #[arg(long = "main")]
    main_class: Option<String>,

    /// Override cache directory.
    #[arg(long = "cache-dir")]
    cache_dir: Option<PathBuf>,

    /// Java source file.
    script: PathBuf,

    /// Arguments passed to the script.
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Parser, Debug)]
struct BuildCommand {
    /// Additional dependency coordinates, same shape as //DEPS.
    #[arg(long = "deps")]
    deps: Vec<String>,

    /// Additional classpath entries.
    #[arg(long = "class-path", alias = "cp")]
    classpath: Vec<PathBuf>,

    /// Additional javac option.
    #[arg(long = "javac-option")]
    javac_options: Vec<String>,

    /// Override //MAIN / inferred class name.
    #[arg(long = "main")]
    main_class: Option<String>,

    /// Override cache directory.
    #[arg(long = "cache-dir")]
    cache_dir: Option<PathBuf>,

    /// Java source file.
    script: PathBuf,
}

#[derive(Parser, Debug)]
struct InfoCommand {
    /// Java source file.
    script: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let code = match cli.command {
        Some(Commands::Run(cmd)) => run_java(RunOptions {
            script: cmd.script,
            script_args: cmd.args,
            extra_deps: cmd.deps,
            classpath: cmd.classpath,
            javac_options: cmd.javac_options,
            runtime_options: cmd.runtime_options,
            main_class: cmd.main_class,
            cache_dir: cmd.cache_dir,
        })?,
        Some(Commands::Build(cmd)) => {
            build_java(BuildOptions {
                script: cmd.script,
                extra_deps: cmd.deps,
                classpath: cmd.classpath,
                javac_options: cmd.javac_options,
                main_class: cmd.main_class,
                cache_dir: cmd.cache_dir,
            })?;
            0
        }
        Some(Commands::Info(cmd)) => {
            let source = std::fs::read_to_string(&cmd.script)?;
            println!("{:#?}", doj::parse_directives(&source));
            0
        }
        None => {
            let Some(script) = cli.script else {
                eprintln!("No script specified. Try: doj run Hello.java");
                std::process::exit(2);
            };
            run_java(RunOptions {
                script,
                script_args: cli.args,
                extra_deps: Vec::new(),
                classpath: Vec::new(),
                javac_options: Vec::new(),
                runtime_options: Vec::new(),
                main_class: None,
                cache_dir: None,
            })?
        }
    };
    std::process::exit(code);
}
