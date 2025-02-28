use pretty_assertions::assert_eq;

/**
 * Copyright (c) 2012 Bernhard Posselt <nukeawhale@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

struct OcApp;

impl OcApp {
    /// Checks if the app is compatible with the current version of the Nextcloud server
    ///
    /// # Arguments
    ///
    /// * `oc_version` - The server version as array of integers
    /// * `app_version` - The app version as string
    ///
    /// # Returns
    ///
    /// * `bool` - True if the app is compatible, false otherwise
    pub fn is_app_version_compatible(oc_version: &[u32], app_version: &str) -> bool {
        let app_parts: Vec<u32> = app_version
            .split('.')
            .filter_map(|part| part.parse().ok())
            .collect();
        
        if app_parts.is_empty() {
            return false;
        }
        
        for (i, &app_part) in app_parts.iter().enumerate() {
            if i >= oc_version.len() {
                // If we've gone beyond the oc_version length, app is compatible
                return true;
            }
            
            if app_part > oc_version[i] {
                return false;
            }
            
            if app_part < oc_version[i] {
                return true;
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_app_version_compatible_single_oc_number() {
        let oc = vec![4];
        let app = "4.0";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_multiple_oc_number() {
        let oc = vec![4, 3, 1];
        let app = "4.3";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_single_number() {
        let oc = vec![4];
        let app = "4";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_single_app_number() {
        let oc = vec![4, 3];
        let app = "4";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_complex() {
        let oc = vec![5, 0, 0];
        let app = "4.5.1";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_should_fail() {
        let oc = vec![4, 3, 1];
        let app = "4.3.2";

        assert!(!OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_should_fail_two_version_numbers() {
        let oc = vec![4, 3, 1];
        let app = "4.4";

        assert!(!OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_should_work_for_pre_alpha() {
        let oc = vec![5, 0, 3];
        let app = "4.93";

        assert!(OcApp::is_app_version_compatible(&oc, app));
    }

    #[test]
    fn test_is_app_version_compatible_should_fail_one_version_numbers() {
        let oc = vec![4, 3, 1];
        let app = "5";

        assert!(!OcApp::is_app_version_compatible(&oc, app));
    }
}