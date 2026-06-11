use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use extism_pdk::*;

#[derive(Debug, Deserialize, Default)]
pub struct RunInput {
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub context: Value,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct RunOutput {
    pub summary: String,
    pub files_seen: Vec<String>,
    pub plugin_type: String,
}

pub fn run_impl(input: RunInput) -> RunOutput {
    let mut files = input.files;
    if let Some(file) = input.file {
        if !files.iter().any(|existing| existing == &file) {
            files.push(file);
        }
    }

    let action = if input.input.trim().is_empty() {
        "Replace this scaffold with your real plugin logic.".to_string()
    } else {
        format!("Received task: {}", input.input.trim())
    };

    RunOutput {
        summary: action,
        files_seen: files,
        plugin_type: "Plugin".to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
#[plugin_fn]
pub fn run(Json(input): Json<RunInput>) -> FnResult<Json<RunOutput>> {
    Ok(Json(run_impl(input)))
}
