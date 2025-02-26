use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Vietnamese translation (vi)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Sunday", "Chủ nhật");
    m.insert("Monday", "Thứ 2");
    m.insert("Tuesday", "Thứ 3");
    m.insert("Wednesday", "Thứ 4");
    m.insert("Thursday", "Thứ 5");
    m.insert("Friday", "Thứ ");
    m.insert("Saturday", "Thứ 7");
    m.insert("January", "Tháng 1");
    m.insert("February", "Tháng 2");
    m.insert("March", "Tháng 3");
    m.insert("April", "Tháng 4");
    m.insert("May", "Tháng 5");
    m.insert("June", "Tháng 6");
    m.insert("July", "Tháng 7");
    m.insert("August", "Tháng 8");
    m.insert("September", "Tháng 9");
    m.insert("October", "Tháng 10");
    m.insert("November", "Tháng 11");
    m.insert("December", "Tháng 12");
    m.insert("Settings", "Cài đặt");
    m.insert("seconds ago", "vài giây trước");
    m.insert("_%n minute ago_::_%n minutes ago_", "");
    m.insert("_%n hour ago_::_%n hours ago_", "");
    m.insert("today", "hôm nay");
    m.insert("yesterday", "hôm qua");
    m.insert("_%n day ago_::_%n days ago_", "");
    m.insert("last month", "tháng trước");
    m.insert("_%n month ago_::_%n months ago_", "");
    m.insert("months ago", "tháng trước");
    m.insert("last year", "năm trước");
    m.insert("years ago", "năm trước");
    m.insert("Choose", "Chọn");
    m.insert("Yes", "Có");
    m.insert("No", "Không");
    m.insert("Ok", "Đồng ý");
    m.insert("_{count} file conflict_::_{count} file conflicts_", "");
    m.insert("Cancel", "Hủy");
    m.insert("Shared", "Được chia sẻ");
    m.insert("Share", "Chia sẻ");
    m.insert("Error", "Lỗi");
    m.insert("Error while sharing", "Lỗi trong quá trình chia sẻ");
    m.insert("Error while unsharing", "Lỗi trong quá trình gỡ chia sẻ");
    m.insert("Error while changing permissions", "Lỗi trong quá trình phân quyền");
    m.insert("Shared with you and the group {group} by {owner}", "Đã được chia sẽ với bạn và nhóm {group} bởi {owner}");
    m.insert("Shared with you by {owner}", "Đã được chia sẽ bởi {owner}");
    m.insert("Password protect", "Mật khẩu bảo vệ");
    m.insert("Password", "Mật khẩu");
    m.insert("Email link to person", "Liên kết email tới cá nhân");
    m.insert("Send", "Gởi");
    m.insert("Set expiration date", "Đặt ngày kết thúc");
    m.insert("Expiration date", "Ngày kết thúc");
    m.insert("Share via email:", "Chia sẻ thông qua email");
    m.insert("No people found", "Không tìm thấy người nào");
    m.insert("group", "nhóm");
    m.insert("Resharing is not allowed", "Chia sẻ lại không được cho phép");
    m.insert("Shared in {item} with {user}", "Đã được chia sẽ trong {item} với {user}");
    m.insert("Unshare", "Bỏ chia sẻ");
    m.insert("can edit", "có thể chỉnh sửa");
    m.insert("access control", "quản lý truy cập");
    m.insert("create", "tạo");
    m.insert("update", "cập nhật");
    m.insert("delete", "xóa");
    m.insert("share", "chia sẻ");
    m.insert("Password protected", "Mật khẩu bảo vệ");
    m.insert("Error unsetting expiration date", "Lỗi không thiết lập ngày kết thúc");
    m.insert("Error setting expiration date", "Lỗi cấu hình ngày kết thúc");
    m.insert("Sending ...", "Đang gởi ...");
    m.insert("Email sent", "Email đã được gửi");
    m.insert("Warning", "Cảnh báo");
    m.insert("The object type is not specified.", "Loại đối tượng không được chỉ định.");
    m.insert("Delete", "Xóa");
    m.insert("Add", "Thêm");
    m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Cập nhật không thành công . Vui lòng thông báo đến <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\"> Cộng đồng ownCloud </a>.");
    m.insert("The update was successful. Redirecting you to ownCloud now.", "Cập nhật thành công .Hệ thống sẽ đưa bạn tới ownCloud.");
    m.insert("Use the following link to reset your password: {link}", "Dùng đường dẫn sau để khôi phục lại mật khẩu : {link}");
    m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Liên kết tạo lại mật khẩu đã được gửi tới hộp thư của bạn.<br>Nếu bạn không thấy nó sau một khoảng thời gian, vui lòng kiểm tra trong thư mục Spam/Rác.<br>Nếu vẫn không thấy, vui lòng hỏi người quản trị hệ thống.");
    m.insert("Request failed!<br>Did you make sure your email/username was right?", "Yêu cầu thất bại!<br>Bạn có chắc là email/tên đăng nhập của bạn chính xác?");
    m.insert("You will receive a link to reset your password via Email.", "Vui lòng kiểm tra Email để khôi phục lại mật khẩu.");
    m.insert("Username", "Tên đăng nhập");
    m.insert("Your password was reset", "Mật khẩu của bạn đã được khôi phục");
    m.insert("To login page", "Trang đăng nhập");
    m.insert("New password", "Mật khẩu mới");
    m.insert("Reset password", "Khôi phục mật khẩu");
    m.insert("Personal", "Cá nhân");
    m.insert("Users", "Người dùng");
    m.insert("Apps", "Ứng dụng");
    m.insert("Admin", "Quản trị");
    m.insert("Help", "Giúp đỡ");
    m.insert("Access forbidden", "Truy cập bị cấm");
    m.insert("Cloud not found", "Không tìm thấy Clound");
    m.insert("Security Warning", "Cảnh bảo bảo mật");
    m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Phiên bản PHP của bạn có lỗ hổng NULL Byte attack (CVE-2006-7243)");
    m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Không an toàn ! chức năng random number generator đã có sẵn ,vui lòng bật  PHP OpenSSL extension.");
    m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Nếu không có random number generator , Hacker có thể  thiết lập lại mật khẩu và chiếm tài khoản của bạn.");
    m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Thư mục và file dữ liệu của bạn có thể được truy cập từ internet bởi vì file .htaccess không hoạt động");
    m.insert("Create an <strong>admin account</strong>", "Tạo một <strong>tài khoản quản trị</strong>");
    m.insert("Advanced", "Nâng cao");
    m.insert("Data folder", "Thư mục dữ liệu");
    m.insert("Configure the database", "Cấu hình cơ sở dữ liệu");
    m.insert("will be used", "được sử dụng");
    m.insert("Database user", "Người dùng cơ sở dữ liệu");
    m.insert("Database password", "Mật khẩu cơ sở dữ liệu");
    m.insert("Database name", "Tên cơ sở dữ liệu");
    m.insert("Database tablespace", "Cơ sở dữ liệu tablespace");
    m.insert("Database host", "Database host");
    m.insert("Finish setup", "Cài đặt hoàn tất");
    m.insert("%s is available. Get more information on how to update.", "%s còn trống. Xem thêm thông tin cách cập nhật.");
    m.insert("Log out", "Đăng xuất");
    m.insert("Automatic logon rejected!", "Tự động đăng nhập đã bị từ chối !");
    m.insert("If you did not change your password recently, your account may be compromised!", "Nếu bạn không thay đổi mật khẩu gần đây của bạn, tài khoản của bạn có thể gặp nguy hiểm!");
    m.insert("Please change your password to secure your account again.", "Vui lòng thay đổi mật khẩu của bạn để đảm bảo tài khoản của bạn một lần nữa.");
    m.insert("Lost your password?", "Bạn quên mật khẩu ?");
    m.insert("remember", "ghi nhớ");
    m.insert("Log in", "Đăng nhập");
    m.insert("Alternative Logins", "Đăng nhập khác");
    m.insert("Updating ownCloud to version %s, this may take a while.", "Cập nhật ownCloud lên phiên bản %s, có thể sẽ mất thời gian");
    m
});

/// Returns the plural forms rule for Vietnamese language
pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}