use crate::{LibraryDependency, LibraryUsage};
use coalesce_core::{Language, Result, CoalesceError};
use regex::Regex;
use std::collections::HashMap;

/// Detects library dependencies and usage patterns in source code
pub struct DependencyDetector {
    patterns: HashMap<Language, Vec<DetectionPattern>>,
}

#[derive(Debug, Clone)]
struct DetectionPattern {
    library_name: String,
    import_regex: Regex,
    usage_patterns: Vec<UsagePattern>,
    ecosystem: String,
}

#[derive(Debug, Clone)]
struct UsagePattern {
    name: String,
    regex: Regex,
    semantic_intent: String,
    extract_params: Vec<String>,
}

impl DependencyDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            patterns: HashMap::new(),
        };
        detector.register_default_patterns();
        detector
    }
    
    pub fn detect_dependencies(&self, code: &str, language: Language) -> Result<Vec<LibraryDependency>> {
        let patterns = self.patterns.get(&language)
            .ok_or_else(|| CoalesceError::UnsupportedLanguage(language))?;
        
        let mut dependencies = Vec::new();
        
        for pattern in patterns {
            if let Some(dep) = self.detect_library_usage(code, pattern)? {
                dependencies.push(dep);
            }
        }
        
        Ok(dependencies)
    }
    
    fn detect_library_usage(&self, code: &str, pattern: &DetectionPattern) -> Result<Option<LibraryDependency>> {
        // Check if the library is imported
        if !pattern.import_regex.is_match(code) {
            return Ok(None);
        }
        
        let mut usage_patterns = Vec::new();
        
        // Look for usage patterns
        for usage in &pattern.usage_patterns {
            for capture in usage.regex.captures_iter(code) {
                let mut parameters = HashMap::new();
                
                // Extract parameters based on named groups
                for param_name in &usage.extract_params {
                    if let Some(value) = capture.name(param_name) {
                        parameters.insert(param_name.clone(), value.as_str().to_string());
                    }
                }
                
                usage_patterns.push(LibraryUsage {
                    pattern_name: usage.name.clone(),
                    method_name: capture.get(0).unwrap().as_str().to_string(),
                    parameters,
                    semantic_intent: usage.semantic_intent.clone(),
                    source_location: (
                        capture.get(0).unwrap().start(),
                        capture.get(0).unwrap().end(),
                    ),
                });
            }
        }
        
        if usage_patterns.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(LibraryDependency {
            name: pattern.library_name.clone(),
            version: None, // TODO: Extract version from imports
            ecosystem: pattern.ecosystem.clone(),
            import_path: None, // TODO: Extract import path
            usage_patterns,
        }))
    }
    
    fn register_default_patterns(&mut self) {
        self.register_react_patterns();
        self.register_django_patterns();
        self.register_networking_patterns();
    }
    
    fn register_react_patterns(&mut self) {
        let patterns = vec![
            DetectionPattern {
                library_name: "react".to_string(),
                import_regex: Regex::new(r#"import.*\{[^}]*useState[^}]*\}.*from.*['"]react['"]"#).unwrap(),
                ecosystem: "javascript".to_string(),
                usage_patterns: vec![
                    UsagePattern {
                        name: "useState".to_string(),
                        regex: Regex::new(r"const\s*\[\s*(?P<state>\w+)\s*,\s*(?P<setter>\w+)\s*\]\s*=\s*useState\s*\(\s*(?P<initial>[^)]*)\s*\)").unwrap(),
                        semantic_intent: "reactive_state_management".to_string(),
                        extract_params: vec!["state".to_string(), "setter".to_string(), "initial".to_string()],
                    },
                ],
            },
            DetectionPattern {
                library_name: "react".to_string(),
                import_regex: Regex::new(r#"import.*\{[^}]*useEffect[^}]*\}.*from.*['"]react['"]"#).unwrap(),
                ecosystem: "javascript".to_string(),
                usage_patterns: vec![
                    UsagePattern {
                        name: "useEffect".to_string(),
                        regex: Regex::new(r"useEffect\s*\(\s*(?P<callback>[^,]+)\s*,\s*(?P<deps>\[[^\]]*\])\s*\)").unwrap(),
                        semantic_intent: "side_effect_lifecycle".to_string(),
                        extract_params: vec!["callback".to_string(), "deps".to_string()],
                    },
                ],
            },
        ];
        
        self.patterns.insert(Language::JavaScript, patterns);
    }
    
    fn register_django_patterns(&mut self) {
        let patterns = vec![
            DetectionPattern {
                library_name: "django".to_string(),
                import_regex: Regex::new(r"from\s+django\.db\s+import\s+models").unwrap(),
                ecosystem: "python".to_string(),
                usage_patterns: vec![
                    UsagePattern {
                        name: "Model".to_string(),
                        regex: Regex::new(r"class\s+(?P<name>\w+)\s*\(\s*models\.Model\s*\)").unwrap(),
                        semantic_intent: "orm_model".to_string(),
                        extract_params: vec!["name".to_string()],
                    },
                    UsagePattern {
                        name: "CharField".to_string(),
                        regex: Regex::new(r"(?P<field>\w+)\s*=\s*models\.CharField\s*\(\s*max_length\s*=\s*(?P<length>\d+)").unwrap(),
                        semantic_intent: "text_field".to_string(),
                        extract_params: vec!["field".to_string(), "length".to_string()],
                    },
                ],
            },
        ];
        
        self.patterns.insert(Language::Python, patterns);
    }
    
    fn register_networking_patterns(&mut self) {
        // C networking patterns
        let c_patterns = vec![
            DetectionPattern {
                library_name: "socket".to_string(),
                import_regex: Regex::new(r"#include\s+<sys/socket\.h>").unwrap(),
                ecosystem: "c".to_string(),
                usage_patterns: vec![
                    UsagePattern {
                        name: "socket".to_string(),
                        regex: Regex::new(r"(?P<var>\w+)\s*=\s*socket\s*\(\s*(?P<family>AF_\w+)\s*,\s*(?P<type>SOCK_\w+)\s*,\s*(?P<protocol>\d+)\s*\)").unwrap(),
                        semantic_intent: "tcp_socket_creation".to_string(),
                        extract_params: vec!["var".to_string(), "family".to_string(), "type".to_string()],
                    },
                ],
            },
        ];
        
        self.patterns.insert(Language::C, c_patterns);
    }
}
