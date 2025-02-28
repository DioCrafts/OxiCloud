// ownCloud
//
// @author Michael Gapczynski
// @copyright 2012 Michael Gapczynski mtgap@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ShareItem {
    pub item_source: String,
    pub item_target: String,
    pub permissions: u32,
}

pub trait ShareBackend {
    fn is_valid_source(&self, item_source: &str, uid_owner: &str) -> bool;
    fn generate_target(&self, item_source: &str, share_with: &str, exclude: Option<&[String]>) -> String;
    fn format_items(&self, items: &[ShareItem], format: u8, parameters: Option<&str>) -> Vec<String>;
}

pub struct TestShareBackend {
    test_item1: String,
    test_item2: String,
}

impl TestShareBackend {
    pub const FORMAT_SOURCE: u8 = 0;
    pub const FORMAT_TARGET: u8 = 1;
    pub const FORMAT_PERMISSIONS: u8 = 2;

    pub fn new() -> Self {
        TestShareBackend {
            test_item1: "test.txt".to_string(),
            test_item2: "share.txt".to_string(),
        }
    }
}

impl Default for TestShareBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ShareBackend for TestShareBackend {
    fn is_valid_source(&self, item_source: &str, _uid_owner: &str) -> bool {
        item_source == self.test_item1 || item_source == self.test_item2
    }

    fn generate_target(&self, _item_source: &str, _share_with: &str, exclude: Option<&[String]>) -> String {
        // Always make target be test.txt to cause conflicts
        let target = "test.txt".to_string();
        
        if let Some(exclude_list) = exclude {
            let exclude_set: HashSet<_> = exclude_list.iter().collect();
            
            if !exclude_set.contains(&target) {
                return target;
            }
            
            if let Some(pos) = target.rfind('.') {
                let name = &target[0..pos];
                let ext = &target[pos..];
                let mut i = 1;
                
                loop {
                    let new_target = format!("{}{}{}", name, i, ext);
                    if !exclude_set.contains(&new_target) {
                        return new_target;
                    }
                    i += 1;
                }
            }
        }
        
        target
    }

    fn format_items(&self, items: &[ShareItem], format: u8, _parameters: Option<&str>) -> Vec<String> {
        items.iter().map(|item| {
            match format {
                Self::FORMAT_SOURCE => item.item_source.clone(),
                Self::FORMAT_TARGET => item.item_target.clone(),
                Self::FORMAT_PERMISSIONS => item.permissions.to_string(),
                _ => String::new(),
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_source() {
        let backend = TestShareBackend::new();
        assert!(backend.is_valid_source("test.txt", "user1"));
        assert!(backend.is_valid_source("share.txt", "user1"));
        assert!(!backend.is_valid_source("other.txt", "user1"));
    }

    #[test]
    fn test_generate_target() {
        let backend = TestShareBackend::new();
        assert_eq!(backend.generate_target("any", "user", None), "test.txt");
        
        let exclude = vec!["test.txt".to_string()];
        assert_eq!(backend.generate_target("any", "user", Some(&exclude)), "test1.txt");
        
        let exclude = vec!["test.txt".to_string(), "test1.txt".to_string()];
        assert_eq!(backend.generate_target("any", "user", Some(&exclude)), "test2.txt");
    }

    #[test]
    fn test_format_items() {
        let backend = TestShareBackend::new();
        let items = vec![
            ShareItem {
                item_source: "source1".to_string(),
                item_target: "target1".to_string(),
                permissions: 1,
            },
            ShareItem {
                item_source: "source2".to_string(),
                item_target: "target2".to_string(),
                permissions: 2,
            },
        ];
        
        assert_eq!(
            backend.format_items(&items, TestShareBackend::FORMAT_SOURCE, None),
            vec!["source1", "source2"]
        );
        
        assert_eq!(
            backend.format_items(&items, TestShareBackend::FORMAT_TARGET, None),
            vec!["target1", "target2"]
        );
        
        assert_eq!(
            backend.format_items(&items, TestShareBackend::FORMAT_PERMISSIONS, None),
            vec!["1", "2"]
        );
    }
}