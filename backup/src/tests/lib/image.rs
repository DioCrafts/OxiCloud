use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use base64::{encode, decode};
use std::str;

struct TestContext {
    server_root: PathBuf,
}

impl TestContext {
    fn new() -> Self {
        let server_root = env::current_dir().unwrap();
        TestContext { server_root }
    }

    fn get_test_file_path(&self, file_name: &str) -> PathBuf {
        self.server_root.join("tests").join("data").join(file_name)
    }
}

struct OcImage {
    image: Option<DynamicImage>,
}

impl OcImage {
    fn new(source: Option<&str>) -> Self {
        let image = match source {
            None => None,
            Some(src) => {
                // Try as file path
                if Path::new(src).exists() {
                    match image::open(src) {
                        Ok(img) => Some(img),
                        Err(_) => None,
                    }
                } 
                // Try as raw data
                else if let Ok(data) = decode(src) {
                    match image::load_from_memory(&data) {
                        Ok(img) => Some(img),
                        Err(_) => None,
                    }
                } 
                // Try as direct binary content
                else {
                    match image::load_from_memory(src.as_bytes()) {
                        Ok(img) => Some(img),
                        Err(_) => None,
                    }
                }
            }
        };

        OcImage { image }
    }

    fn get_mime_type_for_file(path: Option<&str>) -> String {
        if let Some(path) = path {
            if let Ok(img) = image::open(path) {
                return match img {
                    DynamicImage::ImageLuma8(_) |
                    DynamicImage::ImageRgb8(_) |
                    DynamicImage::ImageLuma16(_) |
                    DynamicImage::ImageRgb16(_) => String::from("image/png"),
                    
                    DynamicImage::ImageRgba8(_) |
                    DynamicImage::ImageRgba16(_) => {
                        if path.ends_with(".png") {
                            String::from("image/png")
                        } else if path.ends_with(".gif") {
                            String::from("image/gif")
                        } else {
                            String::from("image/png")
                        }
                    },
                    
                    DynamicImage::ImageBgr8(_) |
                    DynamicImage::ImageBgra8(_) => {
                        if path.ends_with(".jpg") || path.ends_with(".jpeg") {
                            String::from("image/jpeg")
                        } else {
                            String::from("image/png")
                        }
                    },
                    _ => String::from(""),
                };
            }
        }
        String::from("")
    }

    fn valid(&self) -> bool {
        self.image.is_some()
    }

    fn mime_type(&self) -> String {
        if let Some(img) = &self.image {
            match img {
                DynamicImage::ImageLuma8(_) |
                DynamicImage::ImageRgb8(_) |
                DynamicImage::ImageLuma16(_) |
                DynamicImage::ImageRgb16(_) => String::from("image/png"),
                
                DynamicImage::ImageRgba8(_) |
                DynamicImage::ImageRgba16(_) => String::from("image/png"),
                
                DynamicImage::ImageBgr8(_) |
                DynamicImage::ImageBgra8(_) => String::from("image/jpeg"),
                
                _ => String::from(""),
            }
        } else {
            String::from("")
        }
    }

    fn width(&self) -> i32 {
        if let Some(img) = &self.image {
            img.width() as i32
        } else {
            -1
        }
    }

    fn height(&self) -> i32 {
        if let Some(img) = &self.image {
            img.height() as i32
        } else {
            -1
        }
    }

    fn save(&self, path: &str) -> bool {
        if let Some(img) = &self.image {
            let format = if path.ends_with(".png") {
                ImageFormat::Png
            } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
                ImageFormat::Jpeg
            } else if path.ends_with(".gif") {
                ImageFormat::Gif
            } else {
                ImageFormat::Png
            };

            return img.save_with_format(path, format).is_ok();
        }
        false
    }

    fn data(&self) -> Vec<u8> {
        if let Some(img) = &self.image {
            let mut bytes: Vec<u8> = Vec::new();
            let format = match self.mime_type().as_str() {
                "image/jpeg" => ImageOutputFormat::Jpeg(100),
                "image/gif" => ImageOutputFormat::Gif,
                _ => ImageOutputFormat::Png,
            };
            
            if let Ok(()) = img.write_to(&mut bytes, format) {
                return bytes;
            }
        }
        Vec::new()
    }

    fn resize(&mut self, size: u32) -> bool {
        if let Some(img) = &mut self.image {
            *img = img.resize(size, size, image::imageops::FilterType::Lanczos3);
            true
        } else {
            false
        }
    }

    fn precise_resize(&mut self, width: u32, height: u32) -> bool {
        if let Some(img) = &mut self.image {
            *img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
            true
        } else {
            false
        }
    }

    fn center_crop(&mut self, size: Option<u32>) -> bool {
        if let Some(img) = &mut self.image {
            let (width, height) = img.dimensions();
            let size = size.unwrap_or_else(|| width.min(height));
            
            let x = (width as i32 - size as i32) / 2;
            let y = (height as i32 - size as i32) / 2;
            
            if x >= 0 && y >= 0 {
                *img = img.crop(x as u32, y as u32, size, size);
                return true;
            }
        }
        false
    }

    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> bool {
        if let Some(img) = &mut self.image {
            *img = img.crop(x, y, width, height);
            true
        } else {
            false
        }
    }

    fn fit_in(&mut self, max_width: u32, max_height: u32) -> bool {
        if let Some(img) = &mut self.image {
            let (width, height) = img.dimensions();
            
            let width_ratio = max_width as f32 / width as f32;
            let height_ratio = max_height as f32 / height as f32;
            
            let ratio = width_ratio.min(height_ratio);
            let new_width = (width as f32 * ratio) as u32;
            let new_height = (height as f32 * ratio) as u32;
            
            *img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
            true
        } else {
            false
        }
    }
}

impl ToString for OcImage {
    fn to_string(&self) -> String {
        encode(&self.data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn tear_down_after_class() {
        let context = TestContext::new();
        let _ = fs::remove_file(context.get_test_file_path("testimage2.png"));
        let _ = fs::remove_file(context.get_test_file_path("testimage2.jpg"));
    }

    #[test]
    fn test_get_mime_type_for_file() {
        let context = TestContext::new();
        
        let mime_type = OcImage::get_mime_type_for_file(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert_eq!("image/png", mime_type);
        
        let mime_type = OcImage::get_mime_type_for_file(Some(context.get_test_file_path("testimage.jpg").to_str().unwrap()));
        assert_eq!("image/jpeg", mime_type);
        
        let mime_type = OcImage::get_mime_type_for_file(Some(context.get_test_file_path("testimage.gif").to_str().unwrap()));
        assert_eq!("image/gif", mime_type);
        
        let mime_type = OcImage::get_mime_type_for_file(None);
        assert_eq!("", mime_type);
    }

    #[test]
    fn test_construct_destruct() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.valid());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert!(img.valid());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let img = OcImage::new(Some(&base64));
        assert!(img.valid());
        
        let img = OcImage::new(None);
        assert!(!img.valid());
    }

    #[test]
    fn test_valid() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.valid());
        
        let text = encode("Lorem ipsum dolor sir amet …");
        let img = OcImage::new(Some(&text));
        assert!(!img.valid());
        
        let img = OcImage::new(None);
        assert!(!img.valid());
    }

    #[test]
    fn test_mime_type() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert_eq!("image/png", img.mime_type());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert_eq!("image/jpeg", img.mime_type());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let img = OcImage::new(Some(&base64));
        assert_eq!("image/png", img.mime_type());
        
        let img = OcImage::new(None);
        assert_eq!("", img.mime_type());
    }

    #[test]
    fn test_width() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert_eq!(128, img.width());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert_eq!(1680, img.width());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let img = OcImage::new(Some(&base64));
        assert_eq!(64, img.width());
        
        let img = OcImage::new(None);
        assert_eq!(-1, img.width());
    }

    #[test]
    fn test_height() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert_eq!(128, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert_eq!(1050, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let img = OcImage::new(Some(&base64));
        assert_eq!(64, img.height());
        
        let img = OcImage::new(None);
        assert_eq!(-1, img.height());
    }

    #[test]
    fn test_save() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        img.resize(16);
        let out_path = context.get_test_file_path("testimage2.png").to_str().unwrap();
        img.save(out_path);
        let saved_data = fs::read(out_path).unwrap();
        assert_eq!(saved_data, img.data());
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.jpg").to_str().unwrap()));
        img.resize(128);
        let out_path = context.get_test_file_path("testimage2.jpg").to_str().unwrap();
        img.save(out_path);
        let saved_data = fs::read(out_path).unwrap();
        assert_eq!(saved_data, img.data());
        
        tear_down_after_class();
    }

    #[test]
    fn test_data() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(!img.data().is_empty());
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.jpg").to_str().unwrap()));
        assert!(!img.data().is_empty());
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.gif").to_str().unwrap()));
        assert!(!img.data().is_empty());
    }

    #[test]
    fn test_to_string() {
        let context = TestContext::new();
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        let expected = encode(&img.data());
        assert_eq!(expected, img.to_string());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        let expected = encode(&img.data());
        assert_eq!(expected, img.to_string());
        
        let img = OcImage::new(Some(context.get_test_file_path("testimage.gif").to_str().unwrap()));
        let expected = encode(&img.data());
        assert_eq!(expected, img.to_string());
    }

    #[test]
    fn test_resize() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.resize(32));
        assert_eq!(32, img.width());
        assert_eq!(32, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert!(img.resize(840));
        assert_eq!(840, img.width());
        assert_eq!(525, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let mut img = OcImage::new(Some(&base64));
        assert!(img.resize(100));
        assert_eq!(100, img.width());
        assert_eq!(100, img.height());
    }

    #[test]
    fn test_precise_resize() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.precise_resize(128, 512));
        assert_eq!(128, img.width());
        assert_eq!(512, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert!(img.precise_resize(64, 840));
        assert_eq!(64, img.width());
        assert_eq!(840, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let mut img = OcImage::new(Some(&base64));
        assert!(img.precise_resize(1000, 1337));
        assert_eq!(1000, img.width());
        assert_eq!(1337, img.height());
    }

    #[test]
    fn test_center_crop() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        img.center_crop(None);
        assert_eq!(128, img.width());
        assert_eq!(128, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        img.center_crop(None);
        assert_eq!(1050, img.width());
        assert_eq!(1050, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let mut img = OcImage::new(Some(&base64));
        img.center_crop(Some(512));
        assert_eq!(512, img.width());
        assert_eq!(512, img.height());
    }

    #[test]
    fn test_crop() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.crop(0, 0, 50, 20));
        assert_eq!(50, img.width());
        assert_eq!(20, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert!(img.crop(500, 700, 550, 300));
        assert_eq!(550, img.width());
        assert_eq!(300, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let mut img = OcImage::new(Some(&base64));
        assert!(img.crop(10, 10, 15, 15));
        assert_eq!(15, img.width());
        assert_eq!(15, img.height());
    }

    #[test]
    fn test_fit_in() {
        let context = TestContext::new();
        
        let mut img = OcImage::new(Some(context.get_test_file_path("testimage.png").to_str().unwrap()));
        assert!(img.fit_in(200, 100));
        assert_eq!(100, img.width());
        assert_eq!(100, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.jpg")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut img = OcImage::new(Some(str::from_utf8(&buffer).unwrap()));
        assert!(img.fit_in(840, 840));
        assert_eq!(840, img.width());
        assert_eq!(525, img.height());
        
        let mut file = fs::File::open(context.get_test_file_path("testimage.gif")).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let base64 = encode(&buffer);
        let mut img = OcImage::new(Some(&base64));
        assert!(img.fit_in(200, 250));
        assert_eq!(200, img.width());
        assert_eq!(200, img.height());
    }
}