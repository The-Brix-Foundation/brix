use std::env;
use std::fs;
use std::process::Command;

use brix::lexer::scanner::Scanner;
use brix::parser::parser::Parser;
use brix::codegen::native::Codegen;
use inkwell::context::Context;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: brixc <file.br>");
        std::process::exit(1);
    }

    let source_path = &args[1];

    let source = fs::read_to_string(source_path).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", source_path, e);
        std::process::exit(1);
    });

    let tokens = Scanner::new(&source).scan_tokens().unwrap_or_else(|e| {
        eprintln!("Lexer error: {}", e);
        std::process::exit(1);
    });

    let ast = Parser::new(tokens).parse().unwrap_or_else(|e| {
        eprintln!("Parser error: {}", e);
        std::process::exit(1);
    });

    let context = Context::create();
    let codegen = Codegen::new(&context, "brix_module");

    codegen.generate(&ast).unwrap_or_else(|e| {
        eprintln!("Codegen error: {}", e);
        std::process::exit(1);
    });

    codegen.verify().unwrap_or_else(|e| {
        eprintln!("LLVM verification error: {}", e);
        std::process::exit(1);
    });

    let obj_path = "output.o";
    codegen.write_object_file(obj_path).unwrap_or_else(|e| {
        eprintln!("Object file error: {}", e);
        std::process::exit(1);
    });

    let output_name = source_path
        .trim_end_matches(".br")
        .to_string();

    let status = Command::new("clang")
        .args([obj_path, "-o", &output_name])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Linker error: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        eprintln!("Linking failed");
        std::process::exit(1);
    }

    fs::remove_file(obj_path).ok();

    println!("Compiled '{}' -> '{}'", source_path, output_name);
}