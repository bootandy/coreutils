use common::util::*;
use std::env;

static SUB_DIR: &str = "subdir/deeper";
static SUB_DIR_LINKS: &str = "subdir/links";
static SUB_FILE: &str = "subdir/links/subwords.txt";
static SUB_LINK: &str = "subdir/links/sublink.txt";

#[test]
fn test_du_basics() {
    let (_at, mut ucmd) = at_and_ucmd!();
    let result = ucmd.run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
}
#[cfg(target_os = "macos")]
fn _du_basics(s: String) {
    let answer = "32\t./subdir
8\t./subdir/deeper
24\t./subdir/links
40\t./
";
    assert_eq!(s, answer);
}
#[cfg(not(target_os = "macos"))]
fn _du_basics(s: String) {
    let answer = "28\t./subdir
8\t./subdir/deeper
16\t./subdir/links
36\t./
";
    assert_eq!(s, answer);
}

#[test]
fn test_du_basics_subdir() {
    let (_at, mut ucmd) = at_and_ucmd!();

    let result = ucmd.arg(SUB_DIR).run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
    _du_basics_subdir(result.stdout);
}

#[cfg(target_os = "macos")]
fn _du_basics_subdir(s: String) {
    assert_eq!(s, "8\tsubdir/deeper\n");
}
#[cfg(not(target_os = "macos"))]
fn _du_basics_subdir(s: String) {
    assert_eq!(s, "8\tsubdir/deeper\n");
}

#[test]
fn test_du_basics_bad_name() {
    let (_at, mut ucmd) = at_and_ucmd!();

    let result = ucmd.arg("bad_name").run();
    assert_eq!(result.stdout, "");
    assert_eq!(result.stderr, "du: bad_name: No such file or directory\n");
}

#[test]
fn test_du_soft_link() {
    let ts = TestScenario::new("du");

    let link = ts.cmd("ln").arg("-s").arg(SUB_FILE).arg(SUB_LINK).run();
    assert!(link.success);

    let result = ts.ucmd().arg(SUB_DIR_LINKS).run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
    _du_soft_link(result.stdout);
}

#[cfg(target_os = "macos")]
fn _du_soft_link(s: String) {
    assert_eq!(s, "32\tsubdir/links\n");
}
#[cfg(not(target_os = "macos"))]
fn _du_soft_link(s: String) {
    assert_eq!(s, "16\tsubdir/links\n");
}

#[test]
fn test_du_hard_link() {
    let ts = TestScenario::new("du");

    let link = ts.cmd("ln").arg(SUB_FILE).arg(SUB_LINK).run();
    assert!(link.success);

    let result = ts.ucmd().arg(SUB_DIR_LINKS).run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
    // We do not double count hard links as the inodes are identicle
    _du_hard_link(result.stdout);
}

#[cfg(target_os = "macos")]
fn _du_hard_link(s: String) {
    assert_eq!(s, "24\tsubdir/links\n")
}
#[cfg(not(target_os = "macos"))]
fn _du_hard_link(s: String) {
    assert_eq!(s, "16\tsubdir/links\n");
}

#[test]
fn test_du_d_flag() {
    let ts = TestScenario::new("du");

    let result = ts.ucmd().arg("-d").arg("1").run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
    _du_d_flag(result.stdout);
}

#[cfg(target_os = "macos")]
fn _du_d_flag(s: String) {
    assert_eq!(s, "32\t./subdir\n40\t./\n");
}
#[cfg(not(target_os = "macos"))]
fn _du_d_flag(s: String) {
    assert_eq!(s, "28\t./subdir\n36\t./\n");
}

#[test]
fn test_du_blocksize_env_flag() {
    let last = env::var("BLOCKSIZE");
    env::set_var("BLOCKSIZE", "1000000");
    let ts = TestScenario::new("du");

    let result = ts.ucmd_keepenv().run();
    assert!(result.success);
    assert_eq!(result.stderr, "");
    assert_eq!(
        result.stdout,
        "1\t./subdir
1\t./subdir/deeper
1\t./subdir/links
1\t./
"
    );
    if let Ok(s) = last {
        env::set_var("SHELL", s);
    }
}

#[test]
fn test_du_bad_blocksize_env_flag_doesnt_crash() {
    let last = env::var("BLOCKSIZE");
    env::set_var("BLOCKSIZE", "i dont want to be cast to an int");
    let ts = TestScenario::new("du");

    let result = ts.ucmd_keepenv().run();
    assert!(result.success);
    if let Ok(s) = last {
        env::set_var("SHELL", s);
    }
}
