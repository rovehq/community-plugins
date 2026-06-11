use test_wasm_plugin::{run_impl, RunInput};

#[test]
fn run_impl_reports_input_and_files() {
    let output = run_impl(RunInput {
        input: "summarise this".to_string(),
        file: Some("sample.pdf".to_string()),
        files: vec!["notes.txt".to_string()],
        context: serde_json::Value::Null,
    });

    assert!(output.summary.contains("summarise this"));
    assert_eq!(output.files_seen.len(), 2);
    assert!(output.files_seen.contains(&"sample.pdf".to_string()));
}
