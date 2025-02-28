use std::collections::HashMap;
use once_cell::sync::Lazy;

// Translations map for ja_JP locale
pub static JA_JP_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "%s を移動できませんでした ― この名前のファイルはすでに存在します");
    translations.insert("Could not move %s", "%s を移動できませんでした");
    translations.insert("File name cannot be empty.", "ファイル名を空にすることはできません。");
    translations.insert("File name must not contain \"/\". Please choose a different name.", "ファイル名には \"/\" を含めることはできません。別の名前を選択してください。");
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.", "%s はフォルダ %s ないですでに使われています。別の名前を選択してください。");
    translations.insert("Not a valid source", "有効なソースではありません");
    translations.insert("Error while downloading %s to %s", "%s から %s へのダウンロードエラー");
    translations.insert("Error when creating the file", "ファイルの生成エラー");
    translations.insert("Folder name cannot be empty.", "フォルダ名は空にできません");
    translations.insert("Folder name must not contain \"/\". Please choose a different name.", "フォルダ名には \"/\" を含めることはできません。別の名前を選択してください。");
    translations.insert("Error when creating the folder", "フォルダの生成エラー");
    translations.insert("Unable to set upload directory.", "アップロードディレクトリを設定出来ません。");
    translations.insert("Invalid Token", "無効なトークン");
    translations.insert("No file was uploaded. Unknown error", "ファイルは何もアップロードされていません。不明なエラー");
    translations.insert("There is no error, the file uploaded with success", "エラーはありません。ファイルのアップロードは成功しました");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "アップロードされたファイルはphp.ini の upload_max_filesize に設定されたサイズを超えています:");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "アップロードファイルはHTMLフォームで指定された MAX_FILE_SIZE の制限を超えています");
    translations.insert("The uploaded file was only partially uploaded", "アップロードファイルは一部分だけアップロードされました");
    translations.insert("No file was uploaded", "ファイルはアップロードされませんでした");
    translations.insert("Missing a temporary folder", "一時保存フォルダが見つかりません");
    translations.insert("Failed to write to disk", "ディスクへの書き込みに失敗しました");
    translations.insert("Not enough storage available", "ストレージに十分な空き容量がありません");
    translations.insert("Upload failed. Could not get file info.", "アップロードに失敗。ファイル情報を取得できませんでした。");
    translations.insert("Upload failed. Could not find uploaded file", "アップロードに失敗。アップロード済みのファイルを見つけることができませんでした。");
    translations.insert("Invalid directory.", "無効なディレクトリです。");
    translations.insert("Files", "ファイル");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "ディレクトリもしくは0バイトのため {filename} をアップロードできません");
    translations.insert("Not enough space available", "利用可能なスペースが十分にありません");
    translations.insert("Upload cancelled.", "アップロードはキャンセルされました。");
    translations.insert("Could not get result from server.", "サーバから結果を取得できませんでした。");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "ファイル転送を実行中です。今このページから移動するとアップロードが中止されます。");
    translations.insert("URL cannot be empty", "URL は空にできません");
    translations.insert("In the home folder 'Shared' is a reserved filename", "ホームフォルダでは、'Shared' はシステムが使用する予約済みのファイル名です");
    translations.insert("{new_name} already exists", "{new_name} はすでに存在しています");
    translations.insert("Could not create file", "ファイルを作成できませんでした");
    translations.insert("Could not create folder", "フォルダを作成できませんでした");
    translations.insert("Share", "共有");
    translations.insert("Delete permanently", "完全に削除する");
    translations.insert("Rename", "名前の変更");
    translations.insert("Pending", "中断");
    translations.insert("Could not rename file", "ファイルの名前変更ができませんでした");
    translations.insert("replaced {new_name} with {old_name}", "{old_name} を {new_name} に置換");
    translations.insert("undo", "元に戻す");
    translations.insert("_%n folder_::_%n folders_", "%n 個のフォルダ");
    translations.insert("_%n file_::_%n files_", "%n 個のファイル");
    translations.insert("{dirs} and {files}", "{dirs} と {files}");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "%n 個のファイルをアップロード中");
    translations.insert("'.' is an invalid file name.", "'.' は無効なファイル名です。");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "無効な名前、'\\', '/', '<', '>', ':', '\"', '|', '?', '*' は使用できません。");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "あなたのストレージは一杯です。ファイルの更新と同期はもうできません！");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "あなたのストレージはほぼ一杯です（{usedSpacePercent}%）");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "暗号化アプリは有効ですが、あなたの暗号化キーは初期化されていません。ログアウトした後に、再度ログインしてください");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "暗号化アプリの無効なプライベートキーです。あなたの暗号化されたファイルへアクセスするために、個人設定からプライベートキーのパスワードを更新してください。");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "暗号化の機能は無効化されましたが、ファイルはすでに暗号化されています。個人設定からファイルを複合を行ってください。");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "ダウンロードの準備中です。ファイルサイズが大きい場合は少し時間がかかるかもしれません。");
    translations.insert("Error moving file", "ファイルの移動エラー");
    translations.insert("Error", "エラー");
    translations.insert("Name", "名前");
    translations.insert("Size", "サイズ");
    translations.insert("Modified", "更新日時");
    translations.insert("%s could not be renamed", "%sの名前を変更できませんでした");
    translations.insert("Upload", "アップロード");
    translations.insert("File handling", "ファイル操作");
    translations.insert("Maximum upload size", "最大アップロードサイズ");
    translations.insert("max. possible: ", "最大容量: ");
    translations.insert("Needed for multi-file and folder downloads.", "複数ファイルおよびフォルダのダウンロードに必要");
    translations.insert("Enable ZIP-download", "ZIP形式のダウンロードを有効にする");
    translations.insert("0 is unlimited", "0を指定した場合は無制限");
    translations.insert("Maximum input size for ZIP files", "ZIPファイルへの最大入力サイズ");
    translations.insert("Save", "保存");
    translations.insert("New", "新規作成");
    translations.insert("Text file", "テキストファイル");
    translations.insert("Folder", "フォルダ");
    translations.insert("From link", "リンク");
    translations.insert("Deleted files", "ゴミ箱");
    translations.insert("Cancel upload", "アップロードをキャンセル");
    translations.insert("You don't have permission to upload or create files here", "ここにファイルをアップロードもしくは作成する権限がありません");
    translations.insert("Nothing in here. Upload something!", "ここには何もありません。何かアップロードしてください。");
    translations.insert("Download", "ダウンロード");
    translations.insert("Unshare", "共有解除");
    translations.insert("Delete", "削除");
    translations.insert("Upload too large", "アップロードには大きすぎます。");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "アップロードしようとしているファイルは、サーバで規定された最大サイズを超えています。");
    translations.insert("Files are being scanned, please wait.", "ファイルをスキャンしています、しばらくお待ちください。");
    translations.insert("Current scanning", "スキャン中");
    translations.insert("Upgrading filesystem cache...", "ファイルシステムキャッシュを更新中...");
    
    translations
});

// Plural forms information for ja_JP locale
pub const JA_JP_PLURAL_FORMS: &str = "nplurals=1; plural=0;";

/// Get translation for a key
pub fn get_translation(key: &str) -> Option<&'static str> {
    JA_JP_TRANSLATIONS.get(key).copied()
}

/// Format translation with arguments
pub fn format_translation(key: &str, args: &[&str]) -> String {
    if let Some(translation) = get_translation(key) {
        let mut result = translation.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%s", i + 1), arg);
            result = result.replace("%s", arg);
        }
        result
    } else {
        key.to_string()
    }
}