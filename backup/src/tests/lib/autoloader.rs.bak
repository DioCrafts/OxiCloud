/**
 * Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use pretty_assertions::assert_eq;

/// AutoLoader test module
#[cfg(test)]
mod test_auto_loader {
    use super::*;
    use crate::autoloader::AutoLoader;

    struct AutoLoaderTest {
        loader: AutoLoader,
    }

    impl AutoLoaderTest {
        fn new() -> Self {
            Self {
                loader: AutoLoader::new(),
            }
        }
    }

    #[test]
    fn test_leading_slash_on_class_name() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["private/files/storage/local.php".to_string(), "files/storage/local.php".to_string()],
            test.loader.find_class(r"\OC\Files\Storage\Local")
        );
    }

    #[test]
    fn test_no_leading_slash_on_class_name() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["private/files/storage/local.php".to_string(), "files/storage/local.php".to_string()],
            test.loader.find_class(r"OC\Files\Storage\Local")
        );
    }

    #[test]
    fn test_legacy_path() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["private/legacy/files.php".to_string(), "private/files.php".to_string()],
            test.loader.find_class("OC_Files")
        );
    }

    #[test]
    fn test_class_path() {
        let mut test = AutoLoaderTest::new();
        test.loader.register_class("Foo\\Bar", "foobar.php");
        assert_eq!(
            vec!["foobar.php".to_string()],
            test.loader.find_class("Foo\\Bar")
        );
    }

    #[test]
    fn test_prefix_namespace() {
        let mut test = AutoLoaderTest::new();
        test.loader.register_prefix("Foo", "foo");
        assert_eq!(
            vec!["foo/Foo/Bar.php".to_string()],
            test.loader.find_class("Foo\\Bar")
        );
    }

    #[test]
    fn test_prefix() {
        let mut test = AutoLoaderTest::new();
        test.loader.register_prefix("Foo_", "foo");
        assert_eq!(
            vec!["foo/Foo/Bar.php".to_string()],
            test.loader.find_class("Foo_Bar")
        );
    }

    #[test]
    fn test_load_test_namespace() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["tests/lib/foo/bar.php".to_string()],
            test.loader.find_class("Test\\Foo\\Bar")
        );
    }

    #[test]
    fn test_load_test() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["tests/lib/foo/bar.php".to_string()],
            test.loader.find_class("Test_Foo_Bar")
        );
    }

    #[test]
    fn test_load_core_namespace() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["private/foo/bar.php".to_string(), "foo/bar.php".to_string()],
            test.loader.find_class("OC\\Foo\\Bar")
        );
    }

    #[test]
    fn test_load_core() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["private/legacy/foo/bar.php".to_string(), "private/foo/bar.php".to_string()],
            test.loader.find_class("OC_Foo_Bar")
        );
    }

    #[test]
    fn test_load_public_namespace() {
        let test = AutoLoaderTest::new();
        assert_eq!(
            vec!["public/foo/bar.php".to_string()],
            test.loader.find_class("OCP\\Foo\\Bar")
        );
    }

    #[test]
    fn test_load_app_namespace() {
        let test = AutoLoaderTest::new();
        let result = test.loader.find_class("OCA\\Files\\Foobar");
        assert_eq!(2, result.len());
        assert!(result[0].ends_with("apps/files/foobar.php"));
        assert!(result[1].ends_with("apps/files/lib/foobar.php"));
    }
}

// Implementación de AutoLoader que se usa en las pruebas
#[allow(dead_code)]
pub mod autoloader {
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub struct AutoLoader {
        class_map: HashMap<String, String>,
        prefix_map: HashMap<String, String>,
    }

    impl AutoLoader {
        pub fn new() -> Self {
            Self {
                class_map: HashMap::new(),
                prefix_map: HashMap::new(),
            }
        }

        pub fn register_class(&mut self, class_name: &str, path: &str) {
            self.class_map.insert(class_name.to_string(), path.to_string());
        }

        pub fn register_prefix(&mut self, prefix: &str, path: &str) {
            self.prefix_map.insert(prefix.to_string(), path.to_string());
        }

        pub fn find_class(&self, class_name: &str) -> Vec<String> {
            // Eliminar la barra inicial si existe
            let class_name = class_name.trim_start_matches('\\');

            // Verificar clase registrada explícitamente
            if let Some(path) = self.class_map.get(class_name) {
                return vec![path.clone()];
            }

            // Verificar prefijos registrados
            for (prefix, path) in &self.prefix_map {
                if class_name.starts_with(prefix) {
                    let suffix = class_name.trim_start_matches(prefix);
                    let suffix = if prefix.ends_with('_') {
                        suffix.replace('_', "/")
                    } else {
                        suffix.replace('\\', "/")
                    };
                    
                    return vec![format!("{}/{}{}.php", path, prefix.trim_end_matches('_'), suffix)];
                }
            }

            // Manejo de namespaces específicos
            if class_name.starts_with("Test\\") || class_name.starts_with("Test_") {
                let path = if class_name.starts_with("Test\\") {
                    class_name.replace("Test\\", "tests/lib/").replace('\\', "/").to_lowercase()
                } else {
                    class_name.replace("Test_", "tests/lib/").replace('_', "/").to_lowercase()
                };
                return vec![format!("{}.php", path)];
            }

            if class_name.starts_with("OC\\") {
                let path = class_name.replace("OC\\", "").replace('\\', "/").to_lowercase();
                return vec![
                    format!("private/{}.php", path),
                    format!("{}.php", path),
                ];
            }

            if class_name.starts_with("OC_") {
                let path = class_name.replace("OC_", "").replace('_', "/").to_lowercase();
                return vec![
                    format!("private/legacy/{}.php", path),
                    format!("private/{}.php", path),
                ];
            }

            if class_name.starts_with("OCP\\") {
                let path = class_name.replace("OCP\\", "").replace('\\', "/").to_lowercase();
                return vec![format!("public/{}.php", path)];
            }

            if class_name.starts_with("OCA\\") {
                let parts: Vec<&str> = class_name.splitn(3, '\\').collect();
                if parts.len() < 3 {
                    return vec![];
                }
                
                let app = parts[1].to_lowercase();
                let path = parts[2].replace('\\', "/").to_lowercase();
                
                return vec![
                    format!("apps/{}/{}.php", app, path),
                    format!("apps/{}/lib/{}.php", app, path),
                ];
            }

            vec![]
        }
    }
}