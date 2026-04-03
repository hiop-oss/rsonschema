#[cfg(test)]
mod tests {

    use rsonschema::validate;
    use serde_json::Value;
    use std::{fs, path};

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
        let suite_root = path::Path::new(".")
            .join("tests")
            .join("JSON-Schema-Test-Suite")
            .join("tests");
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
                        let report = validate(instance, schema.clone());
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
        if counter != 42 {
            panic!("Total errors: {counter}");
        }
    }
}
