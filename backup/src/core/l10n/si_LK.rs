use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Sinhala (Sri Lanka) localization
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "ඉරිදා");
    translations.insert("Monday", "සඳුදා");
    translations.insert("Tuesday", "අඟහරුවාදා");
    translations.insert("Wednesday", "බදාදා");
    translations.insert("Thursday", "බ්‍රහස්පතින්දා");
    translations.insert("Friday", "සිකුරාදා");
    translations.insert("Saturday", "සෙනසුරාදා");
    translations.insert("January", "ජනවාරි");
    translations.insert("February", "පෙබරවාරි");
    translations.insert("March", "මාර්තු");
    translations.insert("April", "අප්‍රේල්");
    translations.insert("May", "මැයි");
    translations.insert("June", "ජූනි");
    translations.insert("July", "ජූලි");
    translations.insert("August", "අගෝස්තු");
    translations.insert("September", "සැප්තැම්බර්");
    translations.insert("October", "ඔක්තෝබර");
    translations.insert("November", "නොවැම්බර්");
    translations.insert("December", "දෙසැම්බර්");
    translations.insert("Settings", "සිටුවම්");
    translations.insert("seconds ago", "තත්පරයන්ට පෙර");
    translations.insert("today", "අද");
    translations.insert("yesterday", "ඊයේ");
    translations.insert("last month", "පෙර මාසයේ");
    translations.insert("months ago", "මාස කීපයකට පෙර");
    translations.insert("last year", "පෙර අවුරුද්දේ");
    translations.insert("years ago", "අවුරුදු කීපයකට පෙර");
    translations.insert("Choose", "තෝරන්න");
    translations.insert("Yes", "ඔව්");
    translations.insert("No", "එපා");
    translations.insert("Ok", "හරි");
    translations.insert("Cancel", "එපා");
    translations.insert("Share", "බෙදා හදා ගන්න");
    translations.insert("Error", "දෝෂයක්");
    translations.insert("Password protect", "මුර පදයකින් ආරක්ශාකරන්න");
    translations.insert("Password", "මුර පදය");
    translations.insert("Set expiration date", "කල් ඉකුත් විමේ දිනය දමන්න");
    translations.insert("Expiration date", "කල් ඉකුත් විමේ දිනය");
    translations.insert("Share via email:", "විද්‍යුත් තැපෑල මඟින් බෙදාගන්න: ");
    translations.insert("group", "කණ්ඩායම");
    translations.insert("Unshare", "නොබෙදු");
    translations.insert("can edit", "සංස්කරණය කළ හැක");
    translations.insert("access control", "ප්‍රවේශ පාලනය");
    translations.insert("create", "සදන්න");
    translations.insert("update", "යාවත්කාලීන කරන්න");
    translations.insert("delete", "මකන්න");
    translations.insert("share", "බෙදාහදාගන්න");
    translations.insert("Password protected", "මුර පදයකින් ආරක්ශාකර ඇත");
    translations.insert("Error unsetting expiration date", "කල් ඉකුත් දිනය ඉවත් කිරීමේ දෝෂයක්");
    translations.insert("Error setting expiration date", "කල් ඉකුත් දිනය ස්ථාපනය කිරීමේ දෝෂයක්");
    translations.insert("Warning", "අවවාදය");
    translations.insert("Delete", "මකා දමන්න");
    translations.insert("Add", "එකතු කරන්න");
    translations.insert("You will receive a link to reset your password via Email.", "ඔබගේ මුරපදය ප්‍රත්‍යාරම්භ කිරීම සඳහා යොමුව විද්‍යුත් තැපෑලෙන් ලැබෙනු ඇත");
    translations.insert("Username", "පරිශීලක නම");
    translations.insert("Your password was reset", "ඔබේ මුරපදය ප්‍රත්‍යාරම්භ කරන ලදී");
    translations.insert("To login page", "පිවිසුම් පිටුවට");
    translations.insert("New password", "නව මුරපදය");
    translations.insert("Reset password", "මුරපදය ප්‍රත්‍යාරම්භ කරන්න");
    translations.insert("Personal", "පෞද්ගලික");
    translations.insert("Users", "පරිශීලකයන්");
    translations.insert("Apps", "යෙදුම්");
    translations.insert("Admin", "පරිපාලක");
    translations.insert("Help", "උදව්");
    translations.insert("Access forbidden", "ඇතුල් වීම තහනම්");
    translations.insert("Cloud not found", "සොයා ගත නොහැක");
    translations.insert("Security Warning", "ආරක්ෂක නිවේදනයක්");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "ආරක්ෂිත අහඹු සංඛ්‍යා උත්පාදකයක් නොමැති නම් ඔබගේ ගිණුමට පහරදෙන අයකුට එහි මුරපද යළි පිහිටුවීමට අවශ්‍ය ටෝකන පහසුවෙන් සොයාගෙන ඔබගේ ගිණුම පැහැරගත හැක.");
    translations.insert("Advanced", "දියුණු/උසස්");
    translations.insert("Data folder", "දත්ත ෆෝල්ඩරය");
    translations.insert("Configure the database", "දත්ත සමුදාය හැඩගැසීම");
    translations.insert("will be used", "භාවිතා වනු ඇත");
    translations.insert("Database user", "දත්තගබඩා භාවිතාකරු");
    translations.insert("Database password", "දත්තගබඩාවේ මුරපදය");
    translations.insert("Database name", "දත්තගබඩාවේ නම");
    translations.insert("Database host", "දත්තගබඩා සේවාදායකයා");
    translations.insert("Finish setup", "ස්ථාපනය කිරීම අවසන් කරන්න");
    translations.insert("Log out", "නික්මීම");
    translations.insert("Lost your password?", "මුරපදය අමතකද?");
    translations.insert("remember", "මතක තබාගන්න");
    translations.insert("Log in", "ප්‍රවේශවන්න");
    translations
});

/// Plural forms definition for Sinhala (Sri Lanka)
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Get specific plural forms for minute translations
pub fn get_minute_plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "" }
}

/// Get specific plural forms for hour translations
pub fn get_hour_plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "" }
}

/// Get specific plural forms for day translations
pub fn get_day_plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "" }
}

/// Get specific plural forms for month translations
pub fn get_month_plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "" }
}

/// Get specific plural forms for file conflict translations
pub fn get_file_conflict_plural(count: u32) -> &'static str {
    if count == 1 { "" } else { "" }
}