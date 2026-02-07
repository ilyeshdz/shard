use clap::{Parser, Subcommand, ValueEnum};
use shard::{generate, parse, tokenize, ShardError};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Shell,
    Json,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check (lint) a Shard file without generating output
    Check {
        /// Input file path
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,

        /// Output format (shell or json)
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Build a Shard file to a shell script
    Build {
        /// Input file path
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,

        /// Output file path (default: <input>.sh)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Make the output script executable
        #[arg(long)]
        executable: bool,
    },

    /// Transpile and print to stdout
    Transpile {
        /// Input file path (or stdin if not specified)
        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Initialize a new Shard project
    Init {
        /// Project name
        #[arg(default_value = ".")]
        name: String,
    },
}

#[derive(Debug, Parser)]
#[command(name = "shard")]
#[command(author = "Ilyes Hernandez")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A minimalist shell orchestration language that transpiles to POSIX shell scripts", 
          long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn handle_check(args: &Commands, verbose: bool) -> Result<(), ShardError> {
    if let Commands::Check { input, format } = args {
        let input = std::fs::read_to_string(input)?;

        if verbose {
            eprintln!("Checking: {:?}", input);
        }

        let tokens = tokenize(&input)?;
        if verbose {
            eprintln!("Tokenized {} tokens", tokens.len());
        }

        let ast = parse(tokens)?;
        if verbose {
            eprintln!("Parsed {} statements", ast.0.len());
        }

        let format = format.unwrap_or(OutputFormat::Shell);

        match format {
            OutputFormat::Shell => {
                let shell = generate(&ast)?;
                println!(
                    "✓ Check passed - {} statements, {} chars",
                    ast.0.len(),
                    shell.len()
                );
            }
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(&ast)?;
                println!("{}", json);
            }
        }

        Ok(())
    } else {
        unreachable!()
    }
}

fn handle_build(args: &Commands, verbose: bool) -> Result<(), ShardError> {
    if let Commands::Build {
        input,
        output,
        executable,
    } = args
    {
        let input_str = std::fs::read_to_string(input)?;

        if verbose {
            eprintln!("Building: {:?}", input);
        }

        let tokens = tokenize(&input_str)?;
        let ast = parse(tokens)?;
        let shell = generate(&ast)?;

        let output_path = output.clone().unwrap_or_else(|| input.with_extension("sh"));

        std::fs::write(&output_path, &shell)?;

        if *executable {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&output_path, std::fs::Permissions::from_mode(0o755))?;
            }
            println!("✓ Built {} (executable)", output_path.display());
        } else {
            println!("✓ Built {}", output_path.display());
        }

        Ok(())
    } else {
        unreachable!()
    }
}

fn handle_transpile(args: &Commands, verbose: bool) -> Result<(), ShardError> {
    if let Commands::Transpile { input, format } = args {
        let input_str = if let Some(path) = input {
            std::fs::read_to_string(path)?
        } else {
            std::io::read_to_string(std::io::stdin())?
        };

        if verbose {
            eprintln!("Transpiling {} bytes", input_str.len());
        }

        let tokens = tokenize(&input_str)?;
        let ast = parse(tokens)?;
        let format = format.unwrap_or(OutputFormat::Shell);

        match format {
            OutputFormat::Shell => {
                let shell = generate(&ast)?;
                print!("{}", shell);
            }
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(&ast)?;
                println!("{}", json);
            }
        }

        Ok(())
    } else {
        unreachable!()
    }
}

fn handle_init(args: &Commands) -> Result<(), ShardError> {
    if let Commands::Init { name } = args {
        let dir = PathBuf::from(name);

        if dir.exists() && dir.read_dir()?.next().is_some() {
            eprintln!("Warning: Directory '{}' is not empty", name);
        }

        std::fs::create_dir_all(&dir)?;

        let shard_file = dir.join("main.shard");
        if !shard_file.exists() {
            std::fs::write(&shard_file, include_str!("../templates/main.shard"))?;
            println!("✓ Created {}", shard_file.display());
        }

        let readme = dir.join("README.md");
        if !readme.exists() {
            std::fs::write(&readme, include_str!("../templates/README.md"))?;
            println!("✓ Created {}", readme.display());
        }

        println!("\nTo build:");
        println!("  cargo run -- build -i {}", shard_file.display());
        println!("  or");
        println!("  shard build -i {}", shard_file.display());

        Ok(())
    } else {
        unreachable!()
    }
}

fn main() -> Result<(), ShardError> {
    let args = Args::parse();

    match &args.command {
        Commands::Check { .. } => handle_check(&args.command, args.verbose)?,
        Commands::Build { .. } => handle_build(&args.command, args.verbose)?,
        Commands::Transpile { .. } => handle_transpile(&args.command, args.verbose)?,
        Commands::Init { .. } => handle_init(&args.command)?,
    }

    Ok(())
}
