//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Once};

/// Stream wrapper that provides a callback on stream close
pub struct Close {
    path: PathBuf,
    source: Option<File>,
    meta: Option<StreamMetaData>,
}

struct StreamMetaData {
    // Aquí se guardarían metadatos del stream como en PHP
}

type Callback = Box<dyn Fn(&Path) + Send + Sync>;

lazy_static::lazy_static! {
    static ref CALLBACKS: Mutex<HashMap<PathBuf, Callback>> = Mutex::new(HashMap::new());
    static ref OPEN_PATHS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
}

impl Close {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            source: None,
            meta: None,
        }
    }

    pub fn stream_open(&mut self, path: &str, mode: &str) -> io::Result<bool> {
        // Eliminar el prefijo 'close://'
        let path_str = path.strip_prefix("close://").unwrap_or(path);
        self.path = PathBuf::from(path_str);
        
        let file = OpenOptions::new()
            .read(mode.contains('r'))
            .write(mode.contains('w'))
            .append(mode.contains('a'))
            .create(mode.contains('w') || mode.contains('a'))
            .truncate(mode.contains('w'))
            .open(&self.path)?;
        
        self.source = Some(file);
        self.meta = Some(StreamMetaData {}); // Inicializar metadatos

        let mut open_paths = OPEN_PATHS.lock().unwrap();
        open_paths.push(self.path.clone());
        
        Ok(true)
    }

    pub fn stream_seek(&mut self, offset: i64, whence: SeekFrom) -> io::Result<u64> {
        if let Some(file) = &mut self.source {
            file.seek(whence)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_tell(&mut self) -> io::Result<u64> {
        if let Some(file) = &mut self.source {
            file.stream_position()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_read(&mut self, count: usize) -> io::Result<Vec<u8>> {
        if let Some(file) = &mut self.source {
            let mut buffer = vec![0; count];
            let bytes_read = file.read(&mut buffer)?;
            buffer.truncate(bytes_read);
            Ok(buffer)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_write(&mut self, data: &[u8]) -> io::Result<usize> {
        if let Some(file) = &mut self.source {
            file.write(data)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_set_option(&mut self, option: StreamOption, arg1: usize, arg2: Option<usize>) -> io::Result<()> {
        if let Some(file) = &mut self.source {
            match option {
                StreamOption::Blocking => {
                    // En Rust, los archivos son bloqueantes por defecto
                    // Esta funcionalidad requeriría más implementación si es necesaria
                    Ok(())
                },
                StreamOption::ReadTimeout => {
                    // Configurar timeout de lectura
                    // Necesitaría más implementación específica
                    Ok(())
                },
                StreamOption::WriteBuffer => {
                    // Configurar buffer de escritura
                    // Necesitaría más implementación específica
                    Ok(())
                },
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_stat(&mut self) -> io::Result<std::fs::Metadata> {
        if let Some(file) = &self.source {
            file.metadata()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_lock(&mut self, _mode: LockMode) -> io::Result<()> {
        // Implementación de bloqueo de archivo
        // Requeriría usar funciones específicas del sistema
        Ok(())
    }

    pub fn stream_flush(&mut self) -> io::Result<()> {
        if let Some(file) = &mut self.source {
            file.flush()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn stream_eof(&mut self) -> io::Result<bool> {
        // Determinar si estamos al final del archivo
        // Esto requeriría una implementación más compleja en Rust
        Ok(false)
    }

    pub fn url_stat(path: &str) -> io::Result<std::fs::Metadata> {
        let path_str = path.strip_prefix("close://").unwrap_or(path);
        let path = Path::new(path_str);
        
        if path.exists() {
            std::fs::metadata(path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"))
        }
    }

    pub fn stream_close(&mut self) -> io::Result<()> {
        if let Some(file) = self.source.take() {
            // El archivo se cerrará automáticamente cuando file salga de ámbito
            drop(file);
            
            let callbacks = CALLBACKS.lock().unwrap();
            if let Some(callback) = callbacks.get(&self.path) {
                callback(&self.path);
            }
            
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file open"))
        }
    }

    pub fn unlink(path: &str) -> io::Result<()> {
        let path_str = path.strip_prefix("close://").unwrap_or(path);
        std::fs::remove_file(path_str)
    }

    pub fn register_callback<F>(path: &Path, callback: F) 
    where 
        F: Fn(&Path) + Send + Sync + 'static
    {
        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.insert(path.to_path_buf(), Box::new(callback));
    }
}

pub enum StreamOption {
    Blocking,
    ReadTimeout,
    WriteBuffer,
}

pub enum LockMode {
    Shared,
    Exclusive,
}

impl Drop for Close {
    fn drop(&mut self) {
        // Asegurarse de que el archivo se cierre y se llame al callback si aún no se ha hecho
        if self.source.is_some() {
            let _ = self.stream_close();
        }
    }
}