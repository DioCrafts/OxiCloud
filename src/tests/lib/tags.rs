// Copyright (c) 2012-13 Thomas Tanghus (thomas@tanghus.net)
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

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use uuid::Uuid;

// Mocking user management functionality
struct UserBackend {
    users: HashMap<String, String>,
}

impl UserBackend {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn create_user(&mut self, username: &str, password: &str) -> bool {
        self.users.insert(username.to_string(), password.to_string());
        true
    }
}

struct User {
    id: String,
}

impl User {
    fn set_user_id(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }

    fn clear_backends() {
        // Mock implementation
    }

    fn use_backend(_backend: &str) {
        // Mock implementation
    }

    fn create_user(username: &str, password: &str) -> bool {
        let mut backend = UserBackend::new();
        backend.create_user(username, password)
    }
}

// Tag management functionality
#[derive(Debug, Clone)]
struct Tag {
    id: i64,
    name: String,
}

struct TagManager {
    user_id: String,
    taggers: HashMap<String, Arc<Tagger>>,
}

impl TagManager {
    fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            taggers: HashMap::new(),
        }
    }

    fn load(&mut self, object_type: &str, default_tags: Option<Vec<&str>>) -> Arc<Tagger> {
        if let Some(tagger) = self.taggers.get(object_type) {
            return tagger.clone();
        }

        let tagger = Arc::new(Tagger::new(object_type, self.user_id.clone()));

        if let Some(tags) = default_tags {
            let tags_vec: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
            tagger.add_multiple(&tags_vec);
        }

        self.taggers.insert(object_type.to_string(), tagger.clone());
        tagger
    }
}

#[derive(Debug)]
struct Tagger {
    object_type: String,
    user_id: String,
    tags: HashMap<String, Tag>,
    tagged_objects: HashMap<String, Vec<i64>>,
    favorites: Vec<i64>,
}

impl Tagger {
    fn new(object_type: &str, user_id: String) -> Self {
        Self {
            object_type: object_type.to_string(),
            user_id,
            tags: HashMap::new(),
            tagged_objects: HashMap::new(),
            favorites: Vec::new(),
        }
    }

    fn add(&self, tag_name: &str) -> i64 {
        let normalized_name = tag_name.to_lowercase();
        
        if self.has_tag(&normalized_name) {
            return 0;
        }

        // In a real implementation, this would interact with a database
        let id = thread_rng().gen_range(1..1000);
        let tag = Tag {
            id,
            name: normalized_name.clone(),
        };

        // Since self is immutable, in a real implementation we'd need to use interior mutability
        // or return a new state. For this test mock, we'd modify self.tags
        // self.tags.insert(normalized_name, tag);
        
        id
    }

    fn add_multiple(&self, tags: &[String]) -> bool {
        let mut all_added = true;
        
        for tag in tags {
            if self.add(tag) == 0 {
                all_added = false;
            }
        }
        
        all_added
    }

    fn get_tags(&self) -> Vec<Tag> {
        self.tags.values().cloned().collect()
    }

    fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    fn delete(&self, tags: &str) -> bool {
        // In a real implementation, this would remove from self.tags
        true
    }

    fn delete_multiple(&self, tags: &[String]) -> bool {
        // In a real implementation, this would remove multiple tags
        true
    }

    fn rename(&self, old_name: &str, new_name: &str) -> bool {
        let old_name_lower = old_name.to_lowercase();
        
        if !self.has_tag(&old_name_lower) {
            return false;
        }
        
        let new_name_lower = new_name.to_lowercase();
        if self.has_tag(&new_name_lower) {
            return false;
        }
        
        // In a real implementation, this would update the tag name
        true
    }

    fn tag_as(&self, obj_id: i64, tag: &str) -> bool {
        let tag_lower = tag.to_lowercase();
        
        if !self.has_tag(&tag_lower) {
            self.add(&tag_lower);
        }
        
        // In a real implementation, this would add to tagged_objects
        true
    }

    fn untag(&self, obj_id: i64, tag: &str) -> bool {
        let tag_lower = tag.to_lowercase();
        
        if !self.has_tag(&tag_lower) {
            return false;
        }
        
        // In a real implementation, this would remove from tagged_objects
        true
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains_key(&tag.to_lowercase())
    }

    fn get_ids_for_tag(&self, tag: &str) -> Vec<i64> {
        let tag_lower = tag.to_lowercase();
        
        match self.tagged_objects.get(&tag_lower) {
            Some(ids) => ids.clone(),
            None => Vec::new(),
        }
    }

    fn add_to_favorites(&self, obj_id: i64) -> bool {
        // In a real implementation, this would add to favorites
        true
    }

    fn remove_from_favorites(&self, obj_id: i64) -> bool {
        // In a real implementation, this would remove from favorites
        true
    }
}

// Test framework implementations
trait TestCase {
    fn set_up(&mut self);
    fn tear_down(&mut self);
}

struct TestTags {
    object_type: String,
    user: String,
    tag_mgr: Option<TagManager>,
}

impl TestTags {
    fn new() -> Self {
        Self {
            object_type: String::new(),
            user: String::new(),
            tag_mgr: None,
        }
    }

    fn test_instantiate_with_defaults(&self) {
        let default_tags = vec!["Friends", "Family", "Work", "Other"];
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, Some(default_tags));
        
        assert_eq!(4, tagger.get_tags().len());
    }

    fn test_add_tags(&self) {
        let tags = vec!["Friends", "Family", "Work", "Other"];
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        
        for tag in &tags {
            let result = tagger.add(tag);
            assert!(result > 0, "add() returned an ID <= 0");
            assert!(result > 0);
        }
        
        assert_eq!(false, tagger.add("Family") > 0);
        assert_eq!(false, tagger.add("fAMILY") > 0);
        
        assert_eq!(4, tagger.get_tags().len(), "Wrong number of added tags");
    }

    fn test_add_multiple(&self) {
        let tags = vec!["Friends", "Family", "Work", "Other"];
        let tags_str: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        
        for tag in &tags {
            assert_eq!(false, tagger.has_tag(tag));
        }
        
        let result = tagger.add_multiple(&tags_str);
        assert!(result);
        
        for tag in &tags {
            assert!(tagger.has_tag(tag));
        }
        
        assert_eq!(4, tagger.get_tags().len(), "Not all tags added");
    }

    fn test_is_empty(&self) {
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        
        assert_eq!(0, tagger.get_tags().len());
        assert!(tagger.is_empty());
        
        let result = tagger.add("Tag");
        assert!(result > 0, "add() returned an ID <= 0");
        assert!(result > 0, "add() returned false");
        assert_eq!(false, tagger.is_empty());
    }

    fn test_delete_tags(&self) {
        let default_tags = vec!["Friends", "Family", "Work", "Other"];
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, Some(default_tags));
        
        assert_eq!(4, tagger.get_tags().len());
        
        tagger.delete("family");
        assert_eq!(3, tagger.get_tags().len());
        
        let tags_to_delete = vec!["Friends".to_string(), "Work".to_string(), "Other".to_string()];
        tagger.delete_multiple(&tags_to_delete);
        assert_eq!(0, tagger.get_tags().len());
    }

    fn test_rename_tag(&self) {
        let default_tags = vec!["Friends", "Family", "Wrok", "Other"];
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, Some(default_tags));
        
        assert!(tagger.rename("Wrok", "Work"));
        assert!(tagger.has_tag("Work"));
        assert_eq!(false, tagger.has_tag("Wrok"));
        assert_eq!(false, tagger.rename("Wrok", "Work"));
    }

    fn test_tag_as(&self) {
        let obj_ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        
        for id in &obj_ids {
            tagger.tag_as(*id, "Family");
        }
        
        assert_eq!(1, tagger.get_tags().len());
        assert_eq!(9, tagger.get_ids_for_tag("Family").len());
    }

    fn test_untag(&self) {
        let obj_ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        self.test_tag_as();
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        
        for id in &obj_ids {
            assert!(tagger.get_ids_for_tag("Family").contains(id));
            tagger.untag(*id, "Family");
            assert_eq!(false, tagger.get_ids_for_tag("Family").contains(id));
        }
        
        assert_eq!(1, tagger.get_tags().len());
        assert_eq!(0, tagger.get_ids_for_tag("Family").len());
    }

    fn test_favorite(&self) {
        let tagger = self.tag_mgr.as_ref().unwrap().load(&self.object_type, None);
        assert!(tagger.add_to_favorites(1));
        assert!(tagger.remove_from_favorites(1));
    }
}

impl TestCase for TestTags {
    fn set_up(&mut self) {
        User::clear_backends();
        User::use_backend("dummy");
        
        // Generate unique IDs for testing
        self.user = format!("user_{}", Uuid::new_v4().to_string());
        self.object_type = format!("type_{}", Uuid::new_v4().to_string());
        
        User::create_user(&self.user, "pass");
        let user = User::set_user_id(&self.user);
        
        self.tag_mgr = Some(TagManager::new(&self.user));
    }
    
    fn tear_down(&mut self) {
        // In a real implementation, this would clean up test data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_with_defaults() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_instantiate_with_defaults();
        test.tear_down();
    }
    
    #[test]
    fn test_add_tags() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_add_tags();
        test.tear_down();
    }
    
    #[test]
    fn test_add_multiple() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_add_multiple();
        test.tear_down();
    }
    
    #[test]
    fn test_is_empty() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_is_empty();
        test.tear_down();
    }
    
    #[test]
    fn test_delete_tags() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_delete_tags();
        test.tear_down();
    }
    
    #[test]
    fn test_rename_tag() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_rename_tag();
        test.tear_down();
    }
    
    #[test]
    fn test_tag_as() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_tag_as();
        test.tear_down();
    }
    
    #[test]
    fn test_untag() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_untag();
        test.tear_down();
    }
    
    #[test]
    fn test_favorite() {
        let mut test = TestTags::new();
        test.set_up();
        test.test_favorite();
        test.tear_down();
    }
}