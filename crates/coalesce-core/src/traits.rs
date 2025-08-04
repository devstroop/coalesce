use crate::{UIRNode, Language};
use crate::errors::Result;

/// Trait for language parsers
pub trait Parser {
    /// The language this parser handles
    fn language(&self) -> Language;
    
    /// Parse source code into UIR
    fn parse(&self, source: &str) -> Result<UIRNode>;
    
    /// Parse a specific file
    fn parse_file(&self, file_path: &str) -> Result<UIRNode> {
        let source = std::fs::read_to_string(file_path)?;
        self.parse(&source)
    }
}

/// Trait for code generators
pub trait Generator {
    /// The target language this generator produces
    fn target_language(&self) -> Language;
    
    /// Generate code from UIR
    fn generate(&self, uir: &UIRNode) -> Result<String>;
    
    /// Generate code and write to file
    fn generate_file(&self, uir: &UIRNode, output_path: &str) -> Result<()> {
        let code = self.generate(uir)?;
        std::fs::write(output_path, code)?;
        Ok(())
    }
}

/// Trait for ML models that enhance UIR
pub trait MLEnhancer {
    /// Add embeddings and semantic understanding to UIR
    fn enhance(&self, uir: &mut UIRNode) -> Result<()>;
    
    /// Suggest improvements or modernizations
    fn suggest_improvements(&self, uir: &UIRNode) -> Result<Vec<String>>;
}
