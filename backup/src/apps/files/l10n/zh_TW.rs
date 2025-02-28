use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "無法移動 %s ，同名的檔案已經存在");
        m.insert("Could not move %s", "無法移動 %s");
        m.insert("File name cannot be empty.", "檔名不能為空");
        m.insert("File name must not contain \"/\". Please choose a different name.", "檔名不能包含 \"/\" ，請選其他名字");
        m.insert("The name %s is already used in the folder %s. Please choose a different name.", "%s 已經被使用於資料夾 %s ，請換一個名字");
        m.insert("Not a valid source", "不是有效的來源");
        m.insert("Error while downloading %s to %s", "下載 %s 到 %s 失敗");
        m.insert("Error when creating the file", "建立檔案失敗");
        m.insert("Folder name cannot be empty.", "資料夾名稱不能留空");
        m.insert("Folder name must not contain \"/\". Please choose a different name.", "資料夾名稱不能包含 \"/\" ，請選其他名字");
        m.insert("Error when creating the folder", "建立資料夾失敗");
        m.insert("Unable to set upload directory.", "無法設定上傳目錄");
        m.insert("Invalid Token", "無效的 token");
        m.insert("No file was uploaded. Unknown error", "沒有檔案被上傳，原因未知");
        m.insert("There is no error, the file uploaded with success", "一切都順利，檔案上傳成功");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "上傳的檔案大小超過 php.ini 當中 upload_max_filesize 參數的設定：");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "上傳的檔案大小超過 HTML 表單中 MAX_FILE_SIZE 的限制");
        m.insert("The uploaded file was only partially uploaded", "只有檔案的一部分被上傳");
        m.insert("No file was uploaded", "沒有檔案被上傳");
        m.insert("Missing a temporary folder", "找不到暫存資料夾");
        m.insert("Failed to write to disk", "寫入硬碟失敗");
        m.insert("Not enough storage available", "儲存空間不足");
        m.insert("Upload failed. Could not get file info.", "上傳失敗，無法取得檔案資訊");
        m.insert("Upload failed. Could not find uploaded file", "上傳失敗，找不到上傳的檔案");
        m.insert("Invalid directory.", "無效的資料夾");
        m.insert("Files", "檔案");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "因為 {filename} 是個目錄或是大小為零，所以無法上傳");
        m.insert("Not enough space available", "沒有足夠的可用空間");
        m.insert("Upload cancelled.", "上傳已取消");
        m.insert("Could not get result from server.", "無法從伺服器取回結果");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "檔案上傳中，離開此頁面將會取消上傳。");
        m.insert("URL cannot be empty", "URL 不能留空");
        m.insert("In the home folder 'Shared' is a reserved filename", "在家目錄中不能使用「共享」作為檔名");
        m.insert("{new_name} already exists", "{new_name} 已經存在");
        m.insert("Could not create file", "無法建立檔案");
        m.insert("Could not create folder", "無法建立資料夾");
        m.insert("Share", "分享");
        m.insert("Delete permanently", "永久刪除");
        m.insert("Rename", "重新命名");
        m.insert("Pending", "等候中");
        m.insert("Could not rename file", "無法重新命名");
        m.insert("replaced {new_name} with {old_name}", "使用 {new_name} 取代 {old_name}");
        m.insert("undo", "復原");
        m.insert("_%n folder_::_%n folders_", "%n 個資料夾");
        m.insert("_%n file_::_%n files_", "%n 個檔案");
        m.insert("{dirs} and {files}", "{dirs} 和 {files}");
        m.insert("_Uploading %n file_::_Uploading %n files_", "%n 個檔案正在上傳");
        m.insert("'.' is an invalid file name.", "'.' 是不合法的檔名");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "檔名不合法，不允許 \\ / < > : \" | ? * 字元");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "您的儲存空間已滿，沒有辦法再更新或是同步檔案！");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "您的儲存空間快要滿了 ({usedSpacePercent}%)");
        m.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "檔案加密已啓用，但是您的金鑰尚未初始化，請重新登入一次");
        m.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "無效的檔案加密私鑰，請在個人設定中更新您的私鑰密語以存取加密的檔案。");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "加密已經被停用，但是您的舊檔案還是處於已加密的狀態，請前往個人設定以解密這些檔案。");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "正在準備您的下載，若您的檔案較大，將會需要更多時間。");
        m.insert("Error moving file", "移動檔案失敗");
        m.insert("Error", "錯誤");
        m.insert("Name", "名稱");
        m.insert("Size", "大小");
        m.insert("Modified", "修改時間");
        m.insert("%s could not be renamed", "無法重新命名 %s");
        m.insert("Upload", "上傳");
        m.insert("File handling", "檔案處理");
        m.insert("Maximum upload size", "上傳限制");
        m.insert("max. possible: ", "最大允許：");
        m.insert("Needed for multi-file and folder downloads.", "下載多檔案和目錄時，此項是必填的。");
        m.insert("Enable ZIP-download", "啟用 ZIP 下載");
        m.insert("0 is unlimited", "0代表沒有限制");
        m.insert("Maximum input size for ZIP files", "ZIP 壓縮前的原始大小限制");
        m.insert("Save", "儲存");
        m.insert("New", "新增");
        m.insert("Text file", "文字檔");
        m.insert("Folder", "資料夾");
        m.insert("From link", "從連結");
        m.insert("Deleted files", "回收桶");
        m.insert("Cancel upload", "取消上傳");
        m.insert("You don't have permission to upload or create files here", "您沒有權限在這裡上傳或建立檔案");
        m.insert("Nothing in here. Upload something!", "這裡還沒有東西，上傳一些吧！");
        m.insert("Download", "下載");
        m.insert("Unshare", "取消分享");
        m.insert("Delete", "刪除");
        m.insert("Upload too large", "上傳過大");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "您試圖上傳的檔案大小超過伺服器的限制。");
        m.insert("Files are being scanned, please wait.", "正在掃描檔案，請稍等。");
        m.insert("Current scanning", "正在掃描");
        m.insert("Upgrading filesystem cache...", "正在升級檔案系統快取…");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

/// Returns the translation for the given string
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Returns the plural form definition
pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}