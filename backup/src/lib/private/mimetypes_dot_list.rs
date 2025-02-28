// Copyright (c) 2011 Robin Appelman <icewind1991@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// List of mimetypes by extension
lazy_static! {
    pub static ref MIMETYPES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("css", "text/css");
        m.insert("flac", "audio/flac");
        m.insert("gif", "image/gif");
        m.insert("gzip", "application/x-gzip");
        m.insert("gz", "application/x-gzip");
        m.insert("html", "text/html");
        m.insert("htm", "text/html");
        m.insert("ics", "text/calendar");
        m.insert("ical", "text/calendar");
        m.insert("jpeg", "image/jpeg");
        m.insert("jpg", "image/jpeg");
        m.insert("js", "application/javascript");
        m.insert("oga", "audio/ogg");
        m.insert("ogg", "audio/ogg");
        m.insert("ogv", "video/ogg");
        m.insert("pdf", "application/pdf");
        m.insert("png", "image/png");
        m.insert("svg", "image/svg+xml");
        m.insert("tar", "application/x-tar");
        m.insert("tgz", "application/x-compressed");
        m.insert("tar.gz", "application/x-compressed");
        m.insert("tif", "image/tiff");
        m.insert("tiff", "image/tiff");
        m.insert("txt", "text/plain");
        m.insert("zip", "application/zip");
        m.insert("wav", "audio/wav");
        m.insert("odt", "application/vnd.oasis.opendocument.text");
        m.insert("ods", "application/vnd.oasis.opendocument.spreadsheet");
        m.insert("odg", "application/vnd.oasis.opendocument.graphics");
        m.insert("odp", "application/vnd.oasis.opendocument.presentation");
        m.insert("pages", "application/x-iwork-pages-sffpages");
        m.insert("numbers", "application/x-iwork-numbers-sffnumbers");
        m.insert("keynote", "application/x-iwork-keynote-sffkey");
        m.insert("kra", "application/x-krita");
        m.insert("mp3", "audio/mpeg");
        m.insert("doc", "application/msword");
        m.insert("xls", "application/msexcel");
        m.insert("php", "application/x-php");
        m.insert("exe", "application");
        m.insert("pl", "application/x-pearl");
        m.insert("py", "text/x-script.phyton");
        m.insert("blend", "application/x-blender");
        m.insert("xcf", "application/x-gimp");
        m.insert("psd", "application/x-photoshop");
        m.insert("xml", "application/xml");
        m.insert("avi", "video/x-msvideo");
        m.insert("dv", "video/dv");
        m.insert("m2t", "video/mp2t");
        m.insert("mp4", "video/mp4");
        m.insert("m4v", "video/mp4");
        m.insert("mpg", "video/mpeg");
        m.insert("mpeg", "video/mpeg");
        m.insert("mov", "video/quicktime");
        m.insert("webm", "video/webm");
        m.insert("wmv", "video/x-ms-asf");
        m.insert("vcf", "text/vcard");
        m.insert("vcard", "text/vcard");
        m.insert("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document");
        m.insert("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
        m.insert("ppt", "application/mspowerpoint");
        m.insert("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation");
        m.insert("sgf", "application/sgf");
        m.insert("cdr", "application/coreldraw");
        m.insert("impress", "text/impress");
        m.insert("ai", "application/illustrator");
        m.insert("epub", "application/epub+zip");
        m.insert("mobi", "application/x-mobipocket-ebook");
        m.insert("msi", "application");
        m.insert("md", "text/markdown");
        m.insert("markdown", "text/markdown");
        m.insert("mdown", "text/markdown");
        m.insert("mdwn", "text/markdown");
        m.insert("reveal", "text/reveal");
        m
    };
}

/// Get the mimetype for a given file extension
pub fn get_mimetype_for_extension(extension: &str) -> Option<&'static str> {
    MIMETYPES.get(extension).copied()
}