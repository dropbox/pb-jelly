use std::env;
use std::fs;

use pretty_assertions::assert_eq;
use walkdir::WalkDir;

/// Ensure that generated files match .expected
#[test]
fn verify_generated_files() {
    let proto_files: Vec<_> = WalkDir::new("gen/pb-jelly")
        .into_iter()
        .filter_map(|e| match e {
            Err(_) => None,
            Ok(entry) => {
                let ext = entry.path().extension().map(|ext| ext.to_str().unwrap());
                match ext {
                    Some("rs") | Some("toml") => Some(entry.into_path()),
                    _ => None,
                }
            },
        })
        .collect();

    // Assert the correct number of pb-test generated files
    // Developers - please change this number if the change is intentional
    assert_eq!(proto_files.len(), 16);

    // Assert contents of the generated files
    for proto_file in proto_files {
        let filename = proto_file.file_name().unwrap().to_owned().into_string().unwrap() + ".expected";
        let expected = proto_file.parent().unwrap().join(filename);

        if env::var("VALIDATE").is_err() {
            // In CI - don't copy over to `.expected` so we can do a validation
            fs::copy(&proto_file, &expected).unwrap();
        }

        let contents = fs::read_to_string(proto_file).unwrap();
        let expected_contents = fs::read_to_string(&expected).unwrap();
        assert_eq!(
            contents.lines().collect::<Vec<_>>(),
            expected_contents.lines().collect::<Vec<_>>(),
            ".expected files don't match - Please run `cd pb-test ; cargo test` to generate"
        );
    }
}
