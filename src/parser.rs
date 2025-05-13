//! # Parser and Syntax Analysis
//!
//! `parser` defines a simple, unoptimized parser/ semantic analysis function
//! `parse` that is used to turn a stream of BF tokens into an AST, propagating
//! semantic errors if encountered.
use anyhow::Result;
use thiserror::Error;

use crate::lexer::Token;

/// Represents a single node on the created AST for the BF program.
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    /// '-' symbol
    Increment,
    /// '-' symbol
    Decrement,
    /// '<' symbol
    MoveLeft,
    /// '>' symbol
    MoveRight,
    /// ',' symbol
    Read,
    /// '.' symbol
    Write,
    /// Matched '[' and ']' symbols with zero or more ASTNode tokens in their
    /// body
    Loop(Vec<ASTNode>),
}

/// Syntactic error encountered while parser.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Additional ']' without matching loop start symbol
    #[error("Unmartched ']' at position '{0}'")]
    UnmatchedLoopEnd(usize),
    /// Additional '[' without matching loop end symbol
    #[error("Unmartched '[' at position '{0}'")]
    UnmatchedLoopStart(usize),
}

/// Parses a given slice of `Token` values into an abstract syntax tree
pub fn parse(tokens: &[Token]) -> Result<Vec<ASTNode>> {
    let mut ast = Vec::new();

    // stack to check validity of loop terminals
    // will hold (loop_start_idx, "loop body")
    let mut stack: Vec<(usize, Vec<ASTNode>)> = Vec::new();

    for (idx, token) in tokens.iter().enumerate() {
        match token {
            // handle non-loop tokens (simply push onto the AST)
            Token::Increment => add_ast_node(&mut ast, &mut stack, ASTNode::Increment),
            Token::Decrement => add_ast_node(&mut ast, &mut stack, ASTNode::Decrement),
            Token::MoveLeft => add_ast_node(&mut ast, &mut stack, ASTNode::MoveLeft),
            Token::MoveRight => add_ast_node(&mut ast, &mut stack, ASTNode::MoveRight),
            Token::Read => add_ast_node(&mut ast, &mut stack, ASTNode::Read),
            Token::Write => add_ast_node(&mut ast, &mut stack, ASTNode::Write),

            // handle loop operators
            Token::LoopStart => {
                stack.push((idx, Vec::new()));
            }
            Token::LoopEnd => {
                if let Some((_, loop_body)) = stack.pop() {
                    let loop_node = ASTNode::Loop(loop_body);
                    add_ast_node(&mut ast, &mut stack, loop_node);
                } else {
                    // unmatched case
                    return Err(ParseError::UnmatchedLoopEnd(idx).into());
                }
            }
        }
    }

    // check that stack is empty after parsing (all loop bodies are balanced
    // properly)
    if !stack.is_empty() {
        let (start_idx, _) = stack.pop().unwrap();
        return Err(ParseError::UnmatchedLoopStart(start_idx).into());
    }

    Ok(ast)
}

/// Adds a node to AST, checking if the program is in the middle of some
/// arbitrarily nested loop context via the stack.
fn add_ast_node(ast: &mut Vec<ASTNode>, stack: &mut [(usize, Vec<ASTNode>)], node: ASTNode) {
    // can peek to the top of the stack to see if the current node is within a
    // loop body or not
    if let Some((_, curr)) = stack.last_mut() {
        curr.push(node);
    } else {
        ast.push(node);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::Token;
    use anyhow::Result;

    #[test]
    fn test_simple_program() -> Result<()> {
        // "++[->+]"
        let tokens = vec![
            Token::Increment,
            Token::Increment,
            Token::LoopStart,
            Token::Decrement,
            Token::MoveRight,
            Token::Increment,
            Token::MoveLeft,
            Token::LoopEnd,
        ];
        let ast = parse(&tokens)?;
        assert_eq!(
            ast,
            vec![
                ASTNode::Increment,
                ASTNode::Increment,
                ASTNode::Loop(vec![
                    ASTNode::Decrement,
                    ASTNode::MoveRight,
                    ASTNode::Increment,
                    ASTNode::MoveLeft
                ])
            ]
        );
        Ok(())
    }
}
