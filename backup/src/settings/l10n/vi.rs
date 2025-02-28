use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Vietnamese translations for OwnCloud
pub static VI_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Unable to load list from App Store", "Không thể tải danh sách ứng dụng từ App Store");
    translations.insert("Authentication error", "Lỗi xác thực");
    translations.insert("Group already exists", "Nhóm đã tồn tại");
    translations.insert("Unable to add group", "Không thể thêm nhóm");
    translations.insert("Email saved", "Lưu email");
    translations.insert("Invalid email", "Email không hợp lệ");
    translations.insert("Unable to delete group", "Không thể xóa nhóm");
    translations.insert("Unable to delete user", "Không thể xóa người dùng");
    translations.insert("Language changed", "Ngôn ngữ đã được thay đổi");
    translations.insert("Invalid request", "Yêu cầu không hợp lệ");
    translations.insert("Admins can't remove themself from the admin group", "Quản trị viên không thể loại bỏ chính họ khỏi nhóm quản lý");
    translations.insert("Unable to add user to group %s", "Không thể thêm người dùng vào nhóm %s");
    translations.insert("Unable to remove user from group %s", "Không thể xóa người dùng từ nhóm %s");
    translations.insert("Couldn't update app.", "Không thể cập nhật ứng dụng");
    translations.insert("Update to {appversion}", "Cập nhật lên {appversion}");
    translations.insert("Disable", "Tắt");
    translations.insert("Enable", "Bật");
    translations.insert("Please wait....", "Xin hãy đợi...");
    translations.insert("Updating....", "Đang cập nhật...");
    translations.insert("Error while updating app", "Lỗi khi cập nhật ứng dụng");
    translations.insert("Error", "Lỗi");
    translations.insert("Update", "Cập nhật");
    translations.insert("Updated", "Đã cập nhật");
    translations.insert("Saving...", "Đang lưu...");
    translations.insert("deleted", "đã xóa");
    translations.insert("undo", "lùi lại");
    translations.insert("Groups", "Nhóm");
    translations.insert("Group Admin", "Nhóm quản trị");
    translations.insert("Delete", "Xóa");
    translations.insert("__language_name__", "__Ngôn ngữ___");
    translations.insert("Security Warning", "Cảnh bảo bảo mật");
    translations.insert("Cron", "Cron");
    translations.insert("Execute one task with each page loaded", "Thực thi tác vụ mỗi khi trang được tải");
    translations.insert("Sharing", "Chia sẻ");
    translations.insert("Enable Share API", "Bật chia sẻ API");
    translations.insert("Allow apps to use the Share API", "Cho phép các ứng dụng sử dụng chia sẻ API");
    translations.insert("Allow links", "Cho phép liên kết");
    translations.insert("Allow users to share items to the public with links", "Cho phép người dùng chia sẻ công khai các mục bằng các liên kết");
    translations.insert("Allow resharing", "Cho phép chia sẻ lại");
    translations.insert("Allow users to share items shared with them again", "Cho phép người dùng chia sẻ lại những mục đã được chia sẻ");
    translations.insert("Allow users to share with anyone", "Cho phép người dùng chia sẻ với bất cứ ai");
    translations.insert("Allow users to only share with users in their groups", "Chỉ cho phép người dùng chia sẻ với những người dùng trong nhóm của họ");
    translations.insert("Log", "Log");
    translations.insert("More", "hơn");
    translations.insert("Less", "ít");
    translations.insert("Version", "Phiên bản");
    translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Được phát triển bởi <a href=\"http://ownCloud.org/contact\" target=\"_blank\">cộng đồng ownCloud</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">mã nguồn </a> đã được cấp phép theo chuẩn <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
    translations.insert("Add your App", "Thêm ứng dụng của bạn");
    translations.insert("More Apps", "Nhiều ứng dụng hơn");
    translations.insert("Select an App", "Chọn một ứng dụng");
    translations.insert("See application page at apps.owncloud.com", "Xem nhiều ứng dụng hơn tại apps.owncloud.com");
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-Giấy phép được cấp bởi  <span class=\"author\"></span>");
    translations.insert("User Documentation", "Tài liệu người sử dụng");
    translations.insert("Administrator Documentation", "Tài liệu quản trị");
    translations.insert("Online Documentation", "Tài liệu trực tuyến");
    translations.insert("Forum", "Diễn đàn");
    translations.insert("Bugtracker", "Hệ ghi nhận lỗi");
    translations.insert("Commercial Support", "Hỗ trợ có phí");
    translations.insert("Get the apps to sync your files", "Nhận ứng dụng để đồng bộ file của bạn");
    translations.insert("Show First Run Wizard again", "Hiện lại việc chạy đồ thuật khởi đầu");
    translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Bạn đã sử dụng <strong>%s </ strong> có sẵn <strong> %s </ strong>");
    translations.insert("Password", "Mật khẩu");
    translations.insert("Your password was changed", "Mật khẩu của bạn đã được thay đổi.");
    translations.insert("Unable to change your password", "Không thể đổi mật khẩu");
    translations.insert("Current password", "Mật khẩu cũ");
    translations.insert("New password", "Mật khẩu mới");
    translations.insert("Change password", "Đổi mật khẩu");
    translations.insert("Email", "Email");
    translations.insert("Your email address", "Email của bạn");
    translations.insert("Fill in an email address to enable password recovery", "Nhập địa chỉ email của bạn để khôi phục lại mật khẩu");
    translations.insert("Language", "Ngôn ngữ");
    translations.insert("Help translate", "Hỗ trợ dịch thuật");
    translations.insert("WebDAV", "WebDAV");
    translations.insert("Encryption", "Mã hóa");
    translations.insert("Login Name", "Tên đăng nhập");
    translations.insert("Create", "Tạo");
    translations.insert("Default Storage", "Bộ nhớ mặc định");
    translations.insert("Unlimited", "Không giới hạn");
    translations.insert("Other", "Khác");
    translations.insert("Username", "Tên đăng nhập");
    translations.insert("Storage", "Bộ nhớ");
    translations.insert("set new password", "đặt mật khẩu mới");
    translations.insert("Default", "Mặc định");
    translations
});

/// Returns plural forms configuration for Vietnamese language
pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

/// Gets translation for a given key
pub fn get_translation(key: &str) -> &'static str {
    VI_TRANSLATIONS.get(key).copied().unwrap_or(key)
}