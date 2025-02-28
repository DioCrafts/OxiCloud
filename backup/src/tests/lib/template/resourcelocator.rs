// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use std::path::{Path, PathBuf};
    
    // Trait equivalente a la clase abstracta
    #[automock]
    trait ResourceLocator {
        fn do_find(&self, resource: &str) -> Result<(), Box<dyn std::error::Error>>;
        fn do_find_theme(&self, resource: &str) -> Result<(), Box<dyn std::error::Error>>;
    }
    
    struct ResourceLocatorImpl {
        theme: String,
        form_factor: String,
        serverroot: String,
        mapping: HashMap<String, String>,
        thirdpartyroot: String,
        webroot: String,
        resources: Vec<(PathBuf, String, String)>,
    }
    
    impl ResourceLocatorImpl {
        fn new(
            theme: &str, 
            form_factor: &str, 
            core_map: HashMap<String, String>, 
            party_map: HashMap<String, String>, 
            appsroots: HashMap<String, String>
        ) -> Self {
            let serverroot = core_map.keys().next().unwrap_or(&String::new()).clone();
            let webroot = core_map.values().next().unwrap_or(&String::new()).clone();
            let thirdpartyroot = party_map.keys().next().unwrap_or(&String::new()).clone();
            
            let mut mapping = HashMap::new();
            mapping.extend(core_map);
            mapping.extend(party_map);
            
            ResourceLocatorImpl {
                theme: theme.to_string(),
                form_factor: form_factor.to_string(),
                serverroot,
                mapping,
                thirdpartyroot,
                webroot,
                resources: Vec::new(),
            }
        }
        
        fn find(&self, resources: &[String]) -> Result<(), Box<dyn std::error::Error>> {
            for resource in resources {
                match self.do_find(resource) {
                    Err(e) => {
                        let message = format!("{} formfactor:{} serverroot:{}", 
                            e, self.form_factor, self.serverroot);
                        return Err(Box::new(ResourceLocatorError::new(&message)));
                    },
                    Ok(_) => {},
                }
                self.do_find_theme(resource)?;
            }
            Ok(())
        }
        
        fn append_if_exist<P: AsRef<Path>>(&mut self, 
                                          base_dir: P, 
                                          resource: &str, 
                                          web_root: Option<&str>) -> bool {
            let file_path = base_dir.as_ref().join(resource);
            if file_path.exists() {
                let web_path = match web_root {
                    Some(root) => root.to_string(),
                    None => {
                        let base_path = base_dir.as_ref().to_string_lossy().to_string();
                        self.mapping.get(&base_path)
                               .unwrap_or(&self.webroot)
                               .clone()
                    }
                };
                
                self.resources.push((
                    base_dir.as_ref().to_path_buf(),
                    web_path,
                    resource.to_string()
                ));
                true
            } else {
                false
            }
        }
        
        fn get_resources(&self) -> &Vec<(PathBuf, String, String)> {
            &self.resources
        }
    }
    
    impl ResourceLocator for ResourceLocatorImpl {
        fn do_find(&self, _resource: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Implementación por defecto para pruebas
            Ok(())
        }
        
        fn do_find_theme(&self, _resource: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Implementación por defecto para pruebas
            Ok(())
        }
    }
    
    #[derive(Debug)]
    struct ResourceLocatorError {
        message: String,
    }
    
    impl ResourceLocatorError {
        fn new(message: &str) -> Self {
            ResourceLocatorError {
                message: message.to_string(),
            }
        }
    }
    
    impl std::error::Error for ResourceLocatorError {}
    
    impl fmt::Display for ResourceLocatorError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    fn get_resource_locator(
        theme: &str, 
        form_factor: &str, 
        core_map: HashMap<String, String>, 
        party_map: HashMap<String, String>, 
        appsroots: HashMap<String, String>
    ) -> MockResourceLocator {
        let mut mock = MockResourceLocator::new();
        // Configurar el mock según sea necesario
        mock
    }
    
    #[test]
    fn test_constructor() {
        let core_map = HashMap::from([("core".to_string(), "map".to_string())]);
        let party_map = HashMap::from([("3rd".to_string(), "party".to_string())]);
        let appsroots = HashMap::from([("foo".to_string(), "bar".to_string())]);
        
        let locator = ResourceLocatorImpl::new("theme", "form_factor", core_map, party_map, appsroots);
        
        assert_eq!(locator.theme, "theme");
        assert_eq!(locator.form_factor, "form_factor");
        assert_eq!(locator.serverroot, "core");
        
        let expected_mapping = HashMap::from([
            ("core".to_string(), "map".to_string()),
            ("3rd".to_string(), "party".to_string())
        ]);
        assert_eq!(locator.mapping, expected_mapping);
        
        assert_eq!(locator.thirdpartyroot, "3rd");
        assert_eq!(locator.webroot, "map");
        assert_eq!(locator.resources.len(), 0);
    }
    
    #[test]
    fn test_find() {
        let core_map = HashMap::from([("core".to_string(), "map".to_string())]);
        let party_map = HashMap::from([("3rd".to_string(), "party".to_string())]);
        let appsroots = HashMap::from([("foo".to_string(), "bar".to_string())]);
        
        let mut mock = get_resource_locator("theme", "form_factor", 
                                          core_map.clone(), party_map.clone(), appsroots.clone());
        mock.expect_do_find()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Ok(()));
        mock.expect_do_find_theme()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Ok(()));
        
        let resources = vec!["foo".to_string()];
        let result = ResourceLocator::do_find(&mock, "foo");
        assert!(result.is_ok());
        
        // Test para el caso de error
        let mut mock_err = get_resource_locator("theme", "form_factor", 
                                              core_map.clone(), party_map.clone(), appsroots.clone());
        mock_err.expect_do_find()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Err(Box::new(ResourceLocatorError::new("test"))));
        
        let result = ResourceLocator::do_find(&mock_err, "foo");
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "test");
        }
    }
    
    #[test]
    fn test_append_if_exist() {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_str = current_dir.to_string_lossy().to_string();
        let core_map = HashMap::from([(current_dir_str, "map".to_string())]);
        let party_map = HashMap::from([("3rd".to_string(), "party".to_string())]);
        let appsroots = HashMap::from([("foo".to_string(), "bar".to_string())]);
        
        let mut locator = ResourceLocatorImpl::new("theme", "form_factor", 
                                                 core_map, party_map, appsroots);
        
        let file_name = std::file!();
        
        // Test con webroot específico
        let result = locator.append_if_exist(&current_dir, file_name, Some("webroot"));
        assert!(result);
        
        let resources = locator.get_resources();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].0, current_dir);
        assert_eq!(resources[0].1, "webroot");
        assert_eq!(resources[0].2, file_name);
        
        // Test sin webroot específico
        let result = locator.append_if_exist(&current_dir, file_name, None);
        assert!(result);
        
        let resources = locator.get_resources();
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[1].0, current_dir);
        assert_eq!(resources[1].1, "map");
        assert_eq!(resources[1].2, file_name);
        
        // Test con archivo inexistente
        let result = locator.append_if_exist(&current_dir, "does-not-exist", None);
        assert!(!result);
        
        let resources = locator.get_resources();
        assert_eq!(resources.len(), 2); // No debería haber cambiado
    }
}