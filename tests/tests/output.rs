macro_rules! stdout_for_format {
    ($($format:expr)?) => {{
        let out = rust_script!(
            "tests/data/script-no-deps.rs",
            "--gen-pkg-only",
            $(concat!("--output-format=", $format))?
        )
        .unwrap();
        assert!(out.success());
        out.stdout
    }};
}

#[test]
fn test_gen_pkg_output_format_none() {
    let output_none = stdout_for_format!("none");
    assert_eq!(output_none, "");
}

#[test]
fn test_gen_pkg_output_format_path() {
    use std::path::Path;
    const DIGEST_LEN: usize = 24;

    let output_path = stdout_for_format!("path");
    let output_default = stdout_for_format!();
    assert_eq!(output_default, output_path);

    let path = Path::new(output_path.trim());
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
