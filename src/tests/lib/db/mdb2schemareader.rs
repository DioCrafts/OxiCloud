// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

//! MDB2SchemaReader test module

use async_trait::async_trait;
use doctrine_dbal::{
    platforms::MySqlPlatform,
    types::{BooleanType, IntegerType, StringType, TextType},
};
use mockall::{mock, predicate::*};
use std::path::Path;

use crate::db::MDB2SchemaReader;
use crate::lib::config::Config;

mock! {
    pub Config {}

    #[async_trait]
    impl Config for Config {
        async fn get_value<T: serde::de::DeserializeOwned + Send>(&self, key: &str, default: T) -> T;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestCase;

    struct MDB2SchemaReaderTest {
        reader: MDB2SchemaReader,
    }

    impl TestCase for MDB2SchemaReaderTest {
        fn set_up(&mut self) {
            // Implementation not needed for this test
        }

        fn tear_down(&mut self) {
            // Implementation not needed for this test
        }
    }

    impl MDB2SchemaReaderTest {
        fn new() -> Self {
            Self {
                reader: MDB2SchemaReader::new(),
            }
        }

        fn get_config(&self) -> MockConfig {
            let mut config = MockConfig::new();
            
            config.expect_get_value::<String>()
                .with(eq("dbname"), eq("owncloud".to_string()))
                .returning(|_, _| "testDB".to_string());
            
            config.expect_get_value::<String>()
                .with(eq("dbtableprefix"), eq("oc_".to_string()))
                .returning(|_, _| "test_".to_string());
            
            config
        }

        #[tokio::test]
        async fn test_read() {
            let config = self.get_config();
            let platform = MySqlPlatform::new();
            let reader = MDB2SchemaReader::new(config, platform);
            
            let schema = reader.load_schema_from_file(Path::new("tests/db/testschema.xml")).await.unwrap();
            assert_eq!(1, schema.get_tables().len());

            let table = schema.get_table("test_table").unwrap();
            assert_eq!(7, table.get_columns().len());

            let integer_field = table.get_column("integerfield").unwrap();
            assert_eq!(4, integer_field.get_length());
            assert!(integer_field.get_autoincrement());
            assert!(integer_field.get_default().is_none());
            assert!(integer_field.get_not_null());
            assert!(integer_field.get_type().is::<IntegerType>());

            let integer_field_default = table.get_column("integerfield_default").unwrap();
            assert_eq!(Some(10), integer_field_default.get_default().and_then(|v| v.as_i64()));

            let text_field = table.get_column("textfield").unwrap();
            assert_eq!(32, text_field.get_length());
            assert!(!text_field.get_autoincrement());
            assert_eq!(Some("foo".to_string()), text_field.get_default().and_then(|v| v.as_str().map(String::from)));
            assert!(text_field.get_not_null());
            assert!(text_field.get_type().is::<StringType>());

            let clob_field = table.get_column("clobfield").unwrap();
            assert_eq!(None, clob_field.get_length());
            assert!(!clob_field.get_autoincrement());
            assert_eq!(Some("".to_string()), clob_field.get_default().and_then(|v| v.as_str().map(String::from)));
            assert!(clob_field.get_not_null());
            assert!(clob_field.get_type().is::<TextType>());

            let boolean_field = table.get_column("booleanfield").unwrap();
            assert_eq!(None, boolean_field.get_length());
            assert!(!boolean_field.get_autoincrement());
            assert_eq!(Some(false), boolean_field.get_default().and_then(|v| v.as_bool()));
            assert!(boolean_field.get_type().is::<BooleanType>());

            let boolean_field_true = table.get_column("booleanfield_true").unwrap();
            assert_eq!(Some(true), boolean_field_true.get_default().and_then(|v| v.as_bool()));

            let boolean_field_false = table.get_column("booleanfield_false").unwrap();
            assert_eq!(Some(false), boolean_field_false.get_default().and_then(|v| v.as_bool()));

            let indexes = table.get_indexes();
            assert_eq!(2, indexes.len());
            
            let primary_index = table.get_index("primary").unwrap();
            assert_eq!(vec!["integerfield"], primary_index.get_unquoted_columns());
            assert!(primary_index.is_primary());
            assert!(primary_index.is_unique());
            
            let boolean_index = table.get_index("index_boolean").unwrap();
            assert_eq!(vec!["booleanfield"], boolean_index.get_unquoted_columns());
            assert!(!boolean_index.is_primary());
            assert!(!boolean_index.is_unique());
        }
    }
}