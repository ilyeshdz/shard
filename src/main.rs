use clap::Parser;
use shard::{generate, parse, tokenize, ShardError};

#[derive(Parser, Debug)]
#[command(name = "shard")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    check: bool,
}

fn main() -> Result<(), ShardError> {
    let args = Args::parse();

    let input = if let Some(path) = &args.input {
        std::fs::read_to_string(path)?
    } else {
        eprintln!("Error: No input file specified. Use --input <file>");
        std::process::exit(1);
    };

    let tokens = tokenize(&input)?;
    let ast = parse(tokens)?;
    let shell = generate(&ast)?;

    if args.check {
        println!("{}", shell);
    } else if let Some(output) = &args.output {
        std::fs::write(output, &shell)?;
        println!("Written to {}", output);
    } else {
        println!("{}", shell);
    }

    Ok(())
}
