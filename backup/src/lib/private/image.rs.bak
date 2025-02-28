use std::fs;
use std::io::{self, Read, Write, Seek, BufReader, Cursor};
use std::path::{Path, PathBuf};
use std::convert::TryFrom;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat, imageops};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use exif::{In, Reader as ExifReader, Tag};
use mime_guess::from_path;
use log::{debug, error, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    
    #[error("Exif error: {0}")]
    Exif(#[from] exif::Error),
    
    #[error("Invalid image data")]
    InvalidData,
    
    #[error("Unsupported image format")]
    UnsupportedFormat,
    
    #[error("No image loaded")]
    NoImageLoaded,
    
    #[error("Cannot write to directory: {0}")]
    DirectoryNotWritable(String),
    
    #[error("File not writable: {0}")]
    FileNotWritable(String),
    
    #[error("GD module not installed")]
    GdNotInstalled,
}

pub type Result<T> = std::result::Result<T, ImageError>;

/// Estructura para manipulación básica de imágenes
pub struct Image {
    image: Option<DynamicImage>,
    image_type: ImageFormat,
    mime_type: String,
    bit_depth: u8,
    file_path: Option<PathBuf>,
}

impl Image {
    /// Obtiene el tipo MIME para un archivo de imagen
    pub fn get_mime_type_for_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let path = file_path.as_ref();
        
        // Verificar que el archivo existe y es suficientemente grande
        let metadata = fs::metadata(path)?;
        if metadata.len() <= 11 {
            return Ok(String::new());
        }
        
        // Usar mime_guess para determinar el tipo MIME
        let mime = from_path(path).first_or_octet_stream().to_string();
        
        // Verificar que sea una imagen
        if !mime.starts_with("image/") {
            return Ok(String::new());
        }
        
        Ok(mime)
    }
    
    /// Constructor
    pub fn new() -> Self {
        Self {
            image: None,
            image_type: ImageFormat::Png,
            mime_type: "image/png".to_string(),
            bit_depth: 24,
            file_path: None,
        }
    }
    
    /// Crea una nueva instancia de Image y carga una imagen
    pub fn with_data<T: AsRef<[u8]>>(data: T) -> Result<Self> {
        let mut image = Self::new();
        image.load_from_data(data)?;
        Ok(image)
    }
    
    /// Crea una nueva instancia de Image y carga una imagen desde un archivo
    pub fn with_file<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let mut image = Self::new();
        image.load_from_file(file_path)?;
        Ok(image)
    }
    
    /// Crea una nueva instancia de Image y carga una imagen desde un string base64
    pub fn with_base64(data: &str) -> Result<Self> {
        let mut image = Self::new();
        image.load_from_base64(data)?;
        Ok(image)
    }
    
    /// Determina si la instancia contiene un recurso de imagen válido
    pub fn valid(&self) -> bool {
        self.image.is_some()
    }
    
    /// Devuelve el tipo MIME de la imagen o un string vacío si no hay imagen cargada
    pub fn mime_type(&self) -> String {
        if self.valid() {
            self.mime_type.clone()
        } else {
            String::new()
        }
    }
    
    /// Devuelve el ancho de la imagen o -1 si no hay imagen cargada
    pub fn width(&self) -> i32 {
        match &self.image {
            Some(img) => img.width() as i32,
            None => -1,
        }
    }
    
    /// Devuelve el alto de la imagen o -1 si no hay imagen cargada
    pub fn height(&self) -> i32 {
        match &self.image {
            Some(img) => img.height() as i32,
            None => -1,
        }
    }
    
    /// Devuelve el ancho cuando la orientación de la imagen es top-left
    pub fn width_top_left(&self) -> i32 {
        let orientation = self.get_orientation();
        debug!("Image::width_top_left() Orientation: {}", orientation);
        
        match orientation {
            -1 | 1 | 2 | 3 | 4 => self.width(),
            5 | 6 | 7 | 8 => self.height(),
            _ => self.width(),
        }
    }
    
    /// Devuelve el alto cuando la orientación de la imagen es top-left
    pub fn height_top_left(&self) -> i32 {
        let orientation = self.get_orientation();
        debug!("Image::height_top_left() Orientation: {}", orientation);
        
        match orientation {
            -1 | 1 | 2 | 3 | 4 => self.height(),
            5 | 6 | 7 | 8 => self.width(),
            _ => self.height(),
        }
    }
    
    /// Muestra la imagen enviando los headers correctos
    pub fn show(&self) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        println!("Content-Type: {}", self.mime_type());
        self.output(None)
    }
    
    /// Guarda la imagen en un archivo
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<bool> {
        let path = match &file_path {
            Some(p) => p.as_ref(),
            None => match &self.file_path {
                Some(p) => p.as_path(),
                None => return Err(ImageError::NoImageLoaded),
            },
        };
        
        self.output(Some(path))
    }
    
    /// Método interno para mostrar o guardar la imagen
    fn output<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<bool> {
        if let Some(path) = file_path {
            let path = path.as_ref();
            
            // Asegurarse de que el directorio existe
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
                
                if !parent.is_dir() {
                    return Err(ImageError::DirectoryNotWritable(parent.display().to_string()));
                }
            }
            
            // Verificar permisos de escritura
            if path.exists() && fs::metadata(path)?.permissions().readonly() {
                return Err(ImageError::FileNotWritable(path.display().to_string()));
            }
        }
        
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_ref().unwrap();
        
        let result = match file_path {
            Some(path) => {
                // Guardar en archivo
                let format = match self.image_type {
                    ImageFormat::Gif => ImageOutputFormat::Gif,
                    ImageFormat::Jpeg => ImageOutputFormat::Jpeg(90), // Calidad 90 por defecto
                    ImageFormat::Png => ImageOutputFormat::Png,
                    ImageFormat::WebP => ImageOutputFormat::WebP,
                    ImageFormat::Bmp => ImageOutputFormat::Bmp,
                    _ => ImageOutputFormat::Png, // Por defecto, usar PNG
                };
                
                let path = path.as_ref();
                img.save_with_format(path, format.into()).is_ok()
            },
            None => {
                // Mostrar en stdout
                let mut bytes: Vec<u8> = Vec::new();
                let format = match self.image_type {
                    ImageFormat::Gif => ImageOutputFormat::Gif,
                    ImageFormat::Jpeg => ImageOutputFormat::Jpeg(90),
                    ImageFormat::Png => ImageOutputFormat::Png,
                    ImageFormat::WebP => ImageOutputFormat::WebP,
                    ImageFormat::Bmp => ImageOutputFormat::Bmp,
                    _ => ImageOutputFormat::Png,
                };
                
                img.write_to(&mut Cursor::new(&mut bytes), format).is_ok() && io::stdout().write_all(&bytes).is_ok()
            }
        };
        
        Ok(result)
    }
    
    /// Devuelve los datos de la imagen como un vector de bytes
    pub fn data(&self) -> Result<Vec<u8>> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_ref().unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        
        let format = match self.mime_type.as_str() {
            "image/png" => ImageOutputFormat::Png,
            "image/jpeg" => ImageOutputFormat::Jpeg(90),
            "image/gif" => ImageOutputFormat::Gif,
            _ => {
                info!("Image::data. Couldn't guess mimetype, defaulting to png");
                ImageOutputFormat::Png
            }
        };
        
        match img.write_to(&mut Cursor::new(&mut bytes), format) {
            Ok(_) => Ok(bytes),
            Err(_) => {
                error!("Image::data. Error getting image data.");
                Err(ImageError::InvalidData)
            }
        }
    }
    
    /// Devuelve los datos de la imagen como un string codificado en base64
    pub fn to_base64(&self) -> Result<String> {
        let data = self.data()?;
        Ok(BASE64.encode(&data))
    }
    
    /// Obtiene la orientación basándose en datos EXIF
    pub fn get_orientation(&self) -> i32 {
        // Comprobar si tenemos un archivo para leer los datos EXIF
        if !self.valid() {
            debug!("Image::get_orientation() No image loaded.");
            return -1;
        }
        
        let file_path = match &self.file_path {
            Some(path) => path,
            None => {
                debug!("Image::get_orientation() No readable file path set.");
                return -1;
            }
        };
        
        if !file_path.exists() || !file_path.is_file() {
            debug!("Image::get_orientation() File does not exist or is not readable.");
            return -1;
        }
        
        // Leer datos EXIF
        let file = match fs::File::open(file_path) {
            Ok(f) => f,
            Err(_) => return -1,
        };
        
        let mut bufreader = BufReader::new(&file);
        let exifreader = match ExifReader::new().read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(_) => return -1,
        };
        
        // Extraer la orientación
        let orientation = match exifreader.get_field(Tag::Orientation, In::PRIMARY) {
            Some(orientation) => match orientation.value.get_uint(0) {
                Some(v) => v as i32,
                None => -1,
            },
            None => -1,
        };
        
        orientation
    }
    
    /// Corrige la orientación basándose en datos EXIF
    pub fn fix_orientation(&mut self) -> Result<bool> {
        let orientation = self.get_orientation();
        debug!("Image::fix_orientation() Orientation: {}", orientation);
        
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let mut rotate = 0;
        let flip = false; // Implementación del flip pendiente
        
        match orientation {
            -1 => return Ok(false), // Nada que corregir
            1 => (),                // Orientación correcta
            2 => {
                // No testeado
                // rotate = 0;
                // flip = true;
            },
            3 => rotate = 180,
            4 => {
                // No testeado
                // rotate = 180;
                // flip = true;
            },
            5 => {
                // No testeado
                // rotate = 90;
                // flip = true;
            },
            6 => rotate = 270,
            7 => {
                // No testeado
                // rotate = 270;
                // flip = true;
            },
            8 => rotate = 90,
            _ => (),
        }
        
        if rotate > 0 {
            let img = self.image.as_mut().unwrap();
            *img = img.rotate(rotate);
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Carga una imagen desde diferentes fuentes
    pub fn load<T>(&mut self, image_ref: T) -> Result<&DynamicImage>
    where
        T: AsRef<[u8]> + AsRef<Path> + AsRef<str> + std::fmt::Debug,
    {
        // Primero intentar cargar como archivo
        match self.load_from_file(&image_ref) {
            Ok(img) => return Ok(img),
            Err(_) => {}, // Intentar siguientes métodos
        }
        
        // Intentar como base64
        match self.load_from_base64(image_ref.as_ref()) {
            Ok(img) => return Ok(img),
            Err(_) => {}, // Intentar siguientes métodos
        }
        
        // Intentar como datos binarios
        match self.load_from_data(image_ref) {
            Ok(img) => return Ok(img),
            Err(_) => {
                debug!("Image::load(): couldn't load anything. Giving up!");
                return Err(ImageError::InvalidData);
            }
        }
    }
    
    /// Carga una imagen desde un handle de archivo abierto
    pub fn load_from_file_handle<R: Read + Seek>(&mut self, handle: &mut R) -> Result<&DynamicImage> {
        debug!("Image::load_from_file_handle(): Trying");
        
        let mut contents = Vec::new();
        handle.read_to_end(&mut contents)?;
        
        self.load_from_data(&contents)
    }
    
    /// Carga una imagen desde un archivo local
    pub fn load_from_file<P: AsRef<Path>>(&mut self, image_path: P) -> Result<&DynamicImage> {
        let path = image_path.as_ref();
        
        // Verificar que el archivo existe y es suficientemente grande
        if !path.is_file() || fs::metadata(path)?.len() < 12 || !path.exists() {
            debug!("Image::load_from_file, couldn't load: {}", path.display());
            return Err(ImageError::InvalidData);
        }
        
        // Determinar el formato de la imagen
        let format = match image::ImageFormat::from_path(path) {
            Ok(format) => format,
            Err(_) => return Err(ImageError::UnsupportedFormat),
        };
        
        // Cargar la imagen
        let img = image::open(path)?;
        self.image = Some(img);
        self.image_type = format;
        self.mime_type = format.to_mime_type().to_string();
        self.file_path = Some(path.to_path_buf());
        
        Ok(self.image.as_ref().unwrap())
    }
    
    /// Carga una imagen desde datos binarios
    pub fn load_from_data<T: AsRef<[u8]>>(&mut self, data: T) -> Result<&DynamicImage> {
        let data = data.as_ref();
        
        // Intentar interpretar los datos como una imagen
        let format = match image::guess_format(data) {
            Ok(format) => format,
            Err(_) => return Err(ImageError::UnsupportedFormat),
        };
        
        let img = image::load_from_memory(data)?;
        
        self.image = Some(img);
        self.image_type = format;
        self.mime_type = format.to_mime_type().to_string();
        
        Ok(self.image.as_ref().unwrap())
    }
    
    /// Carga una imagen desde un string base64
    pub fn load_from_base64(&mut self, data: &str) -> Result<&DynamicImage> {
        if data.is_empty() {
            return Err(ImageError::InvalidData);
        }
        
        let decoded = match BASE64.decode(data) {
            Ok(decoded) => decoded,
            Err(_) => return Err(ImageError::InvalidData),
        };
        
        self.load_from_data(&decoded)
    }
    
    /// Redimensiona la imagen preservando la proporción
    pub fn resize(&mut self, max_size: u32) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_ref().unwrap();
        let width_orig = img.width();
        let height_orig = img.height();
        let ratio_orig = width_orig as f64 / height_orig as f64;
        
        let (new_width, new_height) = if ratio_orig > 1.0 {
            let new_height = (max_size as f64 / ratio_orig).round() as u32;
            (max_size, new_height)
        } else {
            let new_width = (max_size as f64 * ratio_orig).round() as u32;
            (new_width, max_size)
        };
        
        self.precise_resize(new_width, new_height)
    }
    
    /// Redimensiona la imagen a un tamaño exacto
    pub fn precise_resize(&mut self, width: u32, height: u32) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_mut().unwrap();
        *img = img.resize_exact(width, height, imageops::FilterType::Lanczos3);
        
        Ok(true)
    }
    
    /// Recorta la imagen en un cuadrado central
    pub fn center_crop(&mut self, size: u32) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_ref().unwrap();
        let width_orig = img.width();
        let height_orig = img.height();
        
        // Si ya es cuadrada y no se especifica tamaño, no hacer nada
        if width_orig == height_orig && size == 0 {
            return Ok(true);
        }
        
        let ratio_orig = width_orig as f64 / height_orig as f64;
        let dim = width_orig.min(height_orig);
        
        let (x, y) = if ratio_orig > 1.0 {
            ((width_orig - dim) / 2, 0)
        } else {
            (0, (height_orig - dim) / 2)
        };
        
        let target_size = if size > 0 { size } else { dim };
        
        // Recortar y redimensionar
        let mut new_img = self.image.as_mut().unwrap().crop(x, y, dim, dim);
        
        if target_size != dim {
            new_img = new_img.resize_exact(target_size, target_size, imageops::FilterType::Lanczos3);
        }
        
        self.image = Some(new_img);
        
        Ok(true)
    }
    
    /// Recorta la imagen desde el punto (x,y) con dimensión (w,h)
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let new_img = self.image.as_mut().unwrap().crop(x, y, w, h);
        self.image = Some(new_img);
        
        Ok(true)
    }
    
    /// Redimensiona la imagen para que se ajuste dentro de un límite conservando la proporción
    pub fn fit_in(&mut self, max_width: u32, max_height: u32) -> Result<bool> {
        if !self.valid() {
            return Err(ImageError::NoImageLoaded);
        }
        
        let img = self.image.as_ref().unwrap();
        let width_orig = img.width();
        let height_orig = img.height();
        let ratio = width_orig as f64 / height_orig as f64;
        
        let new_width = (max_width as f64).min(ratio * max_height as f64).round() as u32;
        let new_height = (max_height as f64).min(max_width as f64 / ratio).round() as u32;
        
        self.precise_resize(new_width, new_height)
    }
    
    /// Destruye el recurso de imagen
    pub fn destroy(&mut self) {
        self.image = None;
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_base64() {
            Ok(base64) => write!(f, "{}", base64),
            Err(_) => write!(f, ""),
        }
    }
}

impl std::ops::FnOnce<()> for Image {
    type Output = Result<bool>;
    
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.show()
    }
}

impl std::ops::FnMut<()> for Image {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.show()
    }
}

impl std::ops::Fn<()> for Image {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.show()
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}