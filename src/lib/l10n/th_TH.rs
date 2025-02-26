// lib/l10n/th_th.rs

use phf::phf_map;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ThThTranslations;

impl ThThTranslations {
    pub fn get_translations() -> &'static phf::Map<&'static str, &'static str> {
        &TRANSLATIONS
    }

    pub fn get_plural_forms() -> &'static str {
        PLURAL_FORMS
    }

    pub fn get_plural_translations() -> &'static HashMap<&'static str, Vec<&'static str>> {
        &PLURAL_TRANSLATIONS
    }
}

static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Help" => "ช่วยเหลือ",
    "Personal" => "ส่วนตัว",
    "Settings" => "ตั้งค่า",
    "Users" => "ผู้ใช้งาน",
    "Admin" => "ผู้ดูแล",
    "web services under your control" => "เว็บเซอร์วิสที่คุณควบคุมการใช้งานได้",
    "ZIP download is turned off." => "คุณสมบัติการดาวน์โหลด zip ถูกปิดการใช้งานไว้",
    "Files need to be downloaded one by one." => "ไฟล์สามารถดาวน์โหลดได้ทีละครั้งเท่านั้น",
    "Back to Files" => "กลับไปที่ไฟล์",
    "Selected files too large to generate zip file." => "ไฟล์ที่เลือกมีขนาดใหญ่เกินกว่าที่จะสร้างเป็นไฟล์ zip",
    "Application is not enabled" => "แอพพลิเคชั่นดังกล่าวยังไม่ได้เปิดใช้งาน",
    "Authentication error" => "เกิดข้อผิดพลาดในสิทธิ์การเข้าใช้งาน",
    "Token expired. Please reload page." => "รหัสยืนยันความถูกต้องหมดอายุแล้ว กรุณาโหลดหน้าเว็บใหม่อีกครั้ง",
    "Files" => "ไฟล์",
    "Text" => "ข้อความ",
    "Images" => "รูปภาพ",
    "Could not find category \"%s\"" => "ไม่พบหมวดหมู่ \"%s\"",
    "seconds ago" => "วินาที ก่อนหน้านี้",
    "today" => "วันนี้",
    "yesterday" => "เมื่อวานนี้",
    "last month" => "เดือนที่แล้ว",
    "last year" => "ปีที่แล้ว",
    "years ago" => "ปี ที่ผ่านมา",
};

lazy_static::lazy_static! {
    static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec![""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec![""]);
        m.insert("_%n day go_::_%n days ago_", vec![""]);
        m.insert("_%n month ago_::_%n months ago_", vec![""]);
        m
    };
}

static PLURAL_FORMS: &str = "nplurals=1; plural=0;";