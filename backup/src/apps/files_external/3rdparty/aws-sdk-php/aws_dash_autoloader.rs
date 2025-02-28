/*
 * Copyright 2010-2013 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 * http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::env;

// Mock de la clase UniversalClassLoader de PHP para Rust
pub struct UniversalClassLoader {
    namespaces: HashMap<String, PathBuf>,
}

impl UniversalClassLoader {
    pub fn new() -> Self {
        UniversalClassLoader {
            namespaces: HashMap::new(),
        }
    }

    pub fn register_namespaces(&mut self, namespaces: HashMap<String, PathBuf>) {
        for (namespace, path) in namespaces {
            self.namespaces.insert(namespace, path);
        }
    }

    pub fn register(&self) -> Result<(), Box<dyn Error>> {
        // En Rust no hay un concepto directo de autoloader como en PHP
        // Esta función simularía la registración del autoloader
        Ok(())
    }
}

pub fn init_aws_loader() -> Result<UniversalClassLoader, Box<dyn Error>> {
    // Determinar el prefijo de ruta para los archivos
    let current_dir = env::current_dir()?;
    let aws_file_prefix = env::var("AWS_FILE_PREFIX").unwrap_or_else(|_| current_dir.to_string_lossy().to_string());
    let prefix_path = PathBuf::from(&aws_file_prefix);

    // Crear y configurar el cargador de clases
    let mut class_loader = UniversalClassLoader::new();
    
    let mut namespaces = HashMap::new();
    namespaces.insert("Aws".to_string(), prefix_path.clone());
    namespaces.insert("Guzzle".to_string(), prefix_path.clone());
    namespaces.insert("Symfony".to_string(), prefix_path.clone());
    namespaces.insert("Doctrine".to_string(), prefix_path.clone());
    namespaces.insert("Psr".to_string(), prefix_path.clone());
    namespaces.insert("Monolog".to_string(), prefix_path.clone());
    
    class_loader.register_namespaces(namespaces);
    class_loader.register()?;

    Ok(class_loader)
}