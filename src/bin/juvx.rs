use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "juvx",
    version,
    about = "Run an executable JAR resolved from Maven coordinates"
)]
struct Cli {
    /// Maven coordinate to resolve and run (groupId:artifactId[:classifier]:version).
    coordinate: String,

    /// Additional repository (id=url format or bare URL).
    #[arg(long = "repo", alias = "repos")]
    repos: Vec<String>,

    /// Override dependency cache directory.
    #[arg(long = "cache-dir")]
    cache_dir: Option<PathBuf>,

    /// Main class to launch with the resolved classpath instead of java -jar.
    #[arg(long = "main")]
    main_class: Option<String>,

    /// Arguments passed to the launched Java tool after `--`.
    #[arg(last = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let code = juv::juvx::run(juv::juvx::JuvxOptions {
        coordinate: cli.coordinate,
        repos: cli.repos,
        cache_dir: cli.cache_dir,
        main_class: cli.main_class,
        args: cli.args,
    })?;
    std::process::exit(code);
}
