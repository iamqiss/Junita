//! Junita compiler with real DSL parsing
//!
//! Parses .junita/.bl files and generates compilation artifacts.
//! This is a working implementation of the Junita grammar, ready to be
//! upgraded to use the full Zyntax system when Grammar2 is available.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug, warn};
use regex::Regex;

/// Compiled artifact from Junita compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledArtifact {
    /// Source file that was compiled
    pub source_file: PathBuf,
    /// Compiled widget definitions
    pub widgets: Vec<WidgetDefinition>,
    /// State machines defined in the file
    pub machines: Vec<MachineDef>,
    /// Animations defined in the file
    pub animations: Vec<AnimationDef>,
    /// Springs defined in the file
    pub springs: Vec<SpringDef>,
    /// Timestamp of compilation
    pub timestamp: u64,
    /// Checksum for detecting changes
    pub checksum: String,
}

/// Parsed widget definition from .junita file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDefinition {
    pub name: String,
    pub properties: Vec<PropDef>,
    pub state_vars: Vec<StateVar>,
    pub derived_vars: Vec<DerivedVar>,
    pub machines: Vec<String>,
    pub animations: Vec<String>,
    pub springs: Vec<String>,
    pub render_body: Option<String>,
    pub paint_body: Option<String>,
}

/// Property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDef {
    pub name: String,
    pub prop_type: String,
    pub default_value: Option<String>,
}

/// State variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVar {
    pub name: String,
    pub var_type: String,
    pub initial_value: String,
}

/// Derived value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedVar {
    pub name: String,
    pub var_type: String,
    pub expression: String,
    pub dependencies: Vec<String>,
}

/// State machine definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineDef {
    pub name: String,
    pub states: Vec<String>,
    pub initial_state: String,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub event: String,
}

/// Animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDef {
    pub name: String,
    pub duration_ms: u32,
    pub easing: String,
}

/// Spring animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpringDef {
    pub name: String,
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

/// Junita DSL Compiler with real parsing
pub struct JunitaCompiler {
    cache: HashMap<PathBuf, CompiledArtifact>,
}

impl JunitaCompiler {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Compile a .junita/.bl file with real DSL parsing
    pub async fn compile(&mut self, source_path: &Path) -> Result<CompiledArtifact> {
        debug!("Compiling {}", source_path.display());

        // Check cache
        if let Some(cached) = self.cache.get(source_path) {
            let checksum = Self::file_checksum(source_path)?;
            if cached.checksum == checksum {
                debug!("Using cached compilation for {}", source_path.display());
                return Ok(cached.clone());
            }
        }

        // Read source file
        let source = fs::read_to_string(source_path)
            .map_err(|e| anyhow!("Failed to read {}: {}", source_path.display(), e))?;

        // Validate file extension
        let ext = source_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "junita" && ext != "bl" {
            return Err(anyhow!(
                "Invalid file extension: {}. Expected .junita or .bl",
                ext
            ));
        }

        // Parse with real Junita DSL parser
        let artifact = self.parse_junita(&source, source_path)?;

        // Cache the result
        self.cache.insert(source_path.to_path_buf(), artifact.clone());

        info!("Compiled {} successfully ({} widgets)", source_path.display(), artifact.widgets.len());
        Ok(artifact)
    }

    /// Compile multiple files (incremental compilation)
    pub async fn compile_incremental(
        &mut self,
        files: &[PathBuf],
    ) -> Result<Vec<CompiledArtifact>> {
        let mut artifacts = Vec::new();
        for file in files {
            artifacts.push(self.compile(file).await?);
        }
        Ok(artifacts)
    }

    /// Real Junita DSL parser
    fn parse_junita(&self, source: &str, source_path: &Path) -> Result<CompiledArtifact> {
        let mut widgets = Vec::new();
        let mut machines = Vec::new();
        let mut animations = Vec::new();
        let mut springs = Vec::new();

        // Token-based parser for Junita DSL
        let tokens = self.tokenize(source)?;
        let mut pos = 0;

        while pos < tokens.len() {
            let token = &tokens[pos];
            
            match token.as_str() {
                "@widget" => {
                    if let Ok((widget, new_pos)) = self.parse_widget(&tokens, pos) {
                        widgets.push(widget);
                        pos = new_pos;
                    } else {
                        pos += 1;
                    }
                }
                "@machine" => {
                    if let Ok((machine, new_pos)) = self.parse_machine(&tokens, pos) {
                        machines.push(machine);
                        pos = new_pos;
                    } else {
                        pos += 1;
                    }
                }
                "@animation" => {
                    if let Ok((anim, new_pos)) = self.parse_animation(&tokens, pos) {
                        animations.push(anim);
                        pos = new_pos;
                    } else {
                        pos += 1;
                    }
                }
                "@spring" => {
                    if let Ok((spring, new_pos)) = self.parse_spring(&tokens, pos) {
                        springs.push(spring);
                        pos = new_pos;
                    } else {
                        pos += 1;
                    }
                }
                _ => {
                    pos += 1;
                }
            }
        }

        let checksum = Self::file_checksum(source_path)?;

        Ok(CompiledArtifact {
            source_file: source_path.to_path_buf(),
            widgets,
            machines,
            animations,
            springs,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            checksum,
        })
    }

    /// Tokenize Junita source
    fn tokenize(&self, source: &str) -> Result<Vec<String>> {
        // Simple tokenizer that splits on whitespace and special characters
        let re = Regex::new(r"(@\w+|[{}\[\](),=:]|\w+|[^\s])")?;
        
        let tokens: Vec<String> = re
            .find_iter(source)
            .map(|m| m.as_str().to_string())
            .filter(|t| !t.is_empty() && !t.chars().all(char::is_whitespace))
            .collect();

        Ok(tokens)
    }

    /// Parse @widget declaration
    fn parse_widget(&self, tokens: &[String], start: usize) -> Result<(WidgetDefinition, usize)> {
        if tokens[start] != "@widget" {
            return Err(anyhow!("Expected @widget"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected widget name"))?
            .clone();

        // Find opening brace
        let mut brace_pos = start + 2;
        while brace_pos < tokens.len() && tokens[brace_pos] != "{" {
            brace_pos += 1;
        }

        let mut properties = Vec::new();
        let mut state_vars = Vec::new();
        let mut derived_vars = Vec::new();
        let mut machines = Vec::new();
        let mut animations = Vec::new();
        let mut springs = Vec::new();
        let mut render_body = None;
        let mut paint_body = None;

        // Parse widget body
        let mut pos = brace_pos + 1;
        let mut depth = 1;

        while pos < tokens.len() && depth > 0 {
            match tokens[pos].as_str() {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                "@prop" => {
                    if let Ok((prop, new_pos)) = self.parse_prop(&tokens, pos) {
                        properties.push(prop);
                        pos = new_pos;
                        continue;
                    }
                }
                "@state" => {
                    if let Ok((state, new_pos)) = self.parse_state(&tokens, pos) {
                        state_vars.push(state);
                        pos = new_pos;
                        continue;
                    }
                }
                "@derived" => {
                    if let Ok((derived, new_pos)) = self.parse_derived(&tokens, pos) {
                        derived_vars.push(derived);
                        pos = new_pos;
                        continue;
                    }
                }
                "@machine" => {
                    if let Some(name) = tokens.get(pos + 1) {
                        machines.push(name.clone());
                    }
                }
                "@animation" => {
                    if let Some(name) = tokens.get(pos + 1) {
                        animations.push(name.clone());
                    }
                }
                "@spring" => {
                    if let Some(name) = tokens.get(pos + 1) {
                        springs.push(name.clone());
                    }
                }
                "@render" => {
                    // Capture render body
                    let mut body = String::new();
                    let mut inner_depth = 0;
                    let mut capturing = false;
                    
                    for i in (pos + 1)..tokens.len() {
                        if tokens[i] == "{" {
                            inner_depth += 1;
                            capturing = true;
                        } else if tokens[i] == "}" {
                            inner_depth -= 1;
                            if inner_depth == 0 && capturing {
                                render_body = Some(body.trim().to_string());
                                pos = i;
                                break;
                            }
                        }
                        if capturing {
                            body.push_str(&tokens[i]);
                            body.push(' ');
                        }
                    }
                }
                "@paint" => {
                    // Similar to render
                    let mut body = String::new();
                    let mut inner_depth = 0;
                    let mut capturing = false;
                    
                    for i in (pos + 1)..tokens.len() {
                        if tokens[i] == "{" {
                            inner_depth += 1;
                            capturing = true;
                        } else if tokens[i] == "}" {
                            inner_depth -= 1;
                            if inner_depth == 0 && capturing {
                                paint_body = Some(body.trim().to_string());
                                pos = i;
                                break;
                            }
                        }
                        if capturing {
                            body.push_str(&tokens[i]);
                            body.push(' ');
                        }
                    }
                }
                _ => {}
            }
            pos += 1;
        }

        Ok((
            WidgetDefinition {
                name,
                properties,
                state_vars,
                derived_vars,
                machines,
                animations,
                springs,
                render_body,
                paint_body,
            },
            pos + 1,
        ))
    }

    /// Parse @prop declaration
    fn parse_prop(&self, tokens: &[String], start: usize) -> Result<(PropDef, usize)> {
        if tokens[start] != "@prop" {
            return Err(anyhow!("Expected @prop"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected property name"))?
            .clone();

        // Skip colon
        let type_pos = start + 3;
        let prop_type = tokens.get(type_pos)
            .ok_or_else(|| anyhow!("Expected type"))?
            .clone();

        // Try to find default value (after =)
        let mut default_value = None;
        for i in (start + 4)..tokens.len() {
            if tokens[i] == "=" {
                if let Some(val) = tokens.get(i + 1) {
                    default_value = Some(val.clone());
                }
                return Ok((
                    PropDef {
                        name,
                        prop_type,
                        default_value,
                    },
                    i + 2,
                ));
            } else if tokens[i] == "@" || tokens[i] == "}" {
                break;
            }
        }

        Ok((
            PropDef {
                name,
                prop_type,
                default_value,
            },
            start + 4,
        ))
    }

    /// Parse @state declaration
    fn parse_state(&self, tokens: &[String], start: usize) -> Result<(StateVar, usize)> {
        if tokens[start] != "@state" {
            return Err(anyhow!("Expected @state"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected state name"))?
            .clone();

        let var_type = tokens.get(start + 3)
            .ok_or_else(|| anyhow!("Expected type"))?
            .clone();

        // Find = sign
        let mut eq_pos = start + 4;
        while eq_pos < tokens.len() && tokens[eq_pos] != "=" {
            eq_pos += 1;
        }

        let initial_value = tokens.get(eq_pos + 1)
            .ok_or_else(|| anyhow!("Expected initial value"))?
            .clone();

        Ok((
            StateVar {
                name,
                var_type,
                initial_value,
            },
            eq_pos + 2,
        ))
    }

    /// Parse @derived declaration
    fn parse_derived(&self, tokens: &[String], start: usize) -> Result<(DerivedVar, usize)> {
        if tokens[start] != "@derived" {
            return Err(anyhow!("Expected @derived"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected derived name"))?
            .clone();

        let var_type = tokens.get(start + 3)
            .ok_or_else(|| anyhow!("Expected type"))?
            .clone();

        // Find = sign and gather expression
        let mut eq_pos = start + 4;
        while eq_pos < tokens.len() && tokens[eq_pos] != "=" {
            eq_pos += 1;
        }

        let mut expr = String::new();
        let mut pos = eq_pos + 1;
        while pos < tokens.len() && tokens[pos] != "@" && tokens[pos] != "}" {
            expr.push_str(&tokens[pos]);
            expr.push(' ');
            pos += 1;
        }

        // Simple dependency extraction from expression
        let dependencies: Vec<String> = tokens[start + 1..eq_pos]
            .iter()
            .filter(|t| t.chars().next().map_or(false, |c| c.is_alphabetic()))
            .cloned()
            .collect();

        Ok((
            DerivedVar {
                name,
                var_type,
                expression: expr.trim().to_string(),
                dependencies,
            },
            pos,
        ))
    }

    /// Parse @machine declaration
    fn parse_machine(&self, tokens: &[String], start: usize) -> Result<(MachineDef, usize)> {
        if tokens[start] != "@machine" {
            return Err(anyhow!("Expected @machine"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected machine name"))?
            .clone();

        // Find opening brace
        let mut brace_pos = start + 2;
        while brace_pos < tokens.len() && tokens[brace_pos] != "{" {
            brace_pos += 1;
        }

        let mut states = Vec::new();
        let mut transitions = Vec::new();
        let mut initial_state = String::new();

        // Simple state machine parser
        let mut pos = brace_pos + 1;
        while pos < tokens.len() && tokens[pos] != "}" {
            if tokens[pos].chars().all(|c| c.is_alphabetic() || c == '_') {
                states.push(tokens[pos].clone());
            }
            pos += 1;
        }

        if !states.is_empty() {
            initial_state = states[0].clone();
        }

        Ok((
            MachineDef {
                name,
                states,
                initial_state,
                transitions,
            },
            pos + 1,
        ))
    }

    /// Parse @animation declaration
    fn parse_animation(&self, tokens: &[String], start: usize) -> Result<(AnimationDef, usize)> {
        if tokens[start] != "@animation" {
            return Err(anyhow!("Expected @animation"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected animation name"))?
            .clone();

        let mut duration_ms = 300u32;
        let mut easing = "ease-out".to_string();

        // Find values in the body
        for i in (start + 2)..tokens.len() {
            if tokens[i].contains("duration") {
                if let Some(val_str) = tokens.get(i + 1) {
                    if let Ok(val) = val_str.replace("ms", "").replace("s", "00").parse::<u32>() {
                        duration_ms = val;
                    }
                }
            }
            if tokens[i].contains("easing") || tokens[i].contains("ease") {
                if let Some(val) = tokens.get(i + 1) {
                    easing = val.trim_matches(|c| c == '"' || c == '\'').to_string();
                }
            }
            if tokens[i] == "}" {
                break;
            }
        }

        // Find closing brace
        let mut pos = start + 2;
        let mut depth = 0;
        while pos < tokens.len() {
            if tokens[pos] == "{" {
                depth += 1;
            } else if tokens[pos] == "}" {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            pos += 1;
        }

        Ok((
            AnimationDef {
                name,
                duration_ms,
                easing,
            },
            pos + 1,
        ))
    }

    /// Parse @spring declaration
    fn parse_spring(&self, tokens: &[String], start: usize) -> Result<(SpringDef, usize)> {
        if tokens[start] != "@spring" {
            return Err(anyhow!("Expected @spring"));
        }

        let name = tokens.get(start + 1)
            .ok_or_else(|| anyhow!("Expected spring name"))?
            .clone();

        let mut stiffness = 100.0f32;
        let mut damping = 10.0f32;
        let mut mass = 1.0f32;

        // Extract spring parameters
        for i in (start + 2)..tokens.len() {
            if tokens[i] == "}" {
                break;
            }
            if tokens[i].contains("stiffness") {
                if let Some(val_str) = tokens.get(i + 1) {
                    if let Ok(val) = val_str.parse::<f32>() {
                        stiffness = val;
                    }
                }
            }
            if tokens[i].contains("damping") {
                if let Some(val_str) = tokens.get(i + 1) {
                    if let Ok(val) = val_str.parse::<f32>() {
                        damping = val;
                    }
                }
            }
            if tokens[i].contains("mass") {
                if let Some(val_str) = tokens.get(i + 1) {
                    if let Ok(val) = val_str.parse::<f32>() {
                        mass = val;
                    }
                }
            }
        }

        let mut pos = start + 2;
        let mut depth = 0;
        while pos < tokens.len() {
            if tokens[pos] == "{" {
                depth += 1;
            } else if tokens[pos] == "}" {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            pos += 1;
        }

        Ok((
            SpringDef {
                name,
                stiffness,
                damping,
                mass,
            },
            pos + 1,
        ))
    }

    fn file_checksum(path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let source = fs::read_to_string(path)?;
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Clear compilation cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cached artifact
    pub fn get_cached(&self, path: &Path) -> Option<&CompiledArtifact> {
        self.cache.get(path)
    }
}

impl Default for JunitaCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let compiler = JunitaCompiler::new();
        let source = "@widget Counter { @state count: Int = 0 }";
        let tokens = compiler.tokenize(source).unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], "@widget");
    }

    #[tokio::test]
    async fn test_compile_demo() {
        let mut compiler = JunitaCompiler::new();
        let demo_path = Path::new("examples/hot_reload_demo/main.junita");
        
        if demo_path.exists() {
            let result = compiler.compile(demo_path).await;
            assert!(result.is_ok());
            let artifact = result.unwrap();
            assert!(!artifact.widgets.is_empty(), "Should parse widgets from demo file");
        }
    }
}

