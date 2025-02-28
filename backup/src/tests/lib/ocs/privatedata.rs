/*
 * ownCloud
 *
 * @author Thomas Müller
 * @copyright 2013 Thomas Müller deepdiver@owncloud.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::collections::HashMap;
use uuid::Uuid;
use async_trait::async_trait;

struct OC {
    session: Session,
}

struct Session {
    data: HashMap<String, String>,
}

impl Session {
    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

struct OcsResult {
    status_code: u32,
    data: Vec<HashMap<String, String>>,
}

impl OcsResult {
    fn new(status_code: u32, data: Vec<HashMap<String, String>>) -> Self {
        Self { status_code, data }
    }

    fn get_status_code(&self) -> u32 {
        self.status_code
    }

    fn get_data(&self) -> &Vec<HashMap<String, String>> {
        &self.data
    }
}

struct OcOcsPrivatedata;

impl OcOcsPrivatedata {
    fn get(params: &HashMap<String, String>) -> OcsResult {
        // Simulación de la implementación de get
        if params.contains_key("app") {
            if params.contains_key("key") {
                // Comprobación específica de app y key
                return OcsResult::new(100, vec![]);
            }
            // Comprobación de todas las entradas para una app
            return OcsResult::new(100, vec![]);
        }
        OcsResult::new(100, vec![])
    }

    fn set(params: &HashMap<String, String>) -> OcsResult {
        // Simulación de la implementación de set
        // En un escenario real, aquí guardaríamos los datos en una base de datos
        OcsResult::new(100, vec![])
    }

    fn delete(params: &HashMap<String, String>) -> OcsResult {
        // Validar que los parámetros necesarios están presentes
        if !params.contains_key("app") || !params.contains_key("key") {
            return OcsResult::new(101, vec![]);
        }
        
        // Simulación de la implementación de delete
        OcsResult::new(100, vec![])
    }
}

#[async_trait]
trait TestCase {
    async fn set_up(&mut self);
    async fn tear_down(&mut self);
}

struct TestOcOcsPrivatedata {
    app_key: String,
    oc: OC,
    post: HashMap<String, String>,
}

#[async_trait]
impl TestCase for TestOcOcsPrivatedata {
    async fn set_up(&mut self) {
        self.oc.session.set("user_id", "user1");
        self.app_key = Uuid::new_v4().to_string();
    }

    async fn tear_down(&mut self) {
        // Cleanup if needed
    }
}

impl TestOcOcsPrivatedata {
    fn new() -> Self {
        Self {
            app_key: String::new(),
            oc: OC {
                session: Session {
                    data: HashMap::new(),
                },
            },
            post: HashMap::new(),
        }
    }

    async fn test_get_empty_one(&mut self) {
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        params.insert("key".to_string(), "123".to_string());
        
        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(0, &result);
    }

    async fn test_get_empty_all(&mut self) {
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        
        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(0, &result);
    }

    async fn test_set_one(&mut self) {
        self.post.insert("value".to_string(), "123456789".to_string());
        
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        params.insert("key".to_string(), "k-1".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(1, &result);
    }

    async fn test_set_existing(&mut self) {
        self.post.insert("value".to_string(), "123456789".to_string());
        
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        params.insert("key".to_string(), "k-10".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(1, &result);
        
        let data = result.get_data();
        let data = &data[0];
        assert_eq!("123456789", data.get("value").unwrap());

        self.post.insert("value".to_string(), "updated".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(1, &result);
        
        let data = result.get_data();
        let data = &data[0];
        assert_eq!("updated", data.get("value").unwrap());
    }

    async fn test_set_many(&mut self) {
        self.post.insert("value".to_string(), "123456789".to_string());
        
        // set key 'k-1'
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        params.insert("key".to_string(), "k-1".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        // set key 'k-2'
        params.insert("key".to_string(), "k-2".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        // query for all
        params.remove("key");
        
        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(2, &result);
    }

    async fn test_delete(&mut self) {
        self.post.insert("value".to_string(), "123456789".to_string());
        
        // set key 'k-3'
        let mut params = HashMap::new();
        params.insert("app".to_string(), self.app_key.clone());
        params.insert("key".to_string(), "k-3".to_string());
        
        let result = OcOcsPrivatedata::set(&params);
        assert_eq!(100, result.get_status_code());

        let result = OcOcsPrivatedata::delete(&params);
        assert_eq!(100, result.get_status_code());

        let result = OcOcsPrivatedata::get(&params);
        self.assert_ocs_result(0, &result);
    }

    async fn test_delete_with_empty_keys(&mut self, params: HashMap<String, String>) {
        let result = OcOcsPrivatedata::delete(&params);
        assert_eq!(101, result.get_status_code());
    }

    fn get_delete_with_empty_keys_test_cases() -> Vec<HashMap<String, String>> {
        vec![
            HashMap::new(),
            {
                let mut params = HashMap::new();
                params.insert("app".to_string(), "123".to_string());
                params
            },
            {
                let mut params = HashMap::new();
                params.insert("key".to_string(), "123".to_string());
                params
            },
        ]
    }

    fn assert_ocs_result(&self, expected_array_size: usize, result: &OcsResult) {
        assert_eq!(100, result.get_status_code());
        let data = result.get_data();
        assert_eq!(expected_array_size, data.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_empty_one() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_get_empty_one().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_get_empty_all() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_get_empty_all().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_set_one() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_set_one().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_set_existing() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_set_existing().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_set_many() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_set_many().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_delete() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        test.test_delete().await;
        test.tear_down().await;
    }

    #[tokio::test]
    async fn test_delete_with_empty_keys() {
        let mut test = TestOcOcsPrivatedata::new();
        test.set_up().await;
        
        for params in TestOcOcsPrivatedata::get_delete_with_empty_keys_test_cases() {
            test.test_delete_with_empty_keys(params).await;
        }
        
        test.tear_down().await;
    }
}