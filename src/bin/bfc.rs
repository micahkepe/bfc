//! # Brainf%ck Compiler (`bcf`)
//!
//! A Brainf%ck compiler targeting x86-64 ISA.
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use std::fs;
use std::process::Command;

use bfc::{codegen::CodegenTarget, *};

#[derive(Parser, Debug)]
#[command(about = "Brainf*ck compiler targeting x86-64", version)]
struct Args {
    /// The path string to the Brainf%ck source file
    input: std::path::PathBuf,
    /// The path string to the output executable
    #[arg(short, long, default_value = "a.out")]
    output: std::path::PathBuf,
    /// Verbosity flag
    #[command(flatten)]
    verbosity: Verbosity,
    /// Whether to assemble and link the generated '.asm' file
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

    // Generate x86-64 code
    let asm_target = codegen::asm::AsmTarget::new(args.output.with_extension("asm"));
    asm_target.generate(&ast)?;

    // Execute flag handling
    if args.execute {
        // Assemble
        let asm_file = args.output.with_extension("asm");
        let obj_file = args.output.with_extension("o");
        let status = Command::new("nasm")
            .args([
                "-f",
                "macho64",
                asm_file.to_str().unwrap(),
                "-o",
                obj_file.to_str().unwrap(),
            ])
            .status()
            .context("Failed to run `nasm`")?;
        if !status.success() {
            return Err(anyhow::anyhow!("Assembly failed").into());
        }

        // Link
        let mut clang_args = vec![
            "-arch",
            "x86_64",
            "-e",
            "_main",
            "-o",
            args.output.to_str().unwrap(),
            obj_file.to_str().unwrap(),
        ];
        if args.verbosity.is_present() {
            clang_args.insert(0, "-v");
        }
        let status = Command::new("clang")
            .args([
                "-g",
                "-arch",
                "x86_64",
                "-e",
                "_main",
                "-o",
                args.output.to_str().unwrap(),
                obj_file.to_str().unwrap(),
            ])
            .status()
            .context("Failed to run clang")?;
        if !status.success() {
            return Err(anyhow::anyhow!("Linking failed").into());
        }
    }

    Ok(())
}
