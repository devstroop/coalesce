use crate::patterns::{LibraryPattern, PatternLibrary};
use coalesce_core::{Result, CoalesceError};
use std::collections::HashMap;
use serde_yaml;

/// Registry for managing library patterns and transformations
pub struct LibraryRegistry {
    patterns: HashMap<String, HashMap<String, LibraryPattern>>,
    ecosystems: HashMap<String, Vec<String>>,
}

impl LibraryRegistry {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            ecosystems: HashMap::new(),
        }
    }
    
    /// Register default library patterns
    pub fn register_defaults(&mut self) -> Result<()> {
        // Register React patterns
        for pattern in PatternLibrary::react_patterns() {
            self.register_pattern(pattern)?;
        }
        
        // Register Django patterns
        for pattern in PatternLibrary::django_patterns() {
            self.register_pattern(pattern)?;
        }
        
        // Register networking patterns
        for pattern in PatternLibrary::networking_patterns() {
            self.register_pattern(pattern)?;
        }
        
        // Register ecosystem mappings
        self.register_ecosystem_mappings();
        
        Ok(())
    }
    
    /// Register a library pattern
    pub fn register_pattern(&mut self, pattern: LibraryPattern) -> Result<()> {
        let library_patterns = self.patterns
            .entry(pattern.library.clone())
            .or_insert_with(HashMap::new);
        
        library_patterns.insert(pattern.name.clone(), pattern);
        Ok(())
    }
    
    /// Get a specific pattern by library and pattern name
    pub fn get_pattern(&self, library: &str, pattern_name: &str) -> Option<&LibraryPattern> {
        self.patterns
            .get(library)?
            .get(pattern_name)
    }
    
    /// Get all patterns for a library
    pub fn get_library_patterns(&self, library: &str) -> Option<&HashMap<String, LibraryPattern>> {
        self.patterns.get(library)
    }
    
    /// Get available target ecosystems for a source library
    pub fn get_target_ecosystems(&self, source_library: &str) -> Vec<String> {
        self.ecosystems
            .get(source_library)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Register library from YAML configuration
    pub fn register_from_yaml(&mut self, yaml_config: &str) -> Result<()> {
        let pattern: LibraryPattern = serde_yaml::from_str(yaml_config)
            .map_err(|e| CoalesceError::TransformationError(format!("YAML parse error: {}", e)))?;
        self.register_pattern(pattern)?;
        Ok(())
    }
    
    /// Find equivalent patterns across ecosystems
    pub fn find_equivalent_patterns(&self, semantic_intent: &str) -> Vec<&LibraryPattern> {
        let mut equivalents = Vec::new();
        
        for library_patterns in self.patterns.values() {
            for pattern in library_patterns.values() {
                if pattern.semantics.intent == semantic_intent {
                    equivalents.push(pattern);
                }
            }
        }
        
        equivalents
    }
    
    /// Get transformation suggestions for a pattern
    pub fn get_transformation_suggestions(
        &self,
        source_library: &str,
        pattern_name: &str,
        target_ecosystem: &str,
    ) -> Vec<TransformationSuggestion> {
        let mut suggestions = Vec::new();
        
        if let Some(pattern) = self.get_pattern(source_library, pattern_name) {
            // Direct transformation
            if pattern.transformations.contains_key(target_ecosystem) {
                suggestions.push(TransformationSuggestion {
                    confidence: 1.0,
                    suggestion_type: SuggestionType::DirectTransform,
                    target_library: pattern.transformations[target_ecosystem].target_library.clone(),
                    target_pattern: pattern.transformations[target_ecosystem].target_pattern.clone(),
                    description: format!("Direct transformation to {}", target_ecosystem),
                });
            }
            
            // Semantic equivalent
            for equiv_pattern in self.find_equivalent_patterns(&pattern.semantics.intent) {
                if equiv_pattern.ecosystem == target_ecosystem && equiv_pattern.library != source_library {
                    suggestions.push(TransformationSuggestion {
                        confidence: 0.8,
                        suggestion_type: SuggestionType::SemanticEquivalent,
                        target_library: equiv_pattern.library.clone(),
                        target_pattern: equiv_pattern.name.clone(),
                        description: format!("Semantic equivalent: {}", equiv_pattern.name),
                    });
                }
            }
        }
        
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions
    }
    
    fn register_ecosystem_mappings(&mut self) {
        // JavaScript ecosystem mappings
        self.ecosystems.insert("react".to_string(), vec![
            "vue".to_string(),
            "svelte".to_string(),
            "angular".to_string(),
            "vanilla".to_string(),
        ]);
        
        // Python ecosystem mappings
        self.ecosystems.insert("django".to_string(), vec![
            "sqlalchemy".to_string(),
            "fastapi".to_string(),
            "flask".to_string(),
        ]);
        
        // Cross-platform networking
        self.ecosystems.insert("socket".to_string(), vec![
            "rust".to_string(),
            "go".to_string(),
            "python".to_string(),
            "javascript".to_string(),
        ]);
    }
}

/// Suggestion for transforming a library pattern
#[derive(Debug, Clone)]
pub struct TransformationSuggestion {
    pub confidence: f32,
    pub suggestion_type: SuggestionType,
    pub target_library: String,
    pub target_pattern: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum SuggestionType {
    DirectTransform,
    SemanticEquivalent,
    ManualImplementation,
    PartialTransform,
}

impl Default for LibraryRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register_defaults().expect("Failed to register default patterns");
        registry
    }
}
