use std::fs;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture(std::path::PathBuf);

impl Fixture {
    fn new(contents: &[u8]) -> Self {
        let path = std::env::temp_dir().join(format!(
            "github-actions-test-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        let fixture = Self(path);
        fs::write(fixture.0.join("input.txt"), contents).unwrap();
        fixture
    }

    fn search(&self, ignore_case: bool) -> Output {
        let mut command = command();
        command.arg("rust").arg(self.0.join("input.txt"));
        if ignore_case {
            command.env("IGNORE_CASE", "1");
        }
        command.output().unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_githubActions"));
    command.env_remove("IGNORE_CASE");
    command
}

#[test]
fn cli_selects_case_sensitive_or_insensitive_search() {
    let fixture = Fixture::new(b"Rust\nrust\nRUST\nother\n");
    for (ignore_case, expected) in [(false, vec!["rust"]), (true, vec!["Rust", "rust", "RUST"])] {
        let output = fixture.search(ignore_case);
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.lines().skip(2).collect::<Vec<_>>(), expected);
    }
}

#[test]
fn cli_reports_missing_arguments() {
    for (args, expected) in [
        (vec![], "Didn't get a query string"),
        (vec!["rust"], "Didn't get a file path"),
    ] {
        let output = command().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8(output.stderr).unwrap().contains(expected));
    }
}

#[test]
fn cli_reports_missing_file_and_invalid_utf8() {
    let fixture = Fixture::new(&[0xff]);
    for path in [fixture.0.join("missing.txt"), fixture.0.join("input.txt")] {
        let output = command().arg("rust").arg(path).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("Application error:")
        );
    }
}

#[test]
fn cli_succeeds_when_nothing_matches() {
    let fixture = Fixture::new(b"no matching lines\n");
    let output = fixture.search(false);
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().lines().count(), 2);
}
