use std::path::Path;

#[test]
fn test_list_actions_output() {
    let workflow = r#"
        on: push
        jobs:
          test:
            runs-on: ubuntu-latest
            steps:
              - uses: actions/checkout@v4
              - uses: actions/setup-node@v3
        "#;

    let temp_dir = tempfile::tempdir().unwrap();
    let workflow_path = temp_dir.path().join(".github/workflows/test.yml");
    std::fs::create_dir_all(workflow_path.parent().unwrap()).unwrap();
    std::fs::write(&workflow_path, workflow).unwrap();

    let output = Zizmor::new()
        .input(workflow_path.to_str().unwrap())
        .format("actions-list")
        .output(OutputMode::Stdout)
        .run()
        .unwrap();

    assert!(output.stdout.contains("actions/checkout@v4"));
    assert!(output.stdout.contains("actions/setup-node@v3"));
}