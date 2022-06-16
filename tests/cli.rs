use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_does_not_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("this.does/not.exit");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error opening this.does/not.exit"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("grrs.test.txt")?;
    file.write_str("The answer is\n42.\n\nSo long, so long and thank you for all the fish!")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("42").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2: 42."));
    Ok(())
}
