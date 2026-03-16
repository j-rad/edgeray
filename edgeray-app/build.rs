fn main() {
    // Only run if the tailwind config or input css changes
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=assets/input.css");
    println!("cargo:rerun-if-changed=src");

    // Check if we are in a proper environment to run tailwind
    // We try to run via npx or pnpm exec
    use std::process::Command;

    let status = Command::new("pnpm")
        .args([
            "tailwindcss",
            "-i",
            "./assets/input.css",
            "-o",
            "./assets/styles.css",
            "--minify",
        ])
        .status();

    if let Ok(exit_status) = status {
        if !exit_status.success() {
            println!("cargo:warning=Failed to run tailwindcss via npx. CSS might be outdated.");
        }
    } else {
        println!("cargo:warning=Could not execute npx to build tailwindcss.");
    }
}
