use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(shard::codegen))]
pub enum CodegenError {
    #[error("Unsupported AST node: {node_type}")]
    UnsupportedNode { node_type: String },

    #[error("Codegen error: {0}")]
    ParseError(String),
}

pub type CodegenResult<T> = std::result::Result<T, CodegenError>;
