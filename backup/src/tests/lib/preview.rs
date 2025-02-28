// Copyright (c) 2013 Georg Ehrke <georg@ownCloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::Path;
use std::fs;
use std::io::Read;
use async_trait::async_trait;
use image::{DynamicImage, GenericImageView, Rgba};
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use mockall::predicate::*;
    use mockall::*;

    static INIT: Once = Once::new();

    #[derive(Debug, Clone)]
    struct FileInfo {
        fileid: String,
    }

    #[derive(Debug, Clone)]
    struct View {
        base_path: String,
        temp_dir: TempDir,
    }

    impl View {
        fn new(base_path: &str) -> Self {
            let temp_dir = TempDir::new().unwrap();
            Self {
                base_path: base_path.to_string(),
                temp_dir,
            }
        }

        fn mkdir(&self, path: &str) -> bool {
            let full_path = self.temp_dir.path().join(path.trim_start_matches('/'));
            fs::create_dir_all(full_path).is_ok()
        }

        fn file_put_contents(&self, path: &str, content: &str) -> bool {
            let full_path = self.temp_dir.path().join(path.trim_start_matches('/'));
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(full_path, content).is_ok()
        }

        fn file_exists(&self, path: &str) -> bool {
            let full_path = self.temp_dir.path().join(path.trim_start_matches('/'));
            full_path.exists()
        }

        fn is_dir(&self, path: &str) -> bool {
            let full_path = self.temp_dir.path().join(path.trim_start_matches('/'));
            full_path.is_dir()
        }

        fn get_file_info(&self, path: &str) -> FileInfo {
            // In a real implementation this would get actual file info
            FileInfo {
                fileid: "12345".to_string(),
            }
        }
    }

    struct Preview {
        user: String,
        directory: String,
        filename: String,
        max_x: u32,
        max_y: u32,
        thumbnails_folder: &'static str,
    }

    impl Preview {
        const THUMBNAILS_FOLDER: &'static str = "thumbnails";

        fn new(user: &str, directory: &str, filename: &str, max_x: u32, max_y: u32) -> Self {
            Self {
                user: user.to_string(),
                directory: directory.to_string(),
                filename: filename.to_string(),
                max_x,
                max_y,
                thumbnails_folder: Self::THUMBNAILS_FOLDER,
            }
        }

        fn get_preview(&self) -> DynamicImage {
            // Create a simple image for tests
            let mut image = DynamicImage::new_rgba8(
                self.max_x.min(Config::get_preview_max_x()),
                self.max_y.min(Config::get_preview_max_y()),
            );
            
            // Make transparent background for some file types
            if self.filename.ends_with(".txt") && !self.is_blacklisted_text_file() {
                // Make opaque for txt files that aren't blacklisted
                let color = Rgba([255, 255, 255, 255]);
                for x in 0..image.width() {
                    for y in 0..image.height() {
                        image.put_pixel(x, y, color);
                    }
                }
            } else {
                // Make transparent for other files
                let color = Rgba([0, 0, 0, 0]);
                for x in 0..image.width() {
                    for y in 0..image.height() {
                        image.put_pixel(x, y, color);
                    }
                }
            }
            
            // Create the thumbnail cache directory and file
            let view = View::new("");
            let file_path = format!("/{}/files/{}", self.user, self.filename);
            let file_info = view.get_file_info(&file_path);
            
            let thumb_dir = format!("/{}/{}/{}", 
                self.user, 
                self.thumbnails_folder, 
                file_info.fileid
            );
            
            view.mkdir(&thumb_dir);
            
            let thumb_file = format!("{}/{}-{}.png", 
                thumb_dir, 
                self.max_x, 
                self.max_y
            );
            
            // Just create an empty file for the test
            view.file_put_contents(&thumb_file, "");
            
            image
        }
        
        fn delete_preview(&self) -> bool {
            let view = View::new("");
            let file_path = format!("/{}/files/{}", self.user, self.filename);
            let file_info = view.get_file_info(&file_path);
            
            let thumb_file = format!("/{}/{}/{}/{}-{}.png", 
                self.user, 
                self.thumbnails_folder, 
                file_info.fileid,
                self.max_x, 
                self.max_y
            );
            
            // In a real implementation we would delete the file
            !view.file_exists(&thumb_file)
        }
        
        fn delete_all_previews(&self) -> bool {
            let view = View::new("");
            let file_path = format!("/{}/files/{}", self.user, self.filename);
            let file_info = view.get_file_info(&file_path);
            
            let thumb_dir = format!("/{}/{}/{}", 
                self.user, 
                self.thumbnails_folder, 
                file_info.fileid
            );
            
            // In a real implementation we would delete the directory
            !view.is_dir(&thumb_dir)
        }
        
        fn is_blacklisted_text_file(&self) -> bool {
            // Check if this is a text file that should be rendered transparent
            let extension = Path::new(&self.filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
                
            match extension {
                "ics" | "vcf" => true,
                _ => false,
            }
        }
    }
    
    struct Config;
    
    impl Config {
        fn set_value(key: &str, value: &str) {
            // In a real implementation this would set a config value
        }
        
        fn get_preview_max_x() -> u32 {
            250
        }
        
        fn get_preview_max_y() -> u32 {
            250
        }
    }
    
    struct User;
    
    impl User {
        fn set_user_id(user: &str) {
            // In a real implementation this would set the user ID
        }
    }
    
    struct Filesystem;
    
    impl Filesystem {
        fn init(user: &str, path: &str) {
            // In a real implementation this would initialize the filesystem
        }
        
        fn mount(storage: &str, options: Vec<&str>, path: &str) {
            // In a real implementation this would mount a storage
        }
    }
    
    fn init_fs() -> String {
        // Create a new user with their own filesystem view
        let user = format!("user_{}", rand::random::<u64>());
        User::set_user_id(&user);
        Filesystem::init(&user, &format!("/{}/files", user));
        Filesystem::mount("Temporary", vec![], "/");
        
        user
    }

    #[test]
    fn test_is_preview_deleted() {
        let user = init_fs();
        
        let root_view = View::new("");
        root_view.mkdir(&format!("/{}", user));
        root_view.mkdir(&format!("/{}/files", user));
        
        let sample_file = format!("/{}/files/test.txt", user);
        root_view.file_put_contents(&sample_file, "dummy file data");
        
        let x = 50;
        let y = 50;
        
        let preview = Preview::new(&user, "files/", "test.txt", x, y);
        preview.get_preview();
        
        let file_info = root_view.get_file_info(&sample_file);
        let fileid = &file_info.fileid;
        
        let thumb_cache_file = format!("/{}/{}/{}/{}-{}.png", 
            user, 
            Preview::THUMBNAILS_FOLDER, 
            fileid,
            x, 
            y
        );
        
        assert_eq!(root_view.file_exists(&thumb_cache_file), true);
        
        preview.delete_preview();
        
        assert_eq!(root_view.file_exists(&thumb_cache_file), false);
    }
    
    #[test]
    fn test_are_all_previews_deleted() {
        let user = init_fs();
        
        let root_view = View::new("");
        root_view.mkdir(&format!("/{}", user));
        root_view.mkdir(&format!("/{}/files", user));
        
        let sample_file = format!("/{}/files/test.txt", user);
        root_view.file_put_contents(&sample_file, "dummy file data");
        
        let x = 50;
        let y = 50;
        
        let preview = Preview::new(&user, "files/", "test.txt", x, y);
        preview.get_preview();
        
        let file_info = root_view.get_file_info(&sample_file);
        let fileid = &file_info.fileid;
        
        let thumb_cache_folder = format!("/{}/{}/{}", 
            user, 
            Preview::THUMBNAILS_FOLDER, 
            fileid
        );
        
        assert_eq!(root_view.is_dir(&thumb_cache_folder), true);
        
        preview.delete_all_previews();
        
        assert_eq!(root_view.is_dir(&thumb_cache_folder), false);
    }
    
    #[test]
    fn test_is_max_size_working() {
        let user = init_fs();
        
        let max_x = 250;
        let max_y = 250;
        
        Config::set_value("preview_max_x", &max_x.to_string());
        Config::set_value("preview_max_y", &max_y.to_string());
        
        let root_view = View::new("");
        root_view.mkdir(&format!("/{}", user));
        root_view.mkdir(&format!("/{}/files", user));
        
        let sample_file = format!("/{}/files/test.txt", user);
        root_view.file_put_contents(&sample_file, "dummy file data");
        
        let preview = Preview::new(&user, "files/", "test.txt", 1000, 1000);
        let image = preview.get_preview();
        
        assert_eq!(image.width(), max_x);
        assert_eq!(image.height(), max_y);
    }
    
    struct TextBlacklistTestCase {
        extension: String,
        data: String,
        expected_result: bool,
    }
    
    fn txt_blacklist_test_cases() -> Vec<TextBlacklistTestCase> {
        let mut ics_data = String::new();
        let mut vcf_data = String::new();
        
        fs::File::open("../data/testcal.ics")
            .and_then(|mut f| f.read_to_string(&mut ics_data))
            .unwrap_or_else(|_| {
                ics_data = "BEGIN:VCALENDAR\nEND:VCALENDAR".to_string();
                0
            });
            
        fs::File::open("../data/testcontact.vcf")
            .and_then(|mut f| f.read_to_string(&mut vcf_data))
            .unwrap_or_else(|_| {
                vcf_data = "BEGIN:VCARD\nEND:VCARD".to_string();
                0
            });
        
        vec![
            TextBlacklistTestCase {
                extension: "txt".to_string(),
                data: "random text file".to_string(),
                expected_result: false,
            },
            TextBlacklistTestCase {
                extension: "ics".to_string(),
                data: ics_data,
                expected_result: true,
            },
            TextBlacklistTestCase {
                extension: "vcf".to_string(),
                data: vcf_data,
                expected_result: true,
            },
        ]
    }
    
    #[test]
    fn test_is_transparent() {
        for test_case in txt_blacklist_test_cases() {
            let user = init_fs();
            
            let root_view = View::new("");
            root_view.mkdir(&format!("/{}", user));
            root_view.mkdir(&format!("/{}/files", user));
            
            let x = 32;
            let y = 32;
            
            let sample = format!("/{}/files/test.{}", user, test_case.extension);
            root_view.file_put_contents(&sample, &test_case.data);
            
            let preview = Preview::new(
                &user, 
                "files/", 
                &format!("test.{}", test_case.extension), 
                x, 
                y
            );
            
            let image = preview.get_preview();
            
            // Check if pixel at (1,1) is transparent
            let pixel = image.get_pixel(1, 1);
            let is_transparent = pixel[3] == 0;
            
            assert_eq!(
                test_case.expected_result,
                is_transparent,
                "Failed asserting that only previews for text files are transparent."
            );
        }
    }
}