// Copyright (C) Thomas Müller
// This file is licensed under the GNU AFFERO GENERAL PUBLIC LICENSE Version 3
// See the LICENSE file for details

// Test module for the Files Mapper functionality.

use std::path::PathBuf;

// Simulación de la biblioteca PHPUnit en Rust
#[cfg(test)]
mod tests {
    use super::*;
    
    /// Rust implementation of the OC\Files\Mapper class.
    pub struct Mapper {
        root_path: PathBuf,
    }
    
    impl Mapper {
        /// Create a new mapper with the specified root path.
        pub fn new<P: Into<PathBuf>>(root_path: P) -> Self {
            Mapper {
                root_path: root_path.into(),
            }
        }
        
        /// Slugify a path, optionally adding a numeric suffix.
        pub fn slugify_path<P: AsRef<str>>(&self, path: P, suffix: Option<u32>) -> String {
            let path_str = path.as_ref();
            let path_obj = PathBuf::from(path_str);
            
            // Get the parent directory and filename
            let parent = path_obj.parent().map_or("", |p| p.to_str().unwrap_or(""));
            let filename = path_obj.file_name().map_or("", |f| f.to_str().unwrap_or(""));
            
            // Handle the case where there's no filename
            if filename.is_empty() {
                return path_str.to_string();
            }
            
            // Split the filename into name and extension
            let mut parts: Vec<&str> = filename.rsplitn(2, '.').collect();
            parts.reverse();
            
            let (name, extension) = if parts.len() > 1 {
                (parts[0], Some(parts[1]))
            } else {
                (parts[0], None)
            };
            
            // Replace spaces with hyphens in the name part
            let slugified_name = name.replace(' ', "-");
            
            // Apply the suffix if provided
            let final_name = if let Some(num) = suffix {
                format!("{}-{}", slugified_name, num)
            } else {
                slugified_name
            };
            
            // Reconstruct the path
            let mut result = String::new();
            if !parent.is_empty() {
                result.push_str(parent);
                result.push_str("/");
            }
            
            result.push_str(&final_name);
            
            if let Some(ext) = extension {
                result.push_str(".");
                // Also replace spaces in extension
                result.push_str(&ext.replace(' ', "-"));
            }
            
            result
        }
    }
    
    struct MapperTest {
        mapper: Mapper,
    }
    
    impl MapperTest {
        fn new() -> Self {
            Self {
                mapper: Mapper::new("D:/"),
            }
        }
        
        fn setup(&mut self) {
            self.mapper = Mapper::new("D:/");
        }
        
        fn test_slugify_path(&self) {
            // with extension
            assert_eq!("D:/text.txt", self.mapper.slugify_path("D:/text.txt", None));
            assert_eq!("D:/text-2.txt", self.mapper.slugify_path("D:/text.txt", Some(2)));
            assert_eq!("D:/a/b/text.txt", self.mapper.slugify_path("D:/a/b/text.txt", None));
            
            // without extension
            assert_eq!("D:/text", self.mapper.slugify_path("D:/text", None));
            assert_eq!("D:/text-2", self.mapper.slugify_path("D:/text", Some(2)));
            assert_eq!("D:/a/b/text", self.mapper.slugify_path("D:/a/b/text", None));
            
            // with double dot
            assert_eq!("D:/text.text.txt", self.mapper.slugify_path("D:/text.text.txt", None));
            assert_eq!("D:/text.text-2.txt", self.mapper.slugify_path("D:/text.text.txt", Some(2)));
            assert_eq!("D:/a/b/text.text.txt", self.mapper.slugify_path("D:/a/b/text.text.txt", None));
            
            // foldername and filename with periods
            assert_eq!("D:/folder.name.with.periods", self.mapper.slugify_path("D:/folder.name.with.periods", None));
            assert_eq!("D:/folder.name.with.periods/test-2.txt", self.mapper.slugify_path("D:/folder.name.with.periods/test.txt", Some(2)));
            assert_eq!("D:/folder.name.with.periods/test.txt", self.mapper.slugify_path("D:/folder.name.with.periods/test.txt", None));
            
            // foldername and filename with periods and spaces
            assert_eq!("D:/folder.name.with.peri-ods", self.mapper.slugify_path("D:/folder.name.with.peri ods", None));
            assert_eq!("D:/folder.name.with.peri-ods/te-st-2.t-x-t", self.mapper.slugify_path("D:/folder.name.with.peri ods/te st.t x t", Some(2)));
            assert_eq!("D:/folder.name.with.peri-ods/te-st.t-x-t", self.mapper.slugify_path("D:/folder.name.with.peri ods/te st.t x t", None));
        }
    }
    
    #[test]
    fn test_mapper_functionality() {
        let mut test = MapperTest::new();
        test.setup();
        test.test_slugify_path();
    }
}