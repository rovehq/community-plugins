use test_native_driver::GeneratedSystemTool;
use sdk::{CoreTool, ToolInput};

#[test]
fn generated_system_tool_reports_method() {
    let tool = GeneratedSystemTool::default();
    let input = ToolInput::new("run").with_param("input", serde_json::json!("hello"));
    let output = tool.handle(input).expect("tool output");

    assert!(output.success);
    assert_eq!(output.data["plugin_type"], serde_json::json!("Plugin"));
}
