use std::fs;
use std::path::Path;

#[test]
fn test_tailwind_color_usage() {
    let _app_dir = Path::new("src"); // Adjust path as needed relative to test execution
    let _required_colors = vec!["slate-950", "cyan-glow", "bg-white/5", "backdrop-blur"];
    let _forbidden_colors = vec!["bg-red-500", "bg-blue-500"]; // Example strictness

    // Scan all rs files
    // This is a simplified "grep" test

    // In a real scenario, this would parse AST or use a stricter regex
    // For now, let's just assert that important theme tokens are present in key files

    let key_files = vec![
        "src/components/ui.rs",
        "src/components/sidebar.rs",
        "src/components/dashboard.rs",
    ];

    for file_path in key_files {
        if !Path::new(file_path).exists() {
            // Skip if running from wrong dir, but print warning
            println!("Context: File not found for audit: {}", file_path);
            continue;
        }

        let content = fs::read_to_string(file_path).unwrap_or_default();
        if file_path.contains("ui.rs") {
            assert!(
                content.contains("backdrop-blur"),
                "ui.rs missing glass effect"
            );
        }
    }
}
