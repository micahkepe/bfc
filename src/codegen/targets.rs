use crate::parser::ASTNode;
use anyhow::Result;

/// The target language for code generation.
pub trait CodegenTarget {
    /// Generates the target language source code given the AST.
    fn generate(&self, ast: &[ASTNode]) -> Result<()>;
}
