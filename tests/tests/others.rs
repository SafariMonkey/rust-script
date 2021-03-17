#[test]
fn test_version() {
    let out = rust_script!("--version").unwrap();
    assert!(out.success());
    scan!(&out.stdout;
        ("rust-script", &::std::env::var("CARGO_PKG_VERSION").unwrap(), .._) => ()
    )
    .unwrap();
}

#[test]
fn test_clear_cache() {
    let out = rust_script!("--clear-cache").unwrap();
    assert!(out.success());
}

#[test]
fn test_gen_pkg() {
    let out = rust_script!("tests/data/script-no-deps.rs", "--gen-pkg-only").unwrap();
    assert!(out.success());
}

#[test]
fn test_print_pkg_path() {
    use std::path::Path;
    const DIGEST_LEN: usize = 24;

    let out = rust_script!(
        "tests/data/script-no-deps.rs",
        "--gen-pkg-only",
        "--print-pkg-path"
    )
    .unwrap();
    assert!(out.success());

    let path = Path::new(out.stdout.trim());
    assert!(path.exists());
    assert!(path.is_absolute());

    let path_segments: Vec<Option<&str>> =
        path.components().map(|c| c.as_os_str().to_str()).collect();
    let digest = match &path_segments[..] {
        &[.., Some("rust-script"), Some("projects"), Some(digest)] => (digest),
        _ => panic!("path end segments {:?} were unexpected", path_segments),
    };

    assert_eq!(digest.len(), DIGEST_LEN, "digest: {}", digest);
    let has_non_hex_chars = digest.chars().any(|c| !matches!(c, '0'..='9' | 'a'..='f'));
    assert!(!has_non_hex_chars, "non-hex digest? {:?}", digest);
}
