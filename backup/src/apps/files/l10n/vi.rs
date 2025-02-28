use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("vi");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "Không thể di chuyển %s - Đã có tên tập tin này trên hệ thống");
    translations.insert("Could not move %s", "Không thể di chuyển %s");
    translations.insert("File name cannot be empty.", "Tên file không được rỗng");
    translations.insert("No file was uploaded. Unknown error", "Không có tập tin nào được tải lên. Lỗi không xác định");
    translations.insert("There is no error, the file uploaded with success", "Không có lỗi, các tập tin đã được tải lên thành công");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "The uploaded file exceeds the upload_max_filesize directive in php.ini: ");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Tập tin được tải lên vượt quá MAX_FILE_SIZE được quy định trong mẫu HTML");
    translations.insert("The uploaded file was only partially uploaded", "Các tập tin được tải lên chỉ tải lên được một phần");
    translations.insert("No file was uploaded", "Chưa có file nào được tải lên");
    translations.insert("Missing a temporary folder", "Không tìm thấy thư mục tạm");
    translations.insert("Failed to write to disk", "Không thể ghi ");
    translations.insert("Not enough storage available", "Không đủ không gian lưu trữ");
    translations.insert("Invalid directory.", "Thư mục không hợp lệ");
    translations.insert("Files", "Tập tin");
    translations.insert("Not enough space available", "Không đủ chỗ trống cần thiết");
    translations.insert("Upload cancelled.", "Hủy tải lên");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Tập tin tải lên đang được xử lý. Nếu bạn rời khỏi trang bây giờ sẽ hủy quá trình này.");
    translations.insert("{new_name} already exists", "{new_name} đã tồn tại");
    translations.insert("Share", "Chia sẻ");
    translations.insert("Delete permanently", "Xóa vĩnh vễn");
    translations.insert("Rename", "Sửa tên");
    translations.insert("Pending", "Đang chờ");
    translations.insert("replaced {new_name} with {old_name}", "đã thay thế {new_name} bằng {old_name}");
    translations.insert("undo", "lùi lại");
    translations.insert("_%n folder_::_%n folders_", "");
    translations.insert("_%n file_::_%n files_", "");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "");
    translations.insert("'.' is an invalid file name.", "'.' là một tên file không hợp lệ");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Tên không hợp lệ, '\\', '/', '<', '>', ':', '\"', '|', '?' và '*' thì không được phép dùng.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Your storage is full, files can not be updated or synced anymore!");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Your storage is almost full ({usedSpacePercent}%)");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "Your download is being prepared. This might take some time if the files are big.");
    translations.insert("Error", "Lỗi");
    translations.insert("Name", "Tên");
    translations.insert("Size", "Kích cỡ");
    translations.insert("Modified", "Thay đổi");
    translations.insert("Upload", "Tải lên");
    translations.insert("File handling", "Xử lý tập tin");
    translations.insert("Maximum upload size", "Kích thước tối đa ");
    translations.insert("max. possible: ", "tối đa cho phép:");
    translations.insert("Needed for multi-file and folder downloads.", "Cần thiết cho tải nhiều tập tin và thư mục.");
    translations.insert("Enable ZIP-download", "Cho phép ZIP-download");
    translations.insert("0 is unlimited", "0 là không giới hạn");
    translations.insert("Maximum input size for ZIP files", "Kích thước tối đa cho các tập tin ZIP");
    translations.insert("Save", "Lưu");
    translations.insert("New", "Mới");
    translations.insert("Text file", "Tập tin văn bản");
    translations.insert("Folder", "Thư mục");
    translations.insert("From link", "Từ liên kết");
    translations.insert("Deleted files", "File đã bị xóa");
    translations.insert("Cancel upload", "Hủy upload");
    translations.insert("Nothing in here. Upload something!", "Không có gì ở đây .Hãy tải lên một cái gì đó !");
    translations.insert("Download", "Tải về");
    translations.insert("Unshare", "Bỏ chia sẻ");
    translations.insert("Delete", "Xóa");
    translations.insert("Upload too large", "Tập tin tải lên quá lớn");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Các tập tin bạn đang tải lên vượt quá kích thước tối đa cho phép trên máy chủ .");
    translations.insert("Files are being scanned, please wait.", "Tập tin đang được quét ,vui lòng chờ.");
    translations.insert("Current scanning", "Hiện tại đang quét");
    translations.insert("Upgrading filesystem cache...", "Đang nâng cấp bộ nhớ đệm cho tập tin hệ thống...");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}