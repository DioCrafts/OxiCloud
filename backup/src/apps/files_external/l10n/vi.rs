use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Đã cấp quyền truy cập");
        m.insert("Error configuring Dropbox storage", "Lỗi cấu hình lưu trữ Dropbox ");
        m.insert("Grant access", "Cấp quyền truy cập");
        m.insert("Please provide a valid Dropbox app key and secret.", "Xin vui lòng cung cấp một ứng dụng Dropbox hợp lệ và mã bí mật.");
        m.insert("Error configuring Google Drive storage", "Lỗi cấu hình lưu trữ Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Cảnh báo:</b> \"smbclient\" chưa được cài đặt. Mount CIFS/SMB shares là không thể thực hiện được. Hãy hỏi người quản trị hệ thống để cài đặt nó.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Cảnh báo:</b> FTP trong PHP chưa được cài đặt hoặc chưa được  mở. Mount FTP shares là không thể. Xin hãy yêu cầu quản trị hệ thống của bạn cài đặt nó.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Cảnh báo:</b> Tính năng Curl trong PHP chưa được kích hoạt hoặc cài đặt. Việc gắn kết ownCloud / WebDAV hay GoogleDrive không thực hiện được. Vui lòng liên hệ người quản trị để cài đặt nó.");
        m.insert("External Storage", "Lưu trữ ngoài");
        m.insert("Folder name", "Tên thư mục");
        m.insert("External storage", "Lưu trữ ngoài");
        m.insert("Configuration", "Cấu hình");
        m.insert("Options", "Tùy chọn");
        m.insert("Applicable", "Áp dụng");
        m.insert("Add storage", "Thêm bộ nhớ");
        m.insert("None set", "không");
        m.insert("All Users", "Tất cả người dùng");
        m.insert("Groups", "Nhóm");
        m.insert("Users", "Người dùng");
        m.insert("Delete", "Xóa");
        m.insert("Enable User External Storage", "Kích hoạt tính năng lưu trữ ngoài");
        m.insert("Allow users to mount their own external storage", "Cho phép người dùng kết nối với lưu trữ riêng bên ngoài của họ");
        m.insert("SSL root certificates", "Chứng chỉ SSL root");
        m.insert("Import Root Certificate", "Nhập Root Certificate");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";