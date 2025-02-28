use md5::{Digest, Md5};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::fmt;
use once_cell::sync::Lazy;
use regex::Regex;
use unidecode::unidecode;
use uuid::Uuid;

/// Representa errores específicos del Mapper
#[derive(Debug)]
pub enum MapperError {
    DatabaseError(String),
    PathError(String),
    InvalidPath(String),
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapperError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            MapperError::PathError(msg) => write!(f, "Path error: {}", msg),
            MapperError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
        }
    }
}

impl Error for MapperError {}

/// Struct para representar una fila de la tabla file_map
struct FileMapRow {
    logic_path: String,
    physic_path: String,
    logic_path_hash: String,
    physic_path_hash: String,
}

/// Estructura para realizar operaciones de base de datos
/// Esto es un mock que deberá reemplazarse con la implementación real
struct DB;

impl DB {
    async fn execute_audited(sql: &str, params: Vec<String>) -> Result<Vec<FileMapRow>, MapperError> {
        // Esta es una implementación ficticia que debería reemplazarse con la real
        // Aquí se ejecutaría la consulta SQL real y se devolverían los resultados
        Err(MapperError::DatabaseError("DB implementation needed".to_string()))
    }
}

/// Clase Mapper es responsable de traducir rutas lógicas a físicas y viceversa
pub struct Mapper {
    unchanged_physical_root: PathBuf,
}

impl Mapper {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Mapper {
            unchanged_physical_root: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Convierte una ruta lógica a física
    ///
    /// # Arguments
    ///
    /// * `logic_path` - La ruta lógica a convertir
    /// * `create` - Indica si la ruta física generada debe almacenarse en la base de datos
    ///
    /// # Returns
    ///
    /// La ruta física correspondiente
    pub async fn logic_to_physical(&self, logic_path: &str, create: bool) -> Result<String, MapperError> {
        if let Some(physical_path) = self.resolve_logic_path(logic_path).await? {
            return Ok(physical_path);
        }

        self.create(logic_path, create).await
    }

    /// Convierte una ruta física a lógica
    ///
    /// # Arguments
    ///
    /// * `physical_path` - La ruta física a convertir
    ///
    /// # Returns
    ///
    /// La ruta lógica correspondiente
    pub async fn physical_to_logic(&self, physical_path: &str) -> Result<String, MapperError> {
        if let Some(logic_path) = self.resolve_physical_path(physical_path).await? {
            return Ok(logic_path);
        }

        self.insert(physical_path, physical_path).await?;
        Ok(physical_path.to_string())
    }

    /// Elimina una ruta de la base de datos
    ///
    /// # Arguments
    ///
    /// * `path` - La ruta a eliminar
    /// * `is_logic_path` - Indica si la ruta es lógica o física
    /// * `recursive` - Indica si se debe eliminar recursivamente
    pub async fn remove_path(&self, path: &str, is_logic_path: bool, recursive: bool) -> Result<(), MapperError> {
        let mut path_pattern = path.to_string();
        if recursive {
            path_pattern.push_str("%");
        }

        let sql = if is_logic_path {
            "DELETE FROM `*PREFIX*file_map` WHERE `logic_path` LIKE ?"
        } else {
            "DELETE FROM `*PREFIX*file_map` WHERE `physic_path` LIKE ?"
        };

        DB::execute_audited(sql, vec![path_pattern]).await?;
        Ok(())
    }

    /// Copia una ruta a otra
    ///
    /// # Arguments
    ///
    /// * `path1` - Ruta origen
    /// * `path2` - Ruta destino
    pub async fn copy(&self, path1: &str, path2: &str) -> Result<(), MapperError> {
        let path1 = self.strip_last(path1);
        let path2 = self.strip_last(path2);
        let physic_path1 = self.logic_to_physical(&path1, true).await?;
        let physic_path2 = self.logic_to_physical(&path2, true).await?;

        let sql = "SELECT * FROM `*PREFIX*file_map` WHERE `logic_path` LIKE ?";
        let result = DB::execute_audited(sql, vec![format!("{}%", path1)]).await?;

        for row in result {
            let current_logic = &row.logic_path;
            let current_physic = &row.physic_path;
            
            if let Some(stripped_logic) = self.strip_root_folder(current_logic, &path1) {
                if let Some(stripped_physic) = self.strip_root_folder(current_physic, &physic_path1) {
                    let new_logic = format!("{}{}", path2, stripped_logic);
                    let new_physic = format!("{}{}", physic_path2, stripped_physic);
                    
                    if path1 != *current_logic {
                        let update_sql = "UPDATE `*PREFIX*file_map` SET `logic_path` = ?, `logic_path_hash` = ?, `physic_path` = ?, `physic_path_hash` = ? WHERE `logic_path` = ?";
                        let logic_hash = format!("{:x}", Md5::digest(new_logic.as_bytes()));
                        let physic_hash = format!("{:x}", Md5::digest(new_physic.as_bytes()));
                        
                        match DB::execute_audited(update_sql, vec![
                            new_logic.clone(),
                            logic_hash,
                            new_physic.clone(),
                            physic_hash,
                            current_logic.clone()
                        ]).await {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("Mapper::Copy failed {} -> {}\n{}", current_logic, new_logic, e);
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Elimina la carpeta raíz de una ruta
    ///
    /// # Arguments
    ///
    /// * `path` - La ruta completa
    /// * `root` - La carpeta raíz a eliminar
    ///
    /// # Returns
    ///
    /// La ruta sin la carpeta raíz, o None si la ruta no comienza con root
    pub fn strip_root_folder(&self, path: &str, root: &str) -> Option<String> {
        if !path.starts_with(root) {
            return None;
        }
        
        if path.len() > root.len() {
            Some(path[root.len()..].to_string())
        } else {
            Some(String::new())
        }
    }

    fn strip_last(&self, path: &str) -> String {
        if path.ends_with('/') {
            path[..path.len() - 1].to_string()
        } else {
            path.to_string()
        }
    }

    async fn resolve_logic_path(&self, logic_path: &str) -> Result<Option<String>, MapperError> {
        let logic_path = self.strip_last(logic_path);
        let logic_hash = format!("{:x}", Md5::digest(logic_path.as_bytes()));
        
        let sql = "SELECT * FROM `*PREFIX*file_map` WHERE `logic_path_hash` = ?";
        let result = DB::execute_audited(sql, vec![logic_hash]).await?;
        
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result[0].physic_path.clone()))
        }
    }

    async fn resolve_physical_path(&self, physical_path: &str) -> Result<Option<String>, MapperError> {
        let physical_path = self.strip_last(physical_path);
        let physic_hash = format!("{:x}", Md5::digest(physical_path.as_bytes()));
        
        let sql = "SELECT * FROM `*PREFIX*file_map` WHERE `physic_path_hash` = ?";
        let result = DB::execute_audited(sql, vec![physic_hash]).await?;
        
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result[0].logic_path.clone()))
        }
    }

    async fn create(&self, logic_path: &str, store: bool) -> Result<String, MapperError> {
        let logic_path = self.strip_last(logic_path);
        let mut index = 0;

        // Crear la ruta slugificada
        let mut physical_path = self.slugify_path(&logic_path, None)?;

        // Detectar duplicados
        while let Some(_) = self.resolve_physical_path(&physical_path).await? {
            physical_path = self.slugify_path(&logic_path, Some(index))?;
            index += 1;
        }

        // Insertar el nuevo mapeo de ruta si se solicita
        if store {
            self.insert(&logic_path, &physical_path).await?;
        }

        Ok(physical_path)
    }

    async fn insert(&self, logic_path: &str, physical_path: &str) -> Result<(), MapperError> {
        let sql = "INSERT INTO `*PREFIX*file_map` (`logic_path`, `physic_path`, `logic_path_hash`, `physic_path_hash`) VALUES (?, ?, ?, ?)";
        let logic_hash = format!("{:x}", Md5::digest(logic_path.as_bytes()));
        let physic_hash = format!("{:x}", Md5::digest(physical_path.as_bytes()));
        
        DB::execute_audited(sql, vec![
            logic_path.to_string(),
            physical_path.to_string(),
            logic_hash,
            physic_hash
        ]).await?;
        
        Ok(())
    }

    pub fn slugify_path(&self, path: &str, index: Option<usize>) -> Result<String, MapperError> {
        let stripped_path = match self.strip_root_folder(path, &self.unchanged_physical_root.to_string_lossy()) {
            Some(p) => p,
            None => return Err(MapperError::InvalidPath("Failed to strip root folder".to_string())),
        };

        let path_elements: Vec<&str> = stripped_path.split('/').collect();
        let mut slugged_elements = Vec::new();
        
        for path_element in path_elements {
            // Eliminar elementos vacíos
            if path_element.is_empty() {
                continue;
            }

            slugged_elements.push(Self::slugify(path_element)?);
        }

        // Aplicar índice al nombre de archivo si es necesario
        if let Some(idx) = index {
            if let Some(last) = slugged_elements.pop() {
                static EXTENSION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.([^\.]+)$").unwrap());
                
                if let Some(captures) = EXTENSION_RE.captures(&last) {
                    let extension = captures.get(0).unwrap().as_str();
                    let base_name = &last[..last.len() - extension.len()];
                    slugged_elements.push(format!("{}-{}{}", base_name, idx, extension));
                } else {
                    slugged_elements.push(format!("{}-{}", last, idx));
                }
            }
        }

        let slugged_path = format!("{}{}", 
            self.unchanged_physical_root.to_string_lossy(), 
            slugged_elements.join("/")
        );
        
        Ok(self.strip_last(&slugged_path))
    }

    /// Modifica una cadena para eliminar todos los caracteres no ASCII y espacios
    ///
    /// # Arguments
    ///
    /// * `text` - El texto a modificar
    ///
    /// # Returns
    ///
    /// El texto modificado
    fn slugify(text: &str) -> Result<String, MapperError> {
        static NON_ALNUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9\.]+").unwrap());
        static TRAILING_DOTS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.+$").unwrap());
        
        // Reemplazar caracteres no alfanuméricos o puntos por -
        let text = NON_ALNUM_RE.replace_all(text, "-");
        
        // Eliminar guiones al inicio y al final
        let text = text.trim_matches('-');
        
        // Transliterar: convertir caracteres no ASCII a ASCII
        let text = unidecode(text);
        
        // Convertir a minúsculas
        let text = text.to_lowercase();
        
        // Eliminar caracteres no deseados
        let text = text.replace(|c: char| !(c.is_alphanumeric() || c == '-' || c == '.'), "");
        
        // Eliminar puntos al final (por razones de seguridad y compatibilidad con Windows)
        let text = TRAILING_DOTS_RE.replace(&text, "").to_string();
        
        if text.is_empty() {
            Ok(Uuid::new_v4().to_string())
        } else {
            Ok(text)
        }
    }
}