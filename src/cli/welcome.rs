/// Welcome message handling separated into its own module.
pub fn show() {
    // Print a friendly, informative welcome when user runs `nine-cli` with no args
    println!("");
    println!("=========================================");
    println!("nine-cli — quick help \u{2728}");
    println!("");
    println!("Supported commands:");
    println!("  - nine-cli <skill-name> [args...]\n      Run an installed skill by name. Example: nine-cli hello --help");
    println!("  - nine-cli skill add <path>\n      Install a local skill folder. Example: nine-cli skill add ./my-skill");
    println!("  - nine-cli skill list\n      List installed skills");
    println!("  - nine-cli skill remove <name>\n      Remove an installed skill. Example: nine-cli skill remove hello");
    println!("  - nine-cli skill verify <path>\n      Verify a local skill folder without installing (outputs JSON)");
    println!("");
    println!("Try the example skill:");
    println!("  1) Install the shipped example: scripts/install_sample_skill.sh");
    println!("  2) Run it: nine-cli hello");
    println!("");
    println!("For full documentation and developer guides, check the repository: https://github.com/nine-ai-2046-1/nine-cli");
    println!("=========================================");
    println!("");
}
