macro_rules! stdout_for_format {
    ($script:expr $(,$format:expr)?) => {{
        let out = rust_script!(
            concat!("tests/data/", $script),
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
    let output_none = stdout_for_format!("script-no-deps.rs", "none");
    assert_eq!(output_none, "");
}

fn assert_package_path(path: &std::path::Path) {
    const DIGEST_LEN: usize = 24;
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

#[test]
fn test_gen_pkg_output_format_path() {
    use std::path::Path;

    let output_path = stdout_for_format!("script-no-deps.rs", "path");
    let output_default = stdout_for_format!("script-no-deps.rs");
    assert_eq!(output_default, output_path);

    let path = Path::new(output_path.trim());
    assert!(path.exists());
    assert!(path.is_absolute());
    assert_package_path(path);
}

#[test]
fn test_gen_pkg_output_format_metadata_json() {
    use serde::Deserialize;
    use std::ops::Range;
    #[derive(Deserialize)]
    struct OutputMetadataJson {
        package_path: String,
        script_path: String,
        manifest_span: Option<CodeSpan>,
    }
    #[derive(Deserialize, Debug, Eq, PartialEq)]
    pub struct CodeSpan {
        byte_span: Range<usize>,
        lsp_span: lsp_types::Range,
    }

    fn assert_metadata_json(stdout: String, script_name: &str, manifest_span: Option<CodeSpan>) {
        use std::path::Path;

        let output_data: OutputMetadataJson = serde_json::from_str(&stdout).unwrap();

        assert_package_path(Path::new(&output_data.package_path));

        let script_path = Path::new(&output_data.script_path);
        let current_dir = std::env::current_dir().unwrap();
        let script_path = script_path
            .strip_prefix(current_dir)
            .expect("expected working directory to be ancestor of script");
        let script_path_segments: Vec<Option<&str>> = script_path
            .components()
            .map(|c| c.as_os_str().to_str())
            .collect();
        assert!(matches!(
            &script_path_segments[..],
            [Some("tests"), Some("data"), Some(s_name)] if s_name == &script_name
        ));

        assert_eq!(output_data.manifest_span, manifest_span);
    }

    let output_no_manifest = stdout_for_format!("script-no-deps.rs", "metadata_json");
    assert_metadata_json(output_no_manifest, "script-no-deps.rs", None);

    let output_block_manifest = stdout_for_format!("script-full-block.rs", "metadata_json");
    assert_metadata_json(
        output_block_manifest,
        "script-full-block.rs",
        Some(CodeSpan {
            byte_span: Range {
                start: 91,
                end: 156,
            },
            lsp_span: lsp_types::Range {
                start: lsp_types::Position::new(2, 0),
                end: lsp_types::Position::new(5, 7),
            },
        }),
    );

    let output_line_manifest = stdout_for_format!("script-full-line.rs", "metadata_json");
    assert_metadata_json(
        output_line_manifest,
        "script-full-line.rs",
        Some(CodeSpan {
            byte_span: Range {
                start: 88,
                end: 137,
            },
            lsp_span: lsp_types::Range {
                start: lsp_types::Position::new(3, 0),
                end: lsp_types::Position::new(6, 3),
            },
        }),
    );

    let output_short = stdout_for_format!("script-short.rs", "metadata_json");
    assert_metadata_json(
        output_short,
        "script-short.rs",
        Some(CodeSpan {
            byte_span: Range { start: 0, end: 35 },
            lsp_span: lsp_types::Range {
                start: lsp_types::Position::new(0, 0),
                end: lsp_types::Position::new(1, 0),
            },
        }),
    );
}
