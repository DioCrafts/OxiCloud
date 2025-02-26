use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("th_TH");

#[rust_i18n::i18n("th_TH")]
pub fn register_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to delete the server configuration".to_string(), "การลบการกำหนดค่าเซิร์ฟเวอร์ล้มเหลว".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "การกำหนดค่าถูกต้องและการเชื่อมต่อสามารถเชื่อมต่อได้!".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "การกำหนดค่าถูกต้อง, แต่การผูกข้อมูลล้มเหลว, กรุณาตรวจสอบการตั้งค่าเซิร์ฟเวอร์และข้อมูลการเข้าใช้งาน".to_string());
    translations.insert("Deletion failed".to_string(), "การลบทิ้งล้มเหลว".to_string());
    translations.insert("Keep settings?".to_string(), "รักษาการตั้งค่าไว้?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "ไม่สามารถเพิ่มค่ากำหนดเซิร์ฟเวอร์ได้".to_string());
    translations.insert("Success".to_string(), "เสร็จสิ้น".to_string());
    translations.insert("Error".to_string(), "ข้อผิดพลาด".to_string());
    translations.insert("Select groups".to_string(), "เลือกกลุ่ม".to_string());
    translations.insert("Connection test succeeded".to_string(), "ทดสอบการเชื่อมต่อสำเร็จ".to_string());
    translations.insert("Connection test failed".to_string(), "ทดสอบการเชื่อมต่อล้มเหลว".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "คุณแน่ใจแล้วหรือว่าต้องการลบการกำหนดค่าเซิร์ฟเวอร์ปัจจุบันทิ้งไป?".to_string());
    translations.insert("Confirm Deletion".to_string(), "ยืนยันการลบทิ้ง".to_string());
    translations.insert("_%s group found_::_%s groups found_".to_string(), "".to_string());
    translations.insert("_%s user found_::_%s users found_".to_string(), "".to_string());
    translations.insert("Save".to_string(), "บันทึก".to_string());
    translations.insert("Help".to_string(), "ช่วยเหลือ".to_string());
    translations.insert("Add Server Configuration".to_string(), "เพิ่มการกำหนดค่าเซิร์ฟเวอร์".to_string());
    translations.insert("Host".to_string(), "โฮสต์".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "คุณสามารถปล่อยช่องโปรโตคอลเว้นไว้ได้, ยกเว้นกรณีที่คุณต้องการใช้ SSL จากนั้นเริ่มต้นด้วย ldaps://".to_string());
    translations.insert("Port".to_string(), "พอร์ต".to_string());
    translations.insert("User DN".to_string(), "DN ของผู้ใช้งาน".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "DN ของผู้ใช้งานที่เป็นลูกค้าอะไรก็ตามที่ผูกอยู่ด้วย เช่น uid=agent, dc=example, dc=com, สำหรับการเข้าถึงโดยบุคคลนิรนาม, ให้เว้นว่าง DN และ รหัสผ่านเอาไว้".to_string());
    translations.insert("Password".to_string(), "รหัสผ่าน".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "สำหรับการเข้าถึงโดยบุคคลนิรนาม ให้เว้นว่าง DN และรหัสผ่านไว้".to_string());
    translations.insert("One Base DN per line".to_string(), "หนึ่ง Base DN ต่อบรรทัด".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "คุณสามารถระบุ DN หลักสำหรับผู้ใช้งานและกลุ่มต่างๆในแท็บขั้นสูงได้".to_string());
    translations.insert("Back".to_string(), "ย้อนกลับ".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>คำเตือน:</b> โมดูล PHP LDAP ยังไม่ได้ถูกติดตั้ง, ระบบด้านหลังจะไม่สามารถทำงานได้ กรุณาติดต่อผู้ดูแลระบบของคุณเพื่อทำการติดตั้งโมดูลดังกล่าว".to_string());
    translations.insert("Connection Settings".to_string(), "ตั้งค่าการเชื่อมต่อ".to_string());
    translations.insert("User Login Filter".to_string(), "ตัวกรองข้อมูลการเข้าสู่ระบบของผู้ใช้งาน".to_string());
    translations.insert("Disable Main Server".to_string(), "ปิดใช้งานเซิร์ฟเวอร์หลัก".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "เซิร์ฟเวอร์ LDAP ประเภท Case insensitive (วินโดวส์)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "ปิดใช้งานการตรวจสอบความถูกต้องของใบรับรองความปลอดภัย SSL".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "ในอีกไม่กี่วินาที ระบบจะเปลี่ยนแปลงข้อมูลในแคชให้ว่างเปล่า".to_string());
    translations.insert("Directory Settings".to_string(), "ตั้งค่าไดเร็กทอรี่".to_string());
    translations.insert("User Display Name Field".to_string(), "ช่องแสดงชื่อผู้ใช้งานที่ต้องการ".to_string());
    translations.insert("Base User Tree".to_string(), "รายการผู้ใช้งานหลักแบบ Tree".to_string());
    translations.insert("One User Base DN per line".to_string(), "หนึ่ง User Base DN ต่อบรรทัด".to_string());
    translations.insert("User Search Attributes".to_string(), "คุณลักษณะการค้นหาชื่อผู้ใช้".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "ตัวเลือกเพิ่มเติม; หนึ่งคุณลักษณะต่อบรรทัด".to_string());
    translations.insert("Group Display Name Field".to_string(), "ช่องแสดงชื่อกลุ่มที่ต้องการ".to_string());
    translations.insert("Base Group Tree".to_string(), "รายการกลุ่มหลักแบบ Tree".to_string());
    translations.insert("One Group Base DN per line".to_string(), "หนึ่ง Group Base DN ต่อบรรทัด".to_string());
    translations.insert("Group Search Attributes".to_string(), "คุณลักษณะการค้นหาแบบกลุ่ม".to_string());
    translations.insert("Group-Member association".to_string(), "ความสัมพันธ์ของสมาชิกในกลุ่ม".to_string());
    translations.insert("Special Attributes".to_string(), "คุณลักษณะพิเศษ".to_string());
    translations.insert("in bytes".to_string(), "ในหน่วยไบต์".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "เว้นว่างไว้สำหรับ ชื่อผู้ใช้ (ค่าเริ่มต้น) หรือไม่กรุณาระบุคุณลักษณะของ LDAP/AD".to_string());
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}