use crate::{LibraryDependency, patterns::{LibraryPattern, TransformRule}};
use crate::registry::LibraryRegistry;
use coalesce_core::{UIRNode, Language, Result, CoalesceError};
use std::collections::HashMap;

/// Transforms library-specific patterns between ecosystems
pub struct LibraryTransformer<'a> {
    registry: &'a LibraryRegistry,
}

impl<'a> LibraryTransformer<'a> {
    pub fn new(registry: &'a LibraryRegistry) -> Self {
        Self { registry }
    }
    
    /// Transform a UIR node with library patterns to target language
    pub fn transform(
        &self,
        node: &UIRNode,
        target_lang: Language,
        target_ecosystem: Option<&str>,
    ) -> Result<UIRNode> {
        let mut transformed_node = node.clone();
        
        // Check if this node has library annotations
        if let Some(library_dep_value) = node.metadata.annotations.get("library_dependency") {
            if let serde_json::Value::String(library_dep_str) = library_dep_value {
                let library_dep: LibraryDependency = serde_json::from_str(library_dep_str)?;
                self.transform_library_node(&mut transformed_node, &library_dep, &target_lang, target_ecosystem)?;
            }
        }
        
        // Recursively transform children
        for child in &mut transformed_node.children {
            *child = self.transform(child, target_lang.clone(), target_ecosystem)?;
        }
        
        Ok(transformed_node)
    }
    
    fn transform_library_node(
        &self,
        node: &mut UIRNode,
        library_dep: &LibraryDependency,
        target_lang: &Language,
        target_ecosystem: Option<&str>,
    ) -> Result<()> {
        let default_ecosystem = self.get_default_ecosystem(target_lang);
        let target_eco = target_ecosystem.unwrap_or(&default_ecosystem);
        
        // Find the appropriate pattern for this library usage
        for usage in &library_dep.usage_patterns {
            if let Some(pattern) = self.registry.get_pattern(&library_dep.name, &usage.pattern_name) {
                if let Some(transform_rule) = pattern.transformations.get(target_eco) {
                    self.apply_transform_rule(node, &pattern, transform_rule, usage)?;
                } else {
                    // No direct transformation available, create fallback
                    self.create_fallback_implementation(node, &pattern, target_lang)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn apply_transform_rule(
        &self,
        node: &mut UIRNode,
        pattern: &LibraryPattern,
        rule: &TransformRule,
        usage: &crate::LibraryUsage,
    ) -> Result<()> {
        // Apply template transformation
        let mut transformed_code = rule.template.clone();
        
        // Replace parameter placeholders
        for (param_name, param_value) in &usage.parameters {
            let placeholder = format!("{{{{{}}}}}", param_name);
            transformed_code = transformed_code.replace(&placeholder, param_value);
        }
        
        // Update node metadata with transformation info
        node.metadata.annotations.insert(
            "transformed_from".to_string(),
            serde_json::Value::String(format!("{}:{}", pattern.library, pattern.name)),
        );
        node.metadata.annotations.insert(
            "transformed_to".to_string(),
            serde_json::Value::String(format!("{}:{}", rule.target_library, rule.target_pattern)),
        );
        node.metadata.annotations.insert(
            "generated_code".to_string(),
            serde_json::Value::String(transformed_code),
        );
        
        // Add import requirements
        if !rule.imports.is_empty() {
            node.metadata.annotations.insert(
                "required_imports".to_string(),
                serde_json::Value::String(serde_json::to_string(&rule.imports)?),
            );
        }
        
        // Add setup/cleanup code if needed
        if let Some(setup) = &rule.setup_code {
            node.metadata.annotations.insert(
                "setup_code".to_string(),
                serde_json::Value::String(setup.clone()),
            );
        }
        
        if let Some(cleanup) = &rule.cleanup_code {
            node.metadata.annotations.insert(
                "cleanup_code".to_string(),
                serde_json::Value::String(cleanup.clone()),
            );
        }
        
        Ok(())
    }
    
    fn create_fallback_implementation(
        &self,
        node: &mut UIRNode,
        pattern: &LibraryPattern,
        _target_lang: &Language,
    ) -> Result<()> {
        let fallback_comment = format!(
            "// TODO: Implement equivalent of {}:{}\n// Original behavior: {}",
            pattern.library,
            pattern.name,
            pattern.semantics.behavior
        );
        
        node.metadata.annotations.insert(
            "fallback_implementation".to_string(),
            serde_json::Value::String(fallback_comment),
        );
        
        node.metadata.annotations.insert(
            "requires_manual_implementation".to_string(),
            serde_json::Value::String("true".to_string()),
        );
        
        Ok(())
    }
    
    fn get_default_ecosystem(&self, language: &Language) -> String {
        match language {
            Language::JavaScript => "vanilla".to_string(),
            Language::Python => "stdlib".to_string(),
            Language::Rust => "std".to_string(),
            Language::Go => "stdlib".to_string(),
            Language::C => "stdlib".to_string(),
            Language::Cpp => "std".to_string(),
            Language::CSharp => "dotnet".to_string(),
            Language::FSharp => "dotnet".to_string(),
            Language::VisualBasic => "dotnet".to_string(),
            _ => "stdlib".to_string(), // Fallback for other languages
        }
    }
}
