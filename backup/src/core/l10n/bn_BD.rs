use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Sunday", "রবিবার");
        m.insert("Monday", "সোমবার");
        m.insert("Tuesday", "মঙ্গলবার");
        m.insert("Wednesday", "বুধবার");
        m.insert("Thursday", "বৃহস্পতিবার");
        m.insert("Friday", "শুক্রবার");
        m.insert("Saturday", "শনিবার");
        m.insert("January", "জানুয়ারি");
        m.insert("February", "ফেব্রুয়ারি");
        m.insert("March", "মার্চ");
        m.insert("April", "এপ্রিল");
        m.insert("May", "মে");
        m.insert("June", "জুন");
        m.insert("July", "জুলাই");
        m.insert("August", "অগাষ্ট");
        m.insert("September", "সেপ্টেম্বর");
        m.insert("October", "অক্টোবর");
        m.insert("November", "নভেম্বর");
        m.insert("December", "ডিসেম্বর");
        m.insert("Settings", "নিয়ামকসমূহ");
        m.insert("seconds ago", "সেকেন্ড পূর্বে");
        m.insert("today", "আজ");
        m.insert("yesterday", "গতকাল");
        m.insert("last month", "গত মাস");
        m.insert("months ago", "মাস পূর্বে");
        m.insert("last year", "গত বছর");
        m.insert("years ago", "বছর পূর্বে");
        m.insert("Choose", "বেছে নিন");
        m.insert("Yes", "হ্যাঁ");
        m.insert("No", "না");
        m.insert("Ok", "তথাস্তু");
        m.insert("Cancel", "বাতির");
        m.insert("Shared", "ভাগাভাগিকৃত");
        m.insert("Share", "ভাগাভাগি কর");
        m.insert("Error", "সমস্যা");
        m.insert("Error while sharing", "ভাগাভাগি করতে সমস্যা দেখা দিয়েছে  ");
        m.insert("Error while unsharing", "ভাগাভাগি বাতিল করতে সমস্যা দেখা দিয়েছে");
        m.insert("Error while changing permissions", "অনুমতিসমূহ  পরিবর্তন করতে সমস্যা দেখা দিয়েছে");
        m.insert("Shared with you and the group {group} by {owner}", "{owner} আপনার এবং {group} গোষ্ঠীর সাথে ভাগাভাগি করেছেন");
        m.insert("Shared with you by {owner}", "{owner} আপনার সাথে ভাগাভাগি করেছেন");
        m.insert("Password protect", "কূটশব্দ সুরক্ষিত");
        m.insert("Password", "কূটশব্দ");
        m.insert("Email link to person", "ব্যক্তির সাথে ই-মেইল যুক্ত কর");
        m.insert("Send", "পাঠাও");
        m.insert("Set expiration date", "মেয়াদোত্তীর্ণ হওয়ার তারিখ নির্ধারণ করুন");
        m.insert("Expiration date", "মেয়াদোত্তীর্ণ হওয়ার তারিখ");
        m.insert("Share via email:", "ই-মেইলের মাধ্যমে ভাগাভাগি করুনঃ");
        m.insert("No people found", "কোন ব্যক্তি খুঁজে পাওয়া গেল না");
        m.insert("Resharing is not allowed", "পূনঃরায় ভাগাভাগি অনুমোদিত নয়");
        m.insert("Shared in {item} with {user}", "{user} এর সাথে {item} ভাগাভাগি করা হয়েছে");
        m.insert("Unshare", "ভাগাভাগি বাতিল ");
        m.insert("can edit", "সম্পাদনা করতে পারবেন");
        m.insert("access control", "অধিগম্যতা নিয়ন্ত্রণ");
        m.insert("create", "তৈরী করুন");
        m.insert("update", "পরিবর্ধন কর");
        m.insert("delete", "মুছে ফেল");
        m.insert("share", "ভাগাভাগি কর");
        m.insert("Password protected", "কূটশব্দদ্বারা সুরক্ষিত");
        m.insert("Error unsetting expiration date", "মেয়াদোত্তীর্ণ হওয়ার তারিখ নির্ধারণ বাতিল করতে সমস্যা দেখা দিয়েছে");
        m.insert("Error setting expiration date", "মেয়াদোত্তীর্ণ হওয়ার তারিখ নির্ধারণ করতে সমস্যা দেখা দিয়েছে");
        m.insert("Sending ...", "পাঠানো হচ্ছে......");
        m.insert("Email sent", "ই-মেইল পাঠানো হয়েছে");
        m.insert("Warning", "সতর্কবাণী");
        m.insert("The object type is not specified.", "অবজেক্টের ধরণটি সুনির্দিষ্ট নয়।");
        m.insert("Delete", "মুছে");
        m.insert("Add", "যোগ কর");
        m.insert("Use the following link to reset your password: {link}", "আপনার কূটশব্দটি পূনঃনির্ধারণ  করার জন্য নিম্নোক্ত লিংকটি ব্যবহার করুনঃ {link}");
        m.insert("You will receive a link to reset your password via Email.", "কূটশব্দ পূনঃনির্ধারণের জন্য একটি টূনঃনির্ধারণ লিংকটি আপনাকে ই-মেইলে পাঠানো হয়েছে ।");
        m.insert("Username", "ব্যবহারকারী");
        m.insert("Your password was reset", "আপনার কূটশব্দটি  পূনঃনির্ধারণ  করা হয়েছে");
        m.insert("To login page", "প্রবেশ পৃষ্ঠায়");
        m.insert("New password", "নতুন কূটশব্দ");
        m.insert("Reset password", "কূটশব্দ পূনঃনির্ধারণ কর");
        m.insert("Personal", "ব্যক্তিগত");
        m.insert("Users", "ব্যবহারকারী");
        m.insert("Apps", "অ্যাপ");
        m.insert("Admin", "প্রশাসন");
        m.insert("Help", "সহায়িকা");
        m.insert("Access forbidden", "অধিগমনের অনুমতি নেই");
        m.insert("Cloud not found", "ক্লাউড খুঁজে পাওয়া গেল না");
        m.insert("Security Warning", "নিরাপত্তাজনিত সতর্কতা");
        m.insert("Create an <strong>admin account</strong>", "<strong>প্রশাসক একাউন্ট</strong> তৈরী করুন");
        m.insert("Advanced", "সুচারু");
        m.insert("Data folder", "ডাটা ফোল্ডার ");
        m.insert("Configure the database", "ডাটাবেচ কনফিগার করুন");
        m.insert("will be used", "ব্যবহৃত হবে");
        m.insert("Database user", "ডাটাবেজ ব্যবহারকারী");
        m.insert("Database password", "ডাটাবেজ কূটশব্দ");
        m.insert("Database name", "ডাটাবেজের নাম");
        m.insert("Database tablespace", "ডাটাবেজ টেবলস্পেস");
        m.insert("Database host", "ডাটাবেজ হোস্ট");
        m.insert("Finish setup", "সেটআপ সুসম্পন্ন কর");
        m.insert("Log out", "প্রস্থান");
        m.insert("Lost your password?", "কূটশব্দ হারিয়েছেন?");
        m.insert("remember", "মনে রাখ");
        m.insert("Log in", "প্রবেশ");
        m.insert("Updating ownCloud to version %s, this may take a while.", "%s ভার্সনে ownCloud পরিবর্ধন করা হচ্ছে, এজন্য কিছু সময় প্রয়োজন।");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, (Vec<&'static str>, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", (vec!["", ""], ""));
        m.insert("_%n hour ago_::_%n hours ago_", (vec!["", ""], ""));
        m.insert("_%n day ago_::_%n days ago_", (vec!["", ""], ""));
        m.insert("_%n month ago_::_%n months ago_", (vec!["", ""], ""));
        m.insert("_{count} file conflict_::_{count} file conflicts_", (vec!["", ""], ""));
        m
    };
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}