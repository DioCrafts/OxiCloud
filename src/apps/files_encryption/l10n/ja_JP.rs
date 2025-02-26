use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Recovery key successfully enabled", "リカバリ用のキーは正常に有効化されました");
    map.insert("Could not enable recovery key. Please check your recovery key password!", "リカバリ用のキーを有効にできませんでした。リカバリ用のキーのパスワードを確認して下さい！");
    map.insert("Recovery key successfully disabled", "リカバリ用のキーを正常に無効化しました");
    map.insert("Could not disable recovery key. Please check your recovery key password!", "リカバリ用のキーを無効化できませんでした。リカバリ用のキーのパスワードを確認して下さい！");
    map.insert("Password successfully changed.", "パスワードを変更できました。");
    map.insert("Could not change the password. Maybe the old password was not correct.", "パスワードを変更できませんでした。古いパスワードが間違っているかもしれません。");
    map.insert("Private key password successfully updated.", "秘密鍵のパスワードが正常に更新されました。");
    map.insert("Could not update the private key password. Maybe the old password was not correct.", "秘密鍵のパスワードを更新できませんでした。古いパスワードが正確でない場合があります。");
    map.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "暗号化アプリが初期化されていません。暗号化アプリが接続中に再度有効かされた可能性があります。暗号化アプリを初期化する為に、１回ログアウトしてログインしなおしてください。");
    map.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "このファイルを復号化できません、共有ファイルの可能性があります。ファイルの所有者にお願いして、ファイルを共有しなおしてもらってください。");
    map.insert("Unknown error please check your system settings or contact your administrator", "不明なエラーです。システム設定を確認するか、管理者に問い合わせてください。");
    map.insert("Missing requirements.", "必要要件が満たされていません。");
    map.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "必ず、PHP 5.3.3もしくはそれ以上をインストールし、同時にOpenSSLのPHP拡張を有効にした上でOpenSSLも同様にインストール、適切に設定してください。現時点では暗号化アプリは無効になっています。");
    map.insert("Following users are not set up for encryption:", "以下のユーザーは、暗号化設定がされていません：");
    map.insert("Saving...", "保存中...");
    map.insert("Go directly to your ", "あなたのディレクトリへ");
    map.insert("personal settings", "秘密鍵をアンロックできます");
    map.insert("Encryption", "暗号化");
    map.insert("Enable recovery key (allow to recover users files in case of password loss):", "復旧キーを有効化 (万一パスワードを亡くした場合もユーザーのファイルを回復できる):");
    map.insert("Recovery key password", "復旧キーのパスワード");
    map.insert("Repeat Recovery key password", "復旧キーのパスワードをもう一度入力");
    map.insert("Enabled", "有効");
    map.insert("Disabled", "無効");
    map.insert("Change recovery key password:", "復旧キーのパスワードを変更:");
    map.insert("Old Recovery key password", "古い復旧キーのパスワード");
    map.insert("New Recovery key password", "新しい復旧キーのパスワード");
    map.insert("Repeat New Recovery key password", "新しい復旧キーのパスワードをもう一度入力");
    map.insert("Change Password", "パスワードを変更");
    map.insert("Your private key password no longer match your log-in password:", "もはや秘密鍵はログインパスワードと一致しません:");
    map.insert("Set your old private key password to your current log-in password.", "古い秘密鍵のパスワードを現在のログインパスワードに設定する。");
    map.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "古いパスワードを覚えていない場合、管理者に尋ねてファイルを回復することができます。");
    map.insert("Old log-in password", "古いログインパスワード");
    map.insert("Current log-in password", "現在のログインパスワード");
    map.insert("Update Private Key Password", "秘密鍵のパスワードを更新");
    map.insert("Enable password recovery:", "パスワード復旧を有効化:");
    map.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "このオプションを有効にすると、パスワードを紛失した場合も、暗号化されたファイルに再度アクセスすることができるようになります。");
    map.insert("File recovery settings updated", "ファイル復旧設定が更新されました");
    map.insert("Could not update file recovery", "ファイル復旧を更新できませんでした");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";