use super::targets::CodegenTarget;
use anyhow::Result;

pub struct AsmTarget;
impl CodegenTarget for AsmTarget {
    fn generate(&self, ast: &[crate::parser::ASTNode]) -> Result<()> {
        // TODO: generate x86-64 asssembly
        Ok(())
    }
}
