// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use regex::Regex;
use lazy_static::lazy_static;

/// Adapter for SQL Server databases
pub struct AdapterSqlSrv;

impl super::Adapter for AdapterSqlSrv {
    fn fixup_statement(&self, statement: &str) -> String {
        lazy_static! {
            static ref BACKTICK_RE: Regex = Regex::new(r"`(.*?)`").unwrap();
        }

        let mut result = BACKTICK_RE.replace_all(statement, |caps: &regex::Captures| {
            format!("[{}]", &caps[1])
        }).to_string();

        // Case-insensitive replacements
        result = result.replace("NOW()", "CURRENT_TIMESTAMP");
        result = result.replace("now()", "CURRENT_TIMESTAMP");

        // Case-sensitive replacements
        result = result.replace("LENGTH(", "LEN(");
        result = result.replace("SUBSTR(", "SUBSTRING(");
        
        // Case-insensitive replacements
        result = result.replace("UNIX_TIMESTAMP()", "DATEDIFF(second,{d '1970-01-01'},GETDATE())");
        result = result.replace("unix_timestamp()", "DATEDIFF(second,{d '1970-01-01'},GETDATE())");

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Adapter;

    #[test]
    fn test_fixup_statement() {
        let adapter = AdapterSqlSrv;
        
        assert_eq!(
            adapter.fixup_statement("`test` = 1"),
            "[test] = 1"
        );
        
        assert_eq!(
            adapter.fixup_statement("SELECT NOW()"),
            "SELECT CURRENT_TIMESTAMP"
        );
        
        assert_eq!(
            adapter.fixup_statement("LENGTH(col)"),
            "LEN(col)"
        );
        
        assert_eq!(
            adapter.fixup_statement("SUBSTR(col, 1, 2)"),
            "SUBSTRING(col, 1, 2)"
        );
        
        assert_eq!(
            adapter.fixup_statement("UNIX_TIMESTAMP()"),
            "DATEDIFF(second,{d '1970-01-01'},GETDATE())"
        );
    }
}