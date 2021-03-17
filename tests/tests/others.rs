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
