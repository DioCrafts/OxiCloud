use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "Xóa thất bại");
        m.insert("Success", "Thành công");
        m.insert("Error", "Lỗi");
        m.insert("Select groups", "Chọn nhóm");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "Lưu");
        m.insert("Help", "Giúp đỡ");
        m.insert("Host", "Máy chủ");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Bạn có thể bỏ qua các giao thức, ngoại trừ SSL. Sau đó bắt đầu với ldaps://");
        m.insert("Port", "Cổng");
        m.insert("User DN", "Người dùng DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Các DN của người sử dụng đã được thực hiện, ví dụ như uid =agent , dc = example, dc = com. Để truy cập nặc danh ,DN và mật khẩu trống.");
        m.insert("Password", "Mật khẩu");
        m.insert("For anonymous access, leave DN and Password empty.", "Cho phép truy cập nặc danh , DN và mật khẩu trống.");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Bạn có thể chỉ định DN cơ bản cho người dùng và các nhóm trong tab Advanced");
        m.insert("Back", "Trở lại");
        m.insert("Connection Settings", "Connection Settings");
        m.insert("User Login Filter", "Lọc người dùng đăng nhập");
        m.insert("Backup (Replica) Port", "Cổng sao lưu (Replica)");
        m.insert("Disable Main Server", "Tắt máy chủ chính");
        m.insert("Case insensitve LDAP server (Windows)", "Trường hợp insensitve LDAP máy chủ (Windows)");
        m.insert("Turn off SSL certificate validation.", "Tắt xác thực chứng nhận SSL");
        m.insert("in seconds. A change empties the cache.", "trong vài giây. Một sự thay đổi bộ nhớ cache.");
        m.insert("Directory Settings", "Directory Settings");
        m.insert("User Display Name Field", "Hiển thị tên người sử dụng");
        m.insert("Base User Tree", "Cây người dùng cơ bản");
        m.insert("User Search Attributes", "User Search Attributes");
        m.insert("Optional; one attribute per line", "Optional; one attribute per line");
        m.insert("Group Display Name Field", "Hiển thị tên nhóm");
        m.insert("Base Group Tree", "Cây nhóm cơ bản");
        m.insert("Group Search Attributes", "Group Search Attributes");
        m.insert("Group-Member association", "Nhóm thành viên Cộng đồng");
        m.insert("Special Attributes", "Special Attributes");
        m.insert("in bytes", "Theo Byte");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Để trống tên người dùng (mặc định). Nếu không chỉ định thuộc tính LDAP/AD");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}