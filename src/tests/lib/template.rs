/// ownCloud
///
/// @author Bernhard Posselt
/// @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
///
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use html_escape::encode_safe;
use chrono::{Duration, TimeZone, Utc};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    
    // Mock functions to simulate PHP's output buffering
    struct OutputBuffer {
        buffer: String,
    }
    
    impl OutputBuffer {
        fn new() -> Self {
            OutputBuffer { buffer: String::new() }
        }
        
        fn get_clean(&mut self) -> String {
            let result = self.buffer.clone();
            self.buffer.clear();
            result
        }
    }
    
    // Global output buffer for tests
    thread_local! {
        static OUTPUT_BUFFER: std::cell::RefCell<OutputBuffer> = 
            std::cell::RefCell::new(OutputBuffer::new());
    }
    
    // Mock functions for template system
    fn p(s: &str) -> io::Result<()> {
        let html = encode_safe(s).to_string();
        OUTPUT_BUFFER.with(|buffer| {
            buffer.borrow_mut().buffer.push_str(&html);
        });
        Ok(())
    }
    
    fn print_unescaped(s: &str) -> io::Result<()> {
        OUTPUT_BUFFER.with(|buffer| {
            buffer.borrow_mut().buffer.push_str(s);
        });
        Ok(())
    }
    
    fn relative_modified_date(element_time: i64, current_time: i64, date_only: bool) -> String {
        let diff = current_time - element_time;
        
        if date_only {
            // Only show the date
            if diff < 24 * 3600 {
                return "today".to_string();
            } else if diff < 48 * 3600 {
                return "yesterday".to_string();
            } else if diff < 30 * 86400 {
                let days = diff / 86400;
                return format!("{} days ago", days);
            } else if diff < 60 * 86400 {
                return "last month".to_string();
            } else if diff < 365 * 86400 {
                let months = diff / (30 * 86400);
                return format!("{} months ago", months);
            } else if diff < 730 * 86400 {
                return "last year".to_string();
            } else {
                return "years ago".to_string();
            }
        } else {
            // Show date and time
            if diff < 60 {
                return "seconds ago".to_string();
            } else if diff < 3600 {
                let minutes = diff / 60;
                return format!("{} minutes ago", minutes);
            } else if diff < 24 * 3600 {
                let hours = diff / 3600;
                return format!("{} hours ago", hours);
            } else if diff < 30 * 86400 {
                let days = diff / 86400;
                return format!("{} days ago", days);
            } else if diff < 60 * 86400 {
                return "last month".to_string();
            } else if diff < 365 * 86400 {
                let months = diff / (30 * 86400);
                return format!("{} months ago", months);
            } else if diff < 730 * 86400 {
                return "last year".to_string();
            } else {
                return "years ago".to_string();
            }
        }
    }
    
    struct TestTemplateFunctions {}
    
    impl TestTemplateFunctions {
        fn setup() {
            // This would be equivalent to loading OC_Template in PHP
            // For Rust, we assume the required dependencies are already imported
        }
        
        #[test]
        fn test_p() {
            Self::setup();
            
            // FIXME: do we need more testcases?
            let html_string = "<script>alert('xss');</script>";
            p(html_string).unwrap();
            let result = OUTPUT_BUFFER.with(|buffer| buffer.borrow_mut().get_clean());
            
            assert_eq!("&lt;script&gt;alert(&#39;xss&#39;);&lt;/script&gt;", result);
        }
        
        #[test]
        fn test_p_normal_string() {
            Self::setup();
            
            let normal_string = "This is a good string!";
            p(normal_string).unwrap();
            let result = OUTPUT_BUFFER.with(|buffer| buffer.borrow_mut().get_clean());
            
            assert_eq!("This is a good string!", result);
        }
        
        #[test]
        fn test_print_unescaped() {
            Self::setup();
            
            let html_string = "<script>alert('xss');</script>";
            print_unescaped(html_string).unwrap();
            let result = OUTPUT_BUFFER.with(|buffer| buffer.borrow_mut().get_clean());
            
            assert_eq!(html_string, result);
        }
        
        #[test]
        fn test_print_unescaped_normal_string() {
            Self::setup();
            
            let normal_string = "This is a good string!";
            print_unescaped(normal_string).unwrap();
            let result = OUTPUT_BUFFER.with(|buffer| buffer.borrow_mut().get_clean());
            
            assert_eq!("This is a good string!", result);
        }
        
        // ---------------------------------------------------------------------------
        // Test relative_modified_date with dates only
        // ---------------------------------------------------------------------------
        #[test]
        fn test_relative_date_today() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("today", result);
            
            // 2 hours ago is still today
            let element_time = current_time - 2 * 3600;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("today", result);
        }
        
        #[test]
        fn test_relative_date_yesterday() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 24 * 3600;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("yesterday", result);
            
            // yesterday - 2 hours is still yesterday
            let element_time = current_time - 26 * 3600;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("yesterday", result);
        }
        
        #[test]
        fn test_relative_date_2_days_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 48 * 3600;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("2 days ago", result);
            
            // 2 days ago minus 4 hours is still 2 days ago
            let element_time = current_time - 52 * 3600;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("2 days ago", result);
        }
        
        #[test]
        fn test_relative_date_last_month() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 31;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("last month", result);
            
            let element_time = current_time - 86400 * 35;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("last month", result);
        }
        
        #[test]
        fn test_relative_date_months_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 60;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("2 months ago", result);
            
            let element_time = current_time - 86400 * 65;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("2 months ago", result);
        }
        
        #[test]
        fn test_relative_date_last_year() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 365;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("last year", result);
            
            let element_time = current_time - 86400 * 450;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("last year", result);
        }
        
        #[test]
        fn test_relative_date_years_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - (86400 as f64 * 365.25 * 2.0) as i64;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("years ago", result);
            
            let element_time = current_time - (86400 as f64 * 365.25 * 3.0) as i64;
            let result = relative_modified_date(element_time, current_time, true);
            
            assert_eq!("years ago", result);
        }
        
        // ---------------------------------------------------------------------------
        // Test relative_modified_date with timestamps only (date + time value)
        // ---------------------------------------------------------------------------
        
        #[test]
        fn test_relative_time_seconds_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 5;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("seconds ago", result);
        }
        
        #[test]
        fn test_relative_time_minutes_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 190;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("3 minutes ago", result);
        }
        
        #[test]
        fn test_relative_time_hours_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 7500;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("2 hours ago", result);
        }
        
        #[test]
        fn test_relative_time_2_days_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 48 * 3600;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("2 days ago", result);
            
            // 2 days ago minus 4 hours is still 2 days ago
            let element_time = current_time - 52 * 3600;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("2 days ago", result);
        }
        
        #[test]
        fn test_relative_time_last_month() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 31;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("last month", result);
            
            let element_time = current_time - 86400 * 35;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("last month", result);
        }
        
        #[test]
        fn test_relative_time_months_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 60;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("2 months ago", result);
            
            let element_time = current_time - 86400 * 65;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("2 months ago", result);
        }
        
        #[test]
        fn test_relative_time_last_year() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - 86400 * 365;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("last year", result);
            
            let element_time = current_time - 86400 * 450;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("last year", result);
        }
        
        #[test]
        fn test_relative_time_years_ago() {
            Self::setup();
            
            let current_time = 1380703592;
            let element_time = current_time - (86400 as f64 * 365.25 * 2.0) as i64;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("years ago", result);
            
            let element_time = current_time - (86400 as f64 * 365.25 * 3.0) as i64;
            let result = relative_modified_date(element_time, current_time, false);
            
            assert_eq!("years ago", result);
        }
    }
}