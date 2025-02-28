/// Vietnamese translation file

use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::translation_hashmap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = translation_hashmap![
        "Help" => "Giúp đỡ",
        "Personal" => "Cá nhân",
        "Settings" => "Cài đặt",
        "Users" => "Người dùng",
        "Admin" => "Quản trị",
        "web services under your control" => "dịch vụ web dưới sự kiểm soát của bạn",
        "ZIP download is turned off." => "Tải về ZIP đã bị tắt.",
        "Files need to be downloaded one by one." => "Tập tin cần phải được tải về từng người một.",
        "Back to Files" => "Trở lại tập tin",
        "Selected files too large to generate zip file." => "Tập tin được chọn quá lớn để tạo tập tin ZIP.",
        "Application is not enabled" => "Ứng dụng không được BẬT",
        "Authentication error" => "Lỗi xác thực",
        "Token expired. Please reload page." => "Mã Token đã hết hạn. Hãy tải lại trang.",
        "Files" => "Tập tin",
        "Text" => "Văn bản",
        "Images" => "Hình ảnh",
        "Could not find category \"%s\"" => "không thể tìm thấy mục \"%s\"",
        "seconds ago" => "vài giây trước",
        "_%n minute ago_::_%n minutes ago_" => "",
        "_%n hour ago_::_%n hours ago_" => "",
        "today" => "hôm nay",
        "yesterday" => "hôm qua",
        "_%n day go_::_%n days ago_" => "",
        "last month" => "tháng trước",
        "_%n month ago_::_%n months ago_" => "",
        "last year" => "năm trước",
        "years ago" => "năm trước"
    ];

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}