use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoalesceError>;

#[derive(Error, Debug)]
pub enum CoalesceError {
    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError {
        message: String,
        line: u32,
        column: u32,
    },
    
    #[error("Generation error: {0}")]
    GenerationError(String),
    
    #[error("ML processing error: {0}")]
    MLError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Unsupported language: {0:?}")]
    UnsupportedLanguage(crate::types::Language),
    
    #[error("Transformation error: {0}")]
    TransformationError(String),
    
    #[error("Legacy pattern preservation failed: {pattern}")]
    LegacyPatternError { pattern: String },
}
