/**
 * @author Robin Appelman
 * @copyright 2013 Robin Appelman icewind@owncloud.com
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
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SyntaxException {
    message: String,
}

impl SyntaxException {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SyntaxException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {}", self.message)
    }
}

impl Error for SyntaxException {}

pub struct ArrayParser;

#[derive(Debug, PartialEq)]
enum ValueType {
    Num,
    Bool,
    String,
    Array,
}

impl ArrayParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_php(&self, string: &str) -> Result<Value, SyntaxException> {
        let string = self.strip_php_tags(string);
        let string = self.strip_assign_and_return(&string);
        self.parse(&string)
    }

    fn strip_php_tags(&self, string: &str) -> String {
        let mut result = string.trim().to_string();
        if result.starts_with("<?php") {
            result = result[5..].to_string();
        }
        if result.ends_with("?>") {
            result = result[..result.len() - 2].to_string();
        }
        result
    }

    fn strip_assign_and_return(&self, string: &str) -> String {
        let mut result = string.trim().to_string();
        if result.starts_with("return") {
            result = result[6..].to_string();
        }
        if result.starts_with('$') {
            let parts: Vec<&str> = result.splitn(2, '=').collect();
            if parts.len() > 1 {
                result = parts[1].to_string();
            }
        }
        result
    }

    fn parse(&self, string: &str) -> Result<Value, SyntaxException> {
        let string = string.trim();
        let string = string.trim_end_matches(';');
        
        match self.get_type(string) {
            ValueType::Num => Ok(Value::Num(self.parse_num(string))),
            ValueType::Bool => Ok(Value::Bool(self.parse_bool(string))),
            ValueType::String => Ok(Value::String(self.parse_string(string))),
            ValueType::Array => Ok(Value::Array(self.parse_array(string)?)),
        }
    }

    fn get_type(&self, string: &str) -> ValueType {
        let string = string.to_lowercase();
        let first = string.chars().next();
        let last = string.chars().last();
        
        let array_first = if string.len() >= 5 {
            Some(&string[0..5])
        } else {
            None
        };
        
        if (first == Some('"') || first == Some('\'')) && (last == Some('"') || last == Some('\'')) {
            ValueType::String
        } else if string == "false" || string == "true" {
            ValueType::Bool
        } else if array_first == Some("array") && last == Some(')') {
            ValueType::Array
        } else {
            ValueType::Num
        }
    }

    fn parse_string(&self, string: &str) -> String {
        string[1..string.len() - 1].to_string()
    }

    fn parse_num(&self, string: &str) -> i64 {
        string.parse::<i64>().unwrap_or(0)
    }

    fn parse_bool(&self, string: &str) -> bool {
        string.to_lowercase() == "true"
    }

    fn parse_array(&self, string: &str) -> Result<HashMap<String, Value>, SyntaxException> {
        let body = &string[5..];
        let body = body.trim();
        let body = &body[1..body.len() - 1];
        
        let items = self.split_array(body)?;
        let mut result = HashMap::new();
        let mut last_key: i64 = -1;
        
        for item in items {
            let item = item.trim();
            if !item.is_empty() {
                if item.contains("=>") {
                    let parts: Vec<&str> = item.splitn(2, "=>").collect();
                    let key = self.parse(parts[0].trim())?;
                    let value = self.parse(parts[1].trim())?;
                    
                    match key {
                        Value::Num(key_num) => {
                            last_key = key_num;
                            result.insert(key_num.to_string(), value);
                        },
                        Value::String(key_str) => {
                            result.insert(key_str, value);
                        },
                        _ => return Err(SyntaxException::new("Invalid array key type")),
                    }
                } else {
                    last_key += 1;
                    let value = self.parse(item)?;
                    result.insert(last_key.to_string(), value);
                }
            }
        }
        
        Ok(result)
    }

    fn split_array(&self, body: &str) -> Result<Vec<String>, SyntaxException> {
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut bracket_depth = 0;
        let mut parts = Vec::new();
        let mut start = 0;
        let mut escaped = false;
        let mut skips = Vec::new();
        
        let body_with_comma = if !body.ends_with(',') {
            format!("{},", body)
        } else {
            body.to_string()
        };
        
        let chars: Vec<char> = body_with_comma.chars().collect();
        
        for i in 0..chars.len() {
            let char = chars[i];
            
            if char == '\\' {
                if escaped {
                    skips.push(i - 1);
                }
                escaped = !escaped;
            } else {
                if char == '"' && !in_single_quote {
                    if escaped {
                        skips.push(i - 1);
                    } else {
                        in_double_quote = !in_double_quote;
                    }
                } else if char == '\'' && !in_double_quote {
                    if escaped {
                        skips.push(i - 1);
                    } else {
                        in_single_quote = !in_single_quote;
                    }
                } else if !in_double_quote && !in_single_quote {
                    if char == '(' {
                        bracket_depth += 1;
                    } else if char == ')' {
                        if bracket_depth <= 0 {
                            return Err(SyntaxException::new("Unbalanced brackets"));
                        } else {
                            bracket_depth -= 1;
                        }
                    } else if bracket_depth == 0 && char == ',' {
                        let part = body_with_comma[start..i].to_string();
                        
                        // Apply skips to remove escape characters
                        let mut part_chars: Vec<char> = part.chars().collect();
                        for &skip in skips.iter().rev() {
                            if skip - start < part_chars.len() {
                                part_chars.remove(skip - start);
                            }
                        }
                        
                        parts.push(part_chars.into_iter().collect());
                        start = i + 1;
                        skips.clear();
                    }
                }
                escaped = false;
            }
        }
        
        Ok(parts)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(i64),
    Bool(bool),
    String(String),
    Array(HashMap<String, Value>),
}