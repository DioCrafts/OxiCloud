use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Access granted", "アクセスは許可されました");
    map.insert("Error configuring Dropbox storage", "Dropboxストレージの設定エラー");
    map.insert("Grant access", "アクセスを許可");
    map.insert("Please provide a valid Dropbox app key and secret.", "有効なDropboxアプリのキーとパスワードを入力して下さい。");
    map.insert("Error configuring Google Drive storage", "Googleドライブストレージの設定エラー");
    map.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>警告:</b> \"smbclient\" はインストールされていません。CIFS/SMB 共有のマウントはできません。システム管理者にインストールをお願いして下さい。");
    map.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>警告:</b> PHPのFTPサポートは無効もしくはインストールされていません。FTP共有のマウントはできません。システム管理者にインストールをお願いして下さい。");
    map.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>警告:</b> PHP の Curl サポートは無効もしくはインストールされていません。ownCloud / WebDAV もしくは GoogleDrive のマウントはできません。システム管理者にインストールをお願いして下さい。");
    map.insert("External Storage", "外部ストレージ");
    map.insert("Folder name", "フォルダ名");
    map.insert("External storage", "外部ストレージ");
    map.insert("Configuration", "設定");
    map.insert("Options", "オプション");
    map.insert("Applicable", "適用範囲");
    map.insert("Add storage", "ストレージを追加");
    map.insert("None set", "未設定");
    map.insert("All Users", "すべてのユーザ");
    map.insert("Groups", "グループ");
    map.insert("Users", "ユーザ");
    map.insert("Delete", "削除");
    map.insert("Enable User External Storage", "ユーザの外部ストレージを有効にする");
    map.insert("Allow users to mount their own external storage", "ユーザに外部ストレージのマウントを許可する");
    map.insert("SSL root certificates", "SSLルート証明書");
    map.insert("Import Root Certificate", "ルート証明書をインポート");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";