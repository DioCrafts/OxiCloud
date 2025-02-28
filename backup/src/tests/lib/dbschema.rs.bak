extern crate tempfile;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

use crate::OC;
use crate::OC_Config;
use crate::OC_DB;
use crate::OC_Util;

/**
 * Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

struct TestDBSchema {
    schema_file: String,
    schema_file2: String,
    table1: String,
    table2: String,
}

impl TestDBSchema {
    fn new() -> Self {
        TestDBSchema {
            schema_file: "static://test_db_scheme".to_string(),
            schema_file2: "static://test_db_scheme2".to_string(),
            table1: String::new(),
            table2: String::new(),
        }
    }

    fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let dbfile = PathBuf::from(&OC::server_root()).join("tests/data/db_structure.xml");
        let dbfile2 = PathBuf::from(&OC::server_root()).join("tests/data/db_structure2.xml");

        let r = format!("_{}_", OC_Util::generate_random_bytes(4));
        
        let content = fs::read_to_string(&dbfile)?;
        let content = content.replace("*dbprefix*", &format!("*dbprefix*{}", r));
        fs::write(&self.schema_file, content)?;
        
        let content = fs::read_to_string(&dbfile2)?;
        let content = content.replace("*dbprefix*", &format!("*dbprefix*{}", r));
        fs::write(&self.schema_file2, content)?;

        let prefix = OC_Config::get_value("dbtableprefix", "oc_");
        
        self.table1 = format!("{}{}{}", prefix, r, "cntcts_addrsbks");
        self.table2 = format!("{}{}{}", prefix, r, "cntcts_cards");
        
        Ok(())
    }

    fn tear_down(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::remove_file(&self.schema_file)?;
        fs::remove_file(&self.schema_file2)?;
        Ok(())
    }

    // everything in one test, they depend on each other
    #[test]
    fn test_schema(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.do_test_schema_creating()?;
        self.do_test_schema_changing()?;
        self.do_test_schema_dumping()?;
        self.do_test_schema_removing()?;
        Ok(())
    }

    fn do_test_schema_creating(&self) -> Result<(), Box<dyn std::error::Error>> {
        OC_DB::create_db_from_structure(&self.schema_file)?;
        self.assert_table_exist(&self.table1)?;
        self.assert_table_exist(&self.table2)?;
        Ok(())
    }

    fn do_test_schema_changing(&self) -> Result<(), Box<dyn std::error::Error>> {
        OC_DB::update_db_from_structure(&self.schema_file2)?;
        self.assert_table_exist(&self.table2)?;
        Ok(())
    }

    fn do_test_schema_dumping(&self) -> Result<(), Box<dyn std::error::Error>> {
        let outfile = "static://db_out.xml";
        OC_DB::get_db_structure(outfile)?;
        let content = fs::read_to_string(outfile)?;
        assert!(content.contains(&self.table1), "Dumped schema does not contain table {}", self.table1);
        assert!(content.contains(&self.table2), "Dumped schema does not contain table {}", self.table2);
        Ok(())
    }

    fn do_test_schema_removing(&self) -> Result<(), Box<dyn std::error::Error>> {
        OC_DB::remove_db_structure(&self.schema_file)?;
        self.assert_table_not_exist(&self.table1)?;
        self.assert_table_not_exist(&self.table2)?;
        Ok(())
    }

    fn table_exist(&self, table: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let db_type = OC_Config::get_value("dbtype", "sqlite");
        
        let (sql, params) = match db_type.as_str() {
            "sqlite" | "sqlite3" => {
                let sql = "SELECT name FROM sqlite_master \
                          WHERE type = 'table' AND name = ? \
                          UNION ALL SELECT name FROM sqlite_temp_master \
                          WHERE type = 'table' AND name = ?";
                (sql, vec![table.to_string(), table.to_string()])
            },
            "mysql" => {
                let sql = "SHOW TABLES LIKE ?";
                (sql, vec![table.to_string()])
            },
            "pgsql" => {
                let sql = "SELECT tablename AS table_name, schemaname AS schema_name \
                          FROM pg_tables WHERE schemaname NOT LIKE 'pg_%' \
                          AND schemaname != 'information_schema' \
                          AND tablename = ?";
                (sql, vec![table.to_string()])
            },
            "oci" => {
                let sql = "SELECT TABLE_NAME FROM USER_TABLES WHERE TABLE_NAME = ?";
                (sql, vec![table.to_string()])
            },
            "mssql" => {
                let sql = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = ?";
                (sql, vec![table.to_string()])
            },
            _ => return Err("Unknown database type".into()),
        };
        
        let result = OC_DB::execute_audited(sql, params)?;
        
        let name = result.fetch_one()?;
        Ok(name == table)
    }

    fn assert_table_exist(&self, table: &str) -> Result<(), Box<dyn std::error::Error>> {
        assert!(self.table_exist(table)?, "Table {} does not exist", table);
        Ok(())
    }

    fn assert_table_not_exist(&self, table: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db_type = OC_Config::get_value("dbtype", "sqlite");
        if db_type == "sqlite" || db_type == "sqlite3" {
            // sqlite removes the tables after closing the DB
            return Ok(());
        } else {
            assert!(!self.table_exist(table)?, "Table {} exists", table);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema() {
        let mut test = TestDBSchema::new();
        test.set_up().unwrap();

        // Execute the test and handle any errors
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            test.test_schema().unwrap();
        }));

        // Always run teardown even if the test failed
        let teardown_result = test.tear_down();
        
        // Now propagate any test errors
        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
        
        // Finally check if teardown succeeded
        teardown_result.unwrap();
    }
}