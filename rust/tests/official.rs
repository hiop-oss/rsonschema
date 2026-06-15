#[cfg(test)]
mod tests {

    use rsonschema::validate_with_resolver;
    use serde_json::Value;
    use std::{fs, path};

    /// Base URI the JSON-Schema-Test-Suite uses for its `remotes/` folder.
    const REMOTES_BASE_URI: &str = "http://localhost:1234/";

    /// Resolve remote `$ref`s offline by mapping the suite's `localhost:1234`
    /// base URI onto the on-disk `remotes/` folder instead of doing a real HTTP
    /// request (which would need a server running during the test).
    fn resolve_remote(ref_: &str, remotes_root: &path::Path) -> Option<Value> {
        let relative = ref_.strip_prefix(REMOTES_BASE_URI)?;
        let file = remotes_root.join(relative);
        let reader = fs::File::open(file).ok()?;
        serde_json::from_reader(reader).ok()
    }

    fn get_file_paths(folder: &path::Path) -> Vec<path::PathBuf> {
        let mut test_paths = Vec::new();
        let paths = fs::read_dir(folder).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                if !path.to_string_lossy().ends_with("skip") {
                    let sub_test_paths = get_file_paths(&path);
                    test_paths.extend(sub_test_paths)
                }
            } else {
                test_paths.push(path);
            }
        }
        test_paths
    }

    #[test]
    fn test_official() {
        let suite_dir = path::Path::new(".")
            .join("tests")
            .join("JSON-Schema-Test-Suite");
        let remotes_root = suite_dir.join("remotes");
        let suite_root = suite_dir.join("tests");
        if !suite_root.exists() {
            eprintln!(
                "Skipping official tests: JSON-Schema-Test-Suite not found at {}",
                suite_root.display()
            );
            return;
        }
        let mut counter = 0;
        let drafts = ["draft2020-12"];
        let test_paths = drafts.iter().flat_map(|draft| {
            let folder = suite_root.join(draft);
            get_file_paths(&folder)
        });
        for test_path in test_paths {
            let reader = fs::File::open(&test_path).unwrap();
            let content: Value = serde_json::from_reader(&reader).unwrap();
            for item in content.as_array().unwrap() {
                let schema = item.get("schema").unwrap();
                let tests = item.get("tests").unwrap();
                for test in tests.as_array().unwrap() {
                    let description = test.get("description").unwrap().as_str().unwrap();
                    if !description.ends_with("is only an annotation by default") {
                        let instance = test.get("data").unwrap();
                        let is_valid = test.get("valid").unwrap().as_bool().unwrap();
                        let resolver = |ref_: &str| resolve_remote(ref_, &remotes_root);
                        let report =
                            validate_with_resolver(instance, schema.clone(), None, Some(&resolver));
                        if is_valid != report.is_valid() {
                            eprintln!(
                                "filename: {} ... \x1b[31mfailed\x1b[0m",
                                test_path.display()
                            );
                            eprintln!("description:  {description}");
                            eprintln!("schema:       {schema}");
                            eprintln!("instance:     {instance}");
                            eprintln!("errors:       {:?}", report.errors);
                            eprintln!();
                            counter += 1;
                        }
                    }
                }
            }
        }
        // Known-unsupported cases for the pinned JSON-Schema-Test-Suite version:
        // mostly `$dynamicRef`, IDN `hostname`/`duration` optional formats, and a
        // few remote-ref edge cases. Bump this when adding/removing support.
        const KNOWN_FAILURES: usize = 62;
        if counter != KNOWN_FAILURES {
            panic!("Expected {KNOWN_FAILURES} known failures, got {counter}");
        }
    }
}
