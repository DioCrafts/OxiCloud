use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "පිවිසීමට හැක");
    m.insert("Error configuring Dropbox storage", "Dropbox ගබඩාව වින්‍යාස කිරීමේ දෝශයක් ඇත");
    m.insert("Grant access", "පිවිසුම ලබාදෙන්න");
    m.insert("Please provide a valid Dropbox app key and secret.", "කරුණාකර වලංගු Dropbox යෙදුම් යතුරක් හා රහසක් ලබාදෙන්න.");
    m.insert("Error configuring Google Drive storage", "Google Drive ගබඩාව වින්‍යාස කිරීමේ දෝශයක් ඇත");
    m.insert("External Storage", "භාහිර ගබඩාව");
    m.insert("Folder name", "ෆොල්ඩරයේ නම");
    m.insert("Configuration", "වින්‍යාසය");
    m.insert("Options", "විකල්පයන්");
    m.insert("Applicable", "අදාළ");
    m.insert("None set", "කිසිවක් නැත");
    m.insert("All Users", "සියළු පරිශීලකයන්");
    m.insert("Groups", "කණ්ඩායම්");
    m.insert("Users", "පරිශීලකයන්");
    m.insert("Delete", "මකා දමන්න");
    m.insert("Enable User External Storage", "පරිශීලක භාහිර ගබඩාවන් සක්‍රිය කරන්න");
    m.insert("Allow users to mount their own external storage", "පරිශීලකයන්ට තමාගේම භාහිර ගබඩාවන් මවුන්ට් කිරීමේ අයිතිය දෙන්න");
    m.insert("SSL root certificates", "SSL මූල සහතිකයන්");
    m.insert("Import Root Certificate", "මූල සහතිකය ආයාත කරන්න");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";