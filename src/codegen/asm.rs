use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;

use super::targets::CodegenTarget;
use crate::parser::ASTNode;

pub struct AsmTarget {
    output_path: std::path::PathBuf,
}

impl AsmTarget {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        AsmTarget { output_path }
    }

    /// returns true if any node (even nested in loops) is `Read`
    fn contains_read(nodes: &[ASTNode]) -> bool {
        nodes.iter().any(|n| match n {
            ASTNode::Read => true,
            ASTNode::Loop(body) => AsmTarget::contains_read(body),
            _ => false,
        })
    }

    fn generate_nodes(file: &mut File, nodes: &[ASTNode], loop_counter: &mut usize) -> Result<()> {
        let mut i = 0;
        while i < nodes.len() {
            match nodes[i] {
                ASTNode::Increment => {
                    let mut count = 1;
                    while i + 1 < nodes.len() && matches!(nodes[i + 1], ASTNode::Increment) {
                        count += 1;
                        i += 1;
                    }
                    if count == 1 {
                        writeln!(file, "    inc byte [r12]")?;
                    } else {
                        writeln!(file, "    add byte [r12], {}", count)?;
                    }
                }
                ASTNode::Decrement => {
                    let mut count = 1;
                    while i + 1 < nodes.len() && matches!(nodes[i + 1], ASTNode::Decrement) {
                        count += 1;
                        i += 1;
                    }
                    if count == 1 {
                        writeln!(file, "    dec byte [r12]")?;
                    } else {
                        writeln!(file, "    sub byte [r12], {}", count)?;
                    }
                }
                ASTNode::MoveRight => {
                    let mut count = 1;
                    while i + 1 < nodes.len() && matches!(nodes[i + 1], ASTNode::MoveRight) {
                        count += 1;
                        i += 1;
                    }
                    writeln!(file, "    add r12, {}", count)?;
                    // Bounds check
                    writeln!(file, "    cmp r12, rsp")?;
                    writeln!(file, "    jb bounds_error")?;
                    writeln!(file, "    lea rax, [rsp + 30000]")?;
                    writeln!(file, "    cmp r12, rax")?;
                    writeln!(file, "    jae bounds_error")?;
                }
                ASTNode::MoveLeft => {
                    let mut count = 1;
                    while i + 1 < nodes.len() && matches!(nodes[i + 1], ASTNode::MoveLeft) {
                        count += 1;
                        i += 1;
                    }
                    writeln!(file, "    sub r12, {}", count)?;
                    // Bounds check
                    writeln!(file, "    cmp r12, rsp")?;
                    writeln!(file, "    jb bounds_error")?;
                    writeln!(file, "    lea rax, [rsp + 30000]")?;
                    writeln!(file, "    cmp r12, rax")?;
                    writeln!(file, "    jae bounds_error")?;
                }
                ASTNode::Write => {
                    writeln!(file, "    mov rax, 0x2000004")?; // sys_write on macOS
                    writeln!(file, "    mov rdi, 1")?;
                    writeln!(file, "    mov rsi, r12")?;
                    writeln!(file, "    mov rdx, 1")?;
                    writeln!(file, "    syscall")?;
                }
                ASTNode::Read => {
                    // read one byte into [r12]
                    writeln!(file, "    mov rax, 0x2000003")?; // sys_read
                    writeln!(file, "    mov rdi, 0")?; // stdin
                    writeln!(file, "    mov rsi, r12")?; // buffer
                    writeln!(file, "    mov rdx, 1")?; // count
                    writeln!(file, "    syscall")?;
                }
                ASTNode::Loop(ref body) => {
                    let loop_id = *loop_counter;
                    *loop_counter += 1;
                    writeln!(file, "loop_start_{}:", loop_id)?;
                    writeln!(file, "    cmp byte [r12], 0")?;
                    writeln!(file, "    je loop_end_{}", loop_id)?;
                    AsmTarget::generate_nodes(file, body, loop_counter)?;
                    writeln!(file, "    jmp loop_start_{}", loop_id)?;
                    writeln!(file, "loop_end_{}:", loop_id)?;
                }
            }
            i += 1;
        }
        Ok(())
    }
}

impl CodegenTarget for AsmTarget {
    fn generate(&self, ast: &[ASTNode]) -> Result<()> {
        let mut file = File::create(&self.output_path)
            .with_context(|| format!("Failed to create output file: {:?}", self.output_path))?;

        // Write assembly header
        writeln!(file, "section .data")?;
        writeln!(file, "read_msg:    db 'Read: '")?;
        writeln!(file, "read_msg_end:")?;

        writeln!(file, "section .text")?;
        writeln!(file, "global _main")?;
        writeln!(file, "align 16")?;
        writeln!(file, "_main:")?;
        writeln!(file, "    sub rsp, 30000")?; // Allocate 30,000 bytes on stack for tape
        writeln!(file, "    mov r12, rsp")?; // Point r12 to tape start

        if AsmTarget::contains_read(ast) {
            // Input prompt "'Read: '"
            writeln!(file, "    ; — debug: print \"Read: \" once —")?;
            writeln!(file, "    mov rax, 0x2000004")?; // sys_write
            writeln!(file, "    mov rdi, 2")?; // stderr
            writeln!(file, "    lea rsi, [rel read_msg]")?; // pointer
            writeln!(file, "    mov rdx, read_msg_end - read_msg")?; // length = 6
            writeln!(file, "    syscall")?;
        }

        // Generate code for AST
        let mut loop_counter = 0;
        AsmTarget::generate_nodes(&mut file, ast, &mut loop_counter)?;

        // Exit program
        writeln!(file, "    add rsp, 30000")?; // Deallocate stack
        writeln!(file, "    mov rax, 0x2000001")?; // sys_exit on macOS
        writeln!(file, "    xor rdi, rdi")?;
        writeln!(file, "    syscall")?;

        // Bounds error handler
        writeln!(file, "bounds_error:")?;
        writeln!(file, "    mov rax, 0x2000001")?; // sys_exit
        writeln!(file, "    mov rdi, 1")?; // Exit code 1
        writeln!(file, "    syscall")?;

        // Read error handler
        writeln!(file, "read_error:")?;
        writeln!(file, "    mov byte [r12], 0")?; // Set to 0 on error
        writeln!(file, "    jmp read_done")?;
        writeln!(file, "read_eof:")?;
        writeln!(file, "    mov byte [r12], 0")?; // Set to 0 on EOF
        writeln!(file, "read_done:")?;

        Ok(())
    }
}
