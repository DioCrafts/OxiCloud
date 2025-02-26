use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use maud::{html, Markup, PreEscaped};
use std::time::{SystemTime, UNIX_EPOCH};

struct File {
    name: String,
    directory: String,
    basename: String,
    extension: String,
    timestamp: i64,
    date: i64,
    r#type: String,
    mimetype: String,
    permissions: String,
    is_preview_available: bool,
}

struct Context {
    files: Vec<File>,
    dirlisting: bool,
    base_url: String,
    download_url: String,
    readonly: Option<bool>,
}

fn relative_modified_date(timestamp: i64) -> String {
    // Esta función simula la versión PHP de relative_modified_date
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let seconds_ago = now - timestamp;
    
    if seconds_ago < 60 {
        return format!("{} seconds ago", seconds_ago);
    } else if seconds_ago < 3600 {
        return format!("{} minutes ago", seconds_ago / 60);
    } else if seconds_ago < 86400 {
        return format!("{} hours ago", seconds_ago / 3600);
    } else {
        return format!("{} days ago", seconds_ago / 86400);
    }
}

fn mimetype_icon(mimetype: &str) -> String {
    // Simula la función OCP\mimetype_icon
    format!("/core/img/filetypes/{}.svg", if mimetype == "dir" { "folder" } else { "file" })
}

fn encode_path(path: &str) -> String {
    // Simula la función OCP\Util::encodePath
    urlencoding::encode(path).to_string()
}

fn preview_icon(path: &str) -> String {
    // Simula la función OCA\Files_Trashbin\Trashbin::preview_icon
    format!("/core/preview?file={}&x=32&y=32", urlencoding::encode(path))
}

fn render_file_list(ctx: &Context) -> Markup {
    html! {
        @for file in &ctx.files {
            @let relative_deleted_date = relative_modified_date(file.timestamp);
            // the older the file, the brighter the shade of grey; days*14
            @let relative_date_color = {
                let color = ((SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64 - file.date) / 60 / 60 / 24 * 14) as u8;
                if color > 200 { 200 } else { color }
            };
            @let name = encode_path(&file.name);
            @let directory = encode_path(&file.directory);
            
            tr data-filename=(file.name)
               data-type=(if file.r#type == "dir" { "dir" } else { "file" })
               data-mime=(file.mimetype)
               data-permissions=(file.permissions)
               @if ctx.dirlisting {
                   id=(format!("{}/{}", file.directory, file.name))
                   data-file=(name)
                   data-timestamp=""
                   data-dirlisting="1"
               } @else {
                   id=(format!("{}.d{}", file.name, file.timestamp))
                   data-file=(format!("{}.d{}", file.name, file.timestamp))
                   data-timestamp=(file.timestamp)
                   data-dirlisting="0"
               } {
                @if file.is_preview_available {
                    td class="filename svg preview-icon" 
                        @if file.r#type == "dir" {
                            style=(format!("background-image:url({})", mimetype_icon("dir")))
                        } @else {
                            @if file.is_preview_available {
                                style=(format!("background-image:url({})", 
                                    preview_icon(
                                        &if !ctx.dirlisting { 
                                            format!("{}.d{}", file.name, file.timestamp)
                                        } else { 
                                            format!("{}/{}", file.directory, file.name)
                                        }
                                    )
                                ))
                            } @else {
                                style=(format!("background-image:url({})", mimetype_icon(&file.mimetype)))
                            }
                        } {
                        @if ctx.readonly.is_none() || !ctx.readonly.unwrap() {
                            input type="checkbox" {}
                        }
                        @if file.r#type == "dir" {
                            @if ctx.dirlisting {
                                a class="name" href=(format!("{}/{}", ctx.base_url, name)) title="" {
                            } @else {
                                a class="name" href=(format!("{}/{}.d{}", ctx.base_url, name, file.timestamp)) title="" {
                            }
                        } @else {
                            @if ctx.dirlisting {
                                a class="name" href=(format!("{}/{}", ctx.download_url, name)) title="" {
                            } @else {
                                a class="name" href=(format!("{}/{}.d{}", ctx.download_url, name, file.timestamp)) title="" {
                            }
                        }
                            span class="nametext" {
                                @if file.r#type == "dir" {
                                    (PreEscaped(html_escape::encode_safe(&file.name)))
                                } @else {
                                    (PreEscaped(html_escape::encode_safe(&file.basename)))
                                    span class="extension" { (file.extension) }
                                }
                            }
                            @if file.r#type == "dir" {
                                span class="uploadtext" currentUploads="0" {}
                            }
                        }
                    }
                } @else {
                    td class="filename svg"
                        @if file.r#type == "dir" {
                            style=(format!("background-image:url({})", mimetype_icon("dir")))
                        } @else {
                            @if file.is_preview_available {
                                style=(format!("background-image:url({})", 
                                    preview_icon(
                                        &if !ctx.dirlisting { 
                                            format!("{}.d{}", file.name, file.timestamp)
                                        } else { 
                                            format!("{}/{}", file.directory, file.name)
                                        }
                                    )
                                ))
                            } @else {
                                style=(format!("background-image:url({})", mimetype_icon(&file.mimetype)))
                            }
                        } {
                        @if ctx.readonly.is_none() || !ctx.readonly.unwrap() {
                            input type="checkbox" {}
                        }
                        @if file.r#type == "dir" {
                            @if ctx.dirlisting {
                                a class="name" href=(format!("{}/{}", ctx.base_url, name)) title="" {
                            } @else {
                                a class="name" href=(format!("{}/{}.d{}", ctx.base_url, name, file.timestamp)) title="" {
                            }
                        } @else {
                            @if ctx.dirlisting {
                                a class="name" href=(format!("{}/{}", ctx.download_url, name)) title="" {
                            } @else {
                                a class="name" href=(format!("{}/{}.d{}", ctx.download_url, name, file.timestamp)) title="" {
                            }
                        }
                            span class="nametext" {
                                @if file.r#type == "dir" {
                                    (PreEscaped(html_escape::encode_safe(&file.name)))
                                } @else {
                                    (PreEscaped(html_escape::encode_safe(&file.basename)))
                                    span class="extension" { (file.extension) }
                                }
                            }
                            @if file.r#type == "dir" {
                                span class="uploadtext" currentUploads="0" {}
                            }
                        }
                    }
                }
                
                td class="date" {
                    span class="modified" 
                         title=(file.date)
                         style=(format!("color:rgb({0},{0},{0})", relative_date_color)) {
                        (relative_deleted_date)
                    }
                }
            }
        }
    }
}