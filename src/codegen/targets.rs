use crate::parser::ASTNode;
use anyhow::Result;

pub trait CodegenTarget {
    fn generate(&self, ast: &[ASTNode]) -> Result<()>;
}
