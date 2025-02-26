use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Thai (Thailand) translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "วันอาทิตย์");
    translations.insert("Monday", "วันจันทร์");
    translations.insert("Tuesday", "วันอังคาร");
    translations.insert("Wednesday", "วันพุธ");
    translations.insert("Thursday", "วันพฤหัสบดี");
    translations.insert("Friday", "วันศุกร์");
    translations.insert("Saturday", "วันเสาร์");
    translations.insert("January", "มกราคม");
    translations.insert("February", "กุมภาพันธ์");
    translations.insert("March", "มีนาคม");
    translations.insert("April", "เมษายน");
    translations.insert("May", "พฤษภาคม");
    translations.insert("June", "มิถุนายน");
    translations.insert("July", "กรกฏาคม");
    translations.insert("August", "สิงหาคม");
    translations.insert("September", "กันยายน");
    translations.insert("October", "ตุลาคม");
    translations.insert("November", "พฤศจิกายน");
    translations.insert("December", "ธันวาคม");
    translations.insert("Settings", "ตั้งค่า");
    translations.insert("seconds ago", "วินาที ก่อนหน้านี้");
    translations.insert("_%n minute ago_::_%n minutes ago_", "");
    translations.insert("_%n hour ago_::_%n hours ago_", "");
    translations.insert("today", "วันนี้");
    translations.insert("yesterday", "เมื่อวานนี้");
    translations.insert("_%n day ago_::_%n days ago_", "");
    translations.insert("last month", "เดือนที่แล้ว");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("months ago", "เดือน ที่ผ่านมา");
    translations.insert("last year", "ปีที่แล้ว");
    translations.insert("years ago", "ปี ที่ผ่านมา");
    translations.insert("Choose", "เลือก");
    translations.insert("Yes", "ตกลง");
    translations.insert("No", "ไม่ตกลง");
    translations.insert("Ok", "ตกลง");
    translations.insert("_{count} file conflict_::_{count} file conflicts_", "");
    translations.insert("Cancel", "ยกเลิก");
    translations.insert("Shared", "แชร์แล้ว");
    translations.insert("Share", "แชร์");
    translations.insert("Error", "ข้อผิดพลาด");
    translations.insert("Error while sharing", "เกิดข้อผิดพลาดในระหว่างการแชร์ข้อมูล");
    translations.insert("Error while unsharing", "เกิดข้อผิดพลาดในการยกเลิกการแชร์ข้อมูล");
    translations.insert("Error while changing permissions", "เกิดข้อผิดพลาดในการเปลี่ยนสิทธิ์การเข้าใช้งาน");
    translations.insert("Shared with you and the group {group} by {owner}", "ได้แชร์ให้กับคุณ และกลุ่ม {group} โดย {owner}");
    translations.insert("Shared with you by {owner}", "ถูกแชร์ให้กับคุณโดย {owner}");
    translations.insert("Password protect", "ใส่รหัสผ่านไว้");
    translations.insert("Password", "รหัสผ่าน");
    translations.insert("Email link to person", "ส่งลิงก์ให้ทางอีเมล");
    translations.insert("Send", "ส่ง");
    translations.insert("Set expiration date", "กำหนดวันที่หมดอายุ");
    translations.insert("Expiration date", "วันที่หมดอายุ");
    translations.insert("Share via email:", "แชร์ผ่านทางอีเมล");
    translations.insert("No people found", "ไม่พบบุคคลที่ต้องการ");
    translations.insert("group", "กลุ่มผู้ใช้งาน");
    translations.insert("Resharing is not allowed", "ไม่อนุญาตให้แชร์ข้อมูลซ้ำได้");
    translations.insert("Shared in {item} with {user}", "ได้แชร์ {item} ให้กับ {user}");
    translations.insert("Unshare", "ยกเลิกการแชร์");
    translations.insert("can edit", "สามารถแก้ไข");
    translations.insert("access control", "ระดับควบคุมการเข้าใช้งาน");
    translations.insert("create", "สร้าง");
    translations.insert("update", "อัพเดท");
    translations.insert("delete", "ลบ");
    translations.insert("share", "แชร์");
    translations.insert("Password protected", "ใส่รหัสผ่านไว้");
    translations.insert("Error unsetting expiration date", "เกิดข้อผิดพลาดในการยกเลิกการตั้งค่าวันที่หมดอายุ");
    translations.insert("Error setting expiration date", "เกิดข้อผิดพลาดในการตั้งค่าวันที่หมดอายุ");
    translations.insert("Sending ...", "กำลังส่ง...");
    translations.insert("Email sent", "ส่งอีเมล์แล้ว");
    translations.insert("Warning", "คำเตือน");
    translations.insert("The object type is not specified.", "ชนิดของวัตถุยังไม่ได้รับการระบุ");
    translations.insert("Delete", "ลบ");
    translations.insert("Add", "เพิ่ม");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "การอัพเดทไม่เป็นผลสำเร็จ กรุณาแจ้งปัญหาที่เกิดขึ้นไปยัง <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">คอมมูนิตี้ผู้ใช้งาน ownCloud</a>");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "การอัพเดทเสร็จเรียบร้อยแล้ว กำลังเปลี่ยนเส้นทางไปที่ ownCloud อยู่ในขณะนี้");
    translations.insert("Use the following link to reset your password: {link}", "ใช้ลิงค์ต่อไปนี้เพื่อเปลี่ยนรหัสผ่านของคุณใหม่: {link}");
    translations.insert("You will receive a link to reset your password via Email.", "คุณจะได้รับลิงค์เพื่อกำหนดรหัสผ่านใหม่ทางอีเมล์");
    translations.insert("Username", "ชื่อผู้ใช้งาน");
    translations.insert("Your password was reset", "รหัสผ่านของคุณถูกเปลี่ยนเรียบร้อยแล้ว");
    translations.insert("To login page", "ไปที่หน้าเข้าสู่ระบบ");
    translations.insert("New password", "รหัสผ่านใหม่");
    translations.insert("Reset password", "เปลี่ยนรหัสผ่าน");
    translations.insert("Personal", "ส่วนตัว");
    translations.insert("Users", "ผู้ใช้งาน");
    translations.insert("Apps", "แอปฯ");
    translations.insert("Admin", "ผู้ดูแล");
    translations.insert("Help", "ช่วยเหลือ");
    translations.insert("Access forbidden", "การเข้าถึงถูกหวงห้าม");
    translations.insert("Cloud not found", "ไม่พบ Cloud");
    translations.insert("Security Warning", "คำเตือนเกี่ยวกับความปลอดภัย");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "ยังไม่มีตัวสร้างหมายเลขแบบสุ่มให้ใช้งาน, กรุณาเปิดใช้งานส่วนเสริม PHP OpenSSL");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "หากปราศจากตัวสร้างหมายเลขแบบสุ่มที่ช่วยป้องกันความปลอดภัย ผู้บุกรุกอาจสามารถที่จะคาดคะเนรหัสยืนยันการเข้าถึงเพื่อรีเซ็ตรหัสผ่าน และเอาบัญชีของคุณไปเป็นของตนเองได้");
    translations.insert("Create an <strong>admin account</strong>", "สร้าง <strong>บัญชีผู้ดูแลระบบ</strong>");
    translations.insert("Advanced", "ขั้นสูง");
    translations.insert("Data folder", "โฟลเดอร์เก็บข้อมูล");
    translations.insert("Configure the database", "กำหนดค่าฐานข้อมูล");
    translations.insert("will be used", "จะถูกใช้");
    translations.insert("Database user", "ชื่อผู้ใช้งานฐานข้อมูล");
    translations.insert("Database password", "รหัสผ่านฐานข้อมูล");
    translations.insert("Database name", "ชื่อฐานข้อมูล");
    translations.insert("Database tablespace", "พื้นที่ตารางในฐานข้อมูล");
    translations.insert("Database host", "Database host");
    translations.insert("Finish setup", "ติดตั้งเรียบร้อยแล้ว");
    translations.insert("Log out", "ออกจากระบบ");
    translations.insert("Automatic logon rejected!", "การเข้าสู่ระบบอัตโนมัติถูกปฏิเสธแล้ว");
    translations.insert("If you did not change your password recently, your account may be compromised!", "หากคุณยังไม่ได้เปลี่ยนรหัสผ่านของคุณเมื่อเร็วๆนี้, บัญชีของคุณอาจถูกบุกรุกโดยผู้อื่น");
    translations.insert("Please change your password to secure your account again.", "กรุณาเปลี่ยนรหัสผ่านของคุณอีกครั้ง เพื่อป้องกันบัญชีของคุณให้ปลอดภัย");
    translations.insert("Lost your password?", "ลืมรหัสผ่าน?");
    translations.insert("remember", "จำรหัสผ่าน");
    translations.insert("Log in", "เข้าสู่ระบบ");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "กำลังอัพเดท ownCloud ไปเป็นรุ่น %s, กรุณารอสักครู่");
    translations
});

/// Defines plural forms for Thai language
pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

/// Gets a translation for the given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Handles pluralized translations
pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    // Thai language has only one plural form
    get_translation(key)
}