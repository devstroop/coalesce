pub mod registry;
pub mod patterns;
pub mod transformer;
pub mod detector;

use crate::registry::LibraryRegistry;
use crate::detector::DependencyDetector;
use crate::transformer::LibraryTransformer;
use coalesce_core::{UIRNode, Language, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main entry point for the Library Abstraction Layer
pub struct LibraryAbstractionLayer {
    registry: LibraryRegistry,
    detector: DependencyDetector,
}

/// Represents a detected library dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDependency {
    pub name: String,
    pub version: Option<String>,
    pub ecosystem: String,
    pub import_path: Option<String>,
    pub usage_patterns: Vec<LibraryUsage>,
}

/// Specific usage of a library pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryUsage {
    pub pattern_name: String,
    pub method_name: String,
    pub parameters: HashMap<String, String>,
    pub semantic_intent: String,
    pub source_location: (usize, usize), // (start, end)
}

impl LibraryAbstractionLayer {
    pub fn new() -> Result<Self> {
        let mut registry = LibraryRegistry::new();
        registry.register_defaults()?;
        
        let detector = DependencyDetector::new();
        
        Ok(Self {
            registry,
            detector,
        })
    }
    
    /// Analyze source code to detect library dependencies
    pub fn analyze_dependencies(&self, code: &str, language: Language) -> Result<Vec<LibraryDependency>> {
        self.detector.detect_dependencies(code, language)
    }
    
    /// Enhance UIR nodes with library-specific metadata
    pub fn enhance_uir(&self, node: &mut UIRNode, deps: &[LibraryDependency]) -> Result<()> {
        for dep in deps {
            self.add_library_metadata(node, dep)?;
        }
        Ok(())
    }
    
    /// Transform library-specific patterns to target equivalents
    pub fn transform_library_calls(
        &self,
        node: &UIRNode,
        target_lang: Language,
        target_ecosystem: Option<&str>,
    ) -> Result<UIRNode> {
        let transformer = LibraryTransformer::new(&self.registry);
        transformer.transform(node, target_lang, target_ecosystem)
    }
    
    /// Get available target ecosystems for a source library
    pub fn get_target_ecosystems(&self, source_library: &str) -> Vec<String> {
        self.registry.get_target_ecosystems(source_library)
    }
    
    fn add_library_metadata(&self, node: &mut UIRNode, dep: &LibraryDependency) -> Result<()> {
        // Add library information to node metadata
        node.metadata.annotations.insert(
            "library_dependency".to_string(),
            serde_json::Value::String(serde_json::to_string(dep)?),
        );
        
        // Mark nodes that use library patterns
        for usage in &dep.usage_patterns {
            if let Some(ref node_name) = node.name {
                if *node_name == usage.method_name {
                    node.metadata.annotations.insert(
                        "library_pattern".to_string(),
                        serde_json::Value::String(usage.pattern_name.clone()),
                    );
                    node.metadata.annotations.insert(
                        "semantic_intent".to_string(),
                        serde_json::Value::String(usage.semantic_intent.clone()),
                    );
                }
            }
        }
        
        Ok(())
    }
}

impl Default for LibraryAbstractionLayer {
    fn default() -> Self {
        Self::new().expect("Failed to initialize LibraryAbstractionLayer")
    }
}
