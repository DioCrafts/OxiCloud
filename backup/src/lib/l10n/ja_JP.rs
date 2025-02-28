use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.",
        " \"%s\" アプリは、このバージョンのownCloudと互換性がない為、インストールできません。",
    );
    map.insert("No app name specified", "アプリ名が未指定");
    map.insert("Help", "ヘルプ");
    map.insert("Personal", "個人");
    map.insert("Settings", "設定");
    map.insert("Users", "ユーザ");
    map.insert("Admin", "管理");
    map.insert("Failed to upgrade \"%s\".", "\"%s\" へのアップグレードに失敗しました。");
    map.insert("Unknown filetype", "不明なファイルタイプ");
    map.insert("Invalid image", "無効な画像");
    map.insert("web services under your control", "管理下のウェブサービス");
    map.insert("cannot open \"%s\"", "\"%s\" が開けません");
    map.insert("ZIP download is turned off.", "ZIPダウンロードは無効です。");
    map.insert(
        "Files need to be downloaded one by one.",
        "ファイルは1つずつダウンロードする必要があります。",
    );
    map.insert("Back to Files", "ファイルに戻る");
    map.insert(
        "Selected files too large to generate zip file.",
        "選択したファイルはZIPファイルの生成には大きすぎます。",
    );
    map.insert(
        "Download the files in smaller chunks, seperately or kindly ask your administrator.",
        "ファイルは、小さいファイルに分割されてダウンロードされます。もしくは、管理者にお尋ねください。",
    );
    map.insert(
        "No source specified when installing app",
        "アプリインストール時のソースが未指定",
    );
    map.insert(
        "No href specified when installing app from http",
        "アプリインストール時のhttpの URL が未指定",
    );
    map.insert(
        "No path specified when installing app from local file",
        "アプリインストール時のローカルファイルのパスが未指定",
    );
    map.insert(
        "Archives of type %s are not supported",
        "\"%s\"タイプのアーカイブ形式は未サポート",
    );
    map.insert(
        "Failed to open archive when installing app",
        "アプリをインストール中にアーカイブファイルを開けませんでした。",
    );
    map.insert(
        "App does not provide an info.xml file",
        "アプリにinfo.xmlファイルが入っていません",
    );
    map.insert(
        "App can't be installed because of not allowed code in the App",
        "アプリで許可されないコードが入っているのが原因でアプリがインストールできません",
    );
    map.insert(
        "App can't be installed because it is not compatible with this version of ownCloud",
        "アプリは、このバージョンのownCloudと互換性がない為、インストールできません。",
    );
    map.insert(
        "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps",
        "非shippedアプリには許可されない<shipped>true</shipped>タグが含まれているためにアプリをインストール出来ません。",
    );
    map.insert(
        "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store",
        "info.xml/versionのバージョンがアプリストアのバージョンと合っていない為、アプリはインストールされません",
    );
    map.insert("App directory already exists", "アプリディレクトリは既に存在します");
    map.insert(
        "Can't create app folder. Please fix permissions. %s",
        "アプリフォルダを作成出来ませんでした。%s のパーミッションを修正してください。",
    );
    map.insert("Application is not enabled", "アプリケーションは無効です");
    map.insert("Authentication error", "認証エラー");
    map.insert(
        "Token expired. Please reload page.",
        "トークンが無効になりました。ページを再読込してください。",
    );
    map.insert("Files", "ファイル");
    map.insert("Text", "TTY TDD");
    map.insert("Images", "画像");
    map.insert(
        "%s enter the database username.",
        "%s のデータベースのユーザ名を入力してください。",
    );
    map.insert(
        "%s enter the database name.",
        "%s のデータベース名を入力してください。",
    );
    map.insert(
        "%s you may not use dots in the database name",
        "%s ではデータベース名にドットを利用できないかもしれません。",
    );
    map.insert(
        "MS SQL username and/or password not valid: %s",
        "MS SQL サーバーのユーザー名/パスワードが正しくありません: %s",
    );
    map.insert(
        "You need to enter either an existing account or the administrator.",
        "既存のアカウントもしくは管理者のどちらかを入力する必要があります。",
    );
    map.insert(
        "MySQL username and/or password not valid",
        "MySQLのユーザ名もしくはパスワードは有効ではありません",
    );
    map.insert("DB Error: \"%s\"", "DBエラー: \"%s\"");
    map.insert("Offending command was: \"%s\"", "違反コマンド: \"%s\"");
    map.insert(
        "MySQL user '%s'@'localhost' exists already.",
        "MySQLのユーザ '%s'@'localhost' はすでに存在します。",
    );
    map.insert("Drop this user from MySQL", "MySQLからこのユーザを削除");
    map.insert(
        "MySQL user '%s'@'%%' already exists",
        "MySQLのユーザ '%s'@'%%' はすでに存在します。",
    );
    map.insert(
        "Drop this user from MySQL.",
        "MySQLからこのユーザを削除する。",
    );
    map.insert(
        "Oracle connection could not be established",
        "Oracleへの接続が確立できませんでした。",
    );
    map.insert(
        "Oracle username and/or password not valid",
        "Oracleのユーザ名もしくはパスワードは有効ではありません",
    );
    map.insert(
        "Offending command was: \"%s\", name: %s, password: %s",
        "違反コマンド: \"%s\"、名前: %s、パスワード: %s",
    );
    map.insert(
        "PostgreSQL username and/or password not valid",
        "PostgreSQLのユーザ名もしくはパスワードは有効ではありません",
    );
    map.insert("Set an admin username.", "管理者のユーザ名を設定。");
    map.insert("Set an admin password.", "管理者のパスワードを設定。");
    map.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.",
        "WebDAVインタフェースが動作していないと考えられるため、あなたのWEBサーバはまだファイルの同期を許可するように適切な設定がされていません。",
    );
    map.insert(
        "Please double check the <a href='%s'>installation guides</a>.",
        "<a href='%s'>インストールガイド</a>をよく確認してください。",
    );
    map.insert(
        "Could not find category \"%s\"",
        "カテゴリ \"%s\" が見つかりませんでした",
    );
    map.insert("seconds ago", "数秒前");
    map.insert("_%n minute ago_::_%n minutes ago_", "%n 分前");
    map.insert("_%n hour ago_::_%n hours ago_", "%n 時間前");
    map.insert("today", "今日");
    map.insert("yesterday", "昨日");
    map.insert("_%n day go_::_%n days ago_", "%n 日前");
    map.insert("last month", "一月前");
    map.insert("_%n month ago_::_%n months ago_", "%n ヶ月前");
    map.insert("last year", "一年前");
    map.insert("years ago", "年前");
    map.insert("Caused by:", "原因:");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";