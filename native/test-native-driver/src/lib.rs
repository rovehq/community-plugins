use sdk::{CoreContext, CoreTool, EngineError, ToolInput, ToolOutput};

#[derive(Default)]
pub struct GeneratedSystemTool {
    ctx: Option<CoreContext>,
}

impl CoreTool for GeneratedSystemTool {
    fn name(&self) -> &str {
        "run"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn start(&mut self, ctx: CoreContext) -> Result<(), EngineError> {
        self.ctx = Some(ctx);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), EngineError> {
        self.ctx = None;
        Ok(())
    }

    fn handle(&self, input: ToolInput) -> Result<ToolOutput, EngineError> {
        Ok(ToolOutput::json(serde_json::json!({
            "summary": format!("Replace this scaffold with real driver logic for method '{}'.", input.method),
            "plugin_type": "Plugin",
            "params": input.params,
        })))
    }
}

#[no_mangle]
pub fn create_tool() -> *mut dyn CoreTool {
    Box::into_raw(Box::new(GeneratedSystemTool::default()))
}
