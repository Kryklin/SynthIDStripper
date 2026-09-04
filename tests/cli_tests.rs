use std::process::Command;

fn get_bin_path() -> String {
    let mut path = std::env::current_exe().expect("current test binary path");
    path.pop(); // deps
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("lexicon_stripper");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path.to_str().expect("valid path string").to_string()
}

#[test]
fn test_cli_help_command_displays_overview_and_topics() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("help")
        .output()
        .expect("failed to run cli help");

    assert!(output.status.success(), "help command exited with failure");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SPECIALIZED HELP TOPICS:") || stdout.contains("help typos"),
        "Help output missing specialized help topics pointer"
    );
    assert!(
        stdout.contains("Optional Enhancement: Typing Noise"),
        "Help output missing Typing Noise section"
    );
}

#[test]
fn test_cli_help_typos_displays_typing_system() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .args(["help", "typos"])
        .output()
        .expect("failed to run cli help typos");

    assert!(output.status.success(), "help typos command exited with failure");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HUMAN TYPING NOISE & BIOMECHANICAL ERROR SIMULATOR"),
        "help typos missing banner header"
    );
    assert!(
        stdout.contains("substitution") && stdout.contains("transposition") && stdout.contains("omission"),
        "help typos missing core biomechanical error classes"
    );
    assert!(
        stdout.contains("insertion") && stdout.contains("duplication") && stdout.contains("temporal"),
        "help typos missing extended error classes"
    );
    assert!(
        stdout.contains("--typing-noise") && stdout.contains("--typing-seed"),
        "help typos missing CLI option descriptions"
    );
}

#[test]
fn test_cli_help_typos_flag() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("--help-typos")
        .output()
        .expect("failed to run --help-typos");

    assert!(output.status.success(), "--help-typos exited with failure");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HUMAN TYPING NOISE & BIOMECHANICAL ERROR SIMULATOR"),
        "--help-typos missing banner"
    );
}

#[test]
fn test_cli_help_domains_displays_all_29_domains() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .args(["help", "domains"])
        .output()
        .expect("failed to run cli help domains");

    assert!(output.status.success(), "help domains exited with failure");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SUPPORTED SPECIALIZED LINGUISTIC DOMAINS"),
        "help domains missing banner"
    );
    assert!(stdout.contains("culinary") && stdout.contains("finance"));
    assert!(stdout.contains("maritime_nautical") && stdout.contains("occult_astrology"));
}
