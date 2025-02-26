// lib/l10n/si_lk.rs

use rust_i18n::i18n;

i18n!("si_LK", {
    "Help": "උදව්",
    "Personal": "පෞද්ගලික",
    "Settings": "සිටුවම්",
    "Users": "පරිශීලකයන්",
    "Admin": "පරිපාලක",
    "web services under your control": "ඔබට පාලනය කළ හැකි වෙබ් සේවාවන්",
    "ZIP download is turned off.": "ZIP භාගත කිරීම් අක්‍රියයි",
    "Files need to be downloaded one by one.": "ගොනු එකින් එක භාගත යුතුයි",
    "Back to Files": "ගොනු වෙතට නැවත යන්න",
    "Selected files too large to generate zip file.": "තෝරාගත් ගොනු ZIP ගොනුවක් තැනීමට විශාල වැඩිය.",
    "Application is not enabled": "යෙදුම සක්‍රිය කර නොමැත",
    "Authentication error": "සත්‍යාපන දෝෂයක්",
    "Token expired. Please reload page.": "ටෝකනය කල් ඉකුත් වී ඇත. පිටුව නැවුම් කරන්න",
    "Files": "ගොනු",
    "Text": "පෙළ",
    "Images": "අනු රූ",
    "seconds ago": "තත්පරයන්ට පෙර",
    "_%n minute ago_::_%n minutes ago_": ["", ""],
    "_%n hour ago_::_%n hours ago_": ["", ""],
    "today": "අද",
    "yesterday": "ඊයේ",
    "_%n day go_::_%n days ago_": ["", ""],
    "last month": "පෙර මාසයේ",
    "_%n month ago_::_%n months ago_": ["", ""],
    "last year": "පෙර අවුරුද්දේ",
    "years ago": "අවුරුදු කීපයකට පෙර"
});

pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}