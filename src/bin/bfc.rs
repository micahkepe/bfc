//! # Brainfuck Compiler (`bcf`)
//!
//! A Brainfuck compiler targeting x86-64 ISA.
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use std::fs;

use bfc::*;

#[derive(Parser, Debug)]
struct Args {
    /// The path string to the Brainfuck source file
    #[arg(short, long)]
    input: std::path::PathBuf,

    /// The path string to the output executable
    #[arg(short, long)]
    output: std::path::PathBuf,

    /// Verbosity flag
    #[command(flatten)]
    verbosity: Verbosity,

    /// Whether to assemble, link, and run the generated '.asm' file
    #[arg(short, long)]
    execute: bool,
}

/// Entry point for the compiler
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Instantiate the logger
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    log::debug!("path: {:?}, output: {:?}", args.input, args.output);

    // Read in the input program
    let source: String = fs::read_to_string(&args.input)
        .with_context(|| format!("unable to read file `{:#?}`", args.input.display()))?;

    // Tokenize the source code
    let tokens = lexer::tokenize(&source);
    log::debug!("Tokens: {:?}", &tokens);

    // Parse to create AST from the tokens and perform semantic analysis
    let ast = parser::parse(&tokens)?;
    log::debug!("AST: {:?}", &ast);

    // TODO: Generate x86-64 code

    // TODO: Execute flag handling
    // if args.execute {...}

    Ok(())
}
