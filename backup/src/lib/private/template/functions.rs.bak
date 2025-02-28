use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};
use std::collections::HashMap;

/// Prints an XSS escaped string
/// 
/// # Arguments
/// * `string` - the string which will be escaped and printed
pub fn p(string: &str) {
    print!("{}", sanitize_html(string));
}

/// Prints an unescaped string
/// 
/// # Arguments
/// * `string` - the string which will be printed as it is
pub fn print_unescaped(string: &str) {
    print!("{}", string);
}

/// Make helper::link_to available as a simple function
/// 
/// # Arguments
/// * `app` - app
/// * `file` - file
/// * `args` - HashMap with param=>value, will be appended to the returned url
/// 
/// # Returns
/// link to the file
///
/// For further information have a look at helper::link_to
pub fn link_to(app: &str, file: &str, args: HashMap<String, String>) -> String {
    helper::link_to(app, file, args)
}

/// Get url to the online documentation
///
/// # Arguments
/// * `key` - documentation key
///
/// # Returns
/// url to the online documentation
pub fn link_to_docs(key: &str) -> String {
    helper::link_to_docs(key)
}

/// Make helper::image_path available as a simple function
/// 
/// # Arguments
/// * `app` - app
/// * `image` - image
/// 
/// # Returns
/// link to the image
///
/// For further information have a look at helper::image_path
pub fn image_path(app: &str, image: &str) -> String {
    helper::image_path(app, image)
}

/// Make helper::mimetype_icon available as a simple function
/// 
/// # Arguments
/// * `mimetype` - mimetype
/// 
/// # Returns
/// link to the image
///
/// For further information have a look at helper::mimetype_icon
pub fn mimetype_icon(mimetype: &str) -> String {
    helper::mimetype_icon(mimetype)
}

/// Make preview_icon available as a simple function.
/// Returns the path to the preview of the image.
/// 
/// # Arguments
/// * `path` - path of file
/// 
/// # Returns
/// link to the preview
///
/// For further information have a look at helper::preview_icon
pub fn preview_icon(path: &str) -> String {
    helper::preview_icon(path)
}

pub fn public_preview_icon(path: &str, token: &str) -> String {
    helper::public_preview_icon(path, token)
}

/// Make helper::human_file_size available as a simple function
/// 
/// # Arguments
/// * `bytes` - size in bytes
/// 
/// # Returns
/// size as string
///
/// For further information have a look at helper::human_file_size
pub fn human_file_size(bytes: i64) -> String {
    helper::human_file_size(bytes)
}

/// Strips the timestamp of its time value
/// 
/// # Arguments
/// * `timestamp` - UNIX timestamp to strip
/// 
/// # Returns
/// timestamp without time value
pub fn strip_time(timestamp: i64) -> i64 {
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default(),
        Utc,
    );
    let date_only = Utc.ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0);
    date_only.timestamp()
}

/// Formats timestamp relatively to the current time using
/// a human-friendly format like "x minutes ago" or "yesterday"
/// 
/// # Arguments
/// * `timestamp` - timestamp to format
/// * `from_time` - timestamp to compare from, defaults to current time
/// * `date_only` - whether to strip time information
/// 
/// # Returns
/// formatted timestamp
pub fn relative_modified_date(timestamp: i64, from_time: Option<i64>, date_only: bool) -> String {
    let l10n = l10n::get("lib");
    let from_time = from_time.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs() as i64
    });
    
    let (from_time, timestamp) = if date_only {
        (strip_time(from_time), strip_time(timestamp))
    } else {
        (from_time, timestamp)
    };
    
    let time_diff = from_time - timestamp;
    let diff_minutes = (time_diff / 60) as i32;
    let diff_hours = (diff_minutes / 60) as i32;
    let diff_days = (diff_hours / 24) as i32;
    let diff_months = (diff_days / 31) as i32;

    if !date_only && time_diff < 60 {
        l10n.t("seconds ago")
    } else if !date_only && time_diff < 3600 {
        l10n.n("%n minute ago", "%n minutes ago", diff_minutes)
    } else if !date_only && time_diff < 86400 {
        l10n.n("%n hour ago", "%n hours ago", diff_hours)
    } else {
        let from_dt = Local.timestamp(from_time, 0);
        
        if (from_dt.hour() as i32 - diff_hours) >= 0 {
            l10n.t("today")
        } else if (from_dt.hour() as i32 - diff_hours) >= -24 {
            l10n.t("yesterday")
        // 86400 * 31 days = 2678400
        } else if time_diff < 2678400 {
            l10n.n("%n day ago", "%n days ago", diff_days)
        // 86400 * 60 days = 5184000
        } else if time_diff < 5184000 {
            l10n.t("last month")
        } else if (from_dt.month() as i32 - diff_months) > 0 {
            l10n.n("%n month ago", "%n months ago", diff_months)
        // 86400 * 365.25 days * 2 = 63113852
        } else if time_diff < 63113852 {
            l10n.t("last year")
        } else {
            l10n.t("years ago")
        }
    }
}

pub fn html_select_options<T: AsRef<str> + std::fmt::Display>(
    options: &[T], 
    selected: &[T], 
    params: HashMap<String, bool>
) -> String {
    let combine = params.get("combine").cloned().unwrap_or(false);
    
    let mut html = String::new();
    
    if combine {
        for option in options {
            let value = option.as_ref();
            let label = option.as_ref();
            let is_selected = selected.iter().any(|s| s.as_ref() == value);
            
            html.push_str(&format!(
                "<option value=\"{}\"{}>{}</option>\n",
                sanitize_html(value),
                if is_selected { " selected=\"selected\"" } else { "" },
                sanitize_html(label)
            ));
        }
    } else {
        for (i, option) in options.iter().enumerate() {
            let value = if i < options.len() { i.to_string() } else { option.to_string() };
            let label = option.as_ref();
            let is_selected = selected.iter().any(|s| s.as_ref() == value);
            
            html.push_str(&format!(
                "<option value=\"{}\"{}>{}</option>\n",
                sanitize_html(&value),
                if is_selected { " selected=\"selected\"" } else { "" },
                sanitize_html(label)
            ));
        }
    }
    
    html
}

// Mock implementations for external dependencies
mod helper {
    use std::collections::HashMap;
    
    pub fn link_to(app: &str, file: &str, args: HashMap<String, String>) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn link_to_docs(key: &str) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn image_path(app: &str, image: &str) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn mimetype_icon(mimetype: &str) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn preview_icon(path: &str) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn public_preview_icon(path: &str, token: &str) -> String {
        // Implementation would go here
        String::new()
    }
    
    pub fn human_file_size(bytes: i64) -> String {
        // Implementation would go here
        String::new()
    }
}

mod l10n {
    pub struct L10n;
    
    impl L10n {
        pub fn t(&self, text: &str) -> String {
            // Translation implementation would go here
            text.to_string()
        }
        
        pub fn n(&self, singular: &str, plural: &str, count: i32) -> String {
            // Translation implementation would go here
            if count == 1 {
                singular.replace("%n", &count.to_string())
            } else {
                plural.replace("%n", &count.to_string())
            }
        }
    }
    
    pub fn get(domain: &str) -> L10n {
        // Implementation would go here
        L10n
    }
}

fn sanitize_html(input: &str) -> String {
    // Implementation would be replaced with appropriate HTML sanitization
    // This is just a placeholder
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;")
}