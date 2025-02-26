use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "การเข้าถึงได้รับอนุญาตแล้ว");
    m.insert("Error configuring Dropbox storage", "เกิดข้อผิดพลาดในการกำหนดค่าพื้นที่จัดเก็บข้อมูล Dropbox");
    m.insert("Grant access", "อนุญาตให้เข้าถึงได้");
    m.insert("Please provide a valid Dropbox app key and secret.", "กรุณากรอกรหัส app key ของ Dropbox และรหัสลับ");
    m.insert("Error configuring Google Drive storage", "เกิดข้อผิดพลาดในการกำหนดค่าการจัดเก็บข้อมูลในพื้นที่ของ Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>คำเตือน:</b> \"smbclient\" ยังไม่ได้ถูกติดตั้ง. การชี้ CIFS/SMB เพื่อแชร์ข้อมูลไม่สามารถกระทำได้ กรุณาสอบถามข้อมูลเพิ่มเติมจากผู้ดูแลระบบเพื่อติดตั้ง.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>คำเตือน:</b> การสนับสนุนการใช้งาน FTP ในภาษา PHP ยังไม่ได้ถูกเปิดใช้งานหรือถูกติดตั้ง. การชี้ FTP เพื่อแชร์ข้อมูลไม่สามารถดำเนินการได้ กรุณาสอบถามข้อมูลเพิ่มเติมจากผู้ดูแลระบบเพื่อติดตั้ง");
    m.insert("External Storage", "พื้นทีจัดเก็บข้อมูลจากภายนอก");
    m.insert("Folder name", "ชื่อโฟลเดอร์");
    m.insert("Configuration", "การกำหนดค่า");
    m.insert("Options", "ตัวเลือก");
    m.insert("Applicable", "สามารถใช้งานได้");
    m.insert("None set", "ยังไม่มีการกำหนด");
    m.insert("All Users", "ผู้ใช้งานทั้งหมด");
    m.insert("Groups", "กลุ่ม");
    m.insert("Users", "ผู้ใช้งาน");
    m.insert("Delete", "ลบ");
    m.insert("Enable User External Storage", "เปิดให้มีการใช้พื้นที่จัดเก็บข้อมูลของผู้ใช้งานจากภายนอกได้");
    m.insert("Allow users to mount their own external storage", "อนุญาตให้ผู้ใช้งานสามารถชี้ตำแหน่งไปที่พื้นที่จัดเก็บข้อมูลภายนอกของตนเองได้");
    m.insert("SSL root certificates", "ใบรับรองความปลอดภัยด้วยระบบ SSL จาก Root");
    m.insert("Import Root Certificate", "นำเข้าข้อมูลใบรับรองความปลอดภัยจาก Root");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";