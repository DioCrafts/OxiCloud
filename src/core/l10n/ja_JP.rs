use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%sが あなたと »%s«を共有しました");
        m.insert("Couldn't send mail to following users: %s ", "次のユーザにメールを送信できませんでした: %s");
        m.insert("Turned on maintenance mode", "メンテナンスモードがオンになりました");
        m.insert("Turned off maintenance mode", "メンテナンスモードがオフになりました");
        m.insert("Updated database", "データベース更新完了");
        m.insert("Updating filecache, this may take really long...", "ファイルキャッシュを更新しています、しばらく掛かる恐れがあります...");
        m.insert("Updated filecache", "ファイルキャッシュ更新完了");
        m.insert("... %d%% done ...", "... %d%% 完了 ...");
        m.insert("No image or file provided", "画像もしくはファイルが提供されていません");
        m.insert("Unknown filetype", "不明なファイルタイプ");
        m.insert("Invalid image", "無効な画像");
        m.insert("No temporary profile picture available, try again", "一時的なプロファイル用画像が利用できません。もう一度試して下さい");
        m.insert("No crop data provided", "クロップデータは提供されません");
        m.insert("Sunday", "日");
        m.insert("Monday", "月");
        m.insert("Tuesday", "火");
        m.insert("Wednesday", "水");
        m.insert("Thursday", "木");
        m.insert("Friday", "金");
        m.insert("Saturday", "土");
        m.insert("January", "1月");
        m.insert("February", "2月");
        m.insert("March", "3月");
        m.insert("April", "4月");
        m.insert("May", "5月");
        m.insert("June", "6月");
        m.insert("July", "7月");
        m.insert("August", "8月");
        m.insert("September", "9月");
        m.insert("October", "10月");
        m.insert("November", "11月");
        m.insert("December", "12月");
        m.insert("Settings", "設定");
        m.insert("seconds ago", "数秒前");
        m.insert("_%n minute ago_::_%n minutes ago_", "%n 分前");
        m.insert("_%n hour ago_::_%n hours ago_", "%n 時間前");
        m.insert("today", "今日");
        m.insert("yesterday", "昨日");
        m.insert("_%n day ago_::_%n days ago_", "%n 日前");
        m.insert("last month", "一月前");
        m.insert("_%n month ago_::_%n months ago_", "%n ヶ月前");
        m.insert("months ago", "月前");
        m.insert("last year", "一年前");
        m.insert("years ago", "年前");
        m.insert("Choose", "選択");
        m.insert("Error loading file picker template: {error}", "ファイル選択テンプレートの読み込みエラー: {error}");
        m.insert("Yes", "はい");
        m.insert("No", "いいえ");
        m.insert("Ok", "OK");
        m.insert("Error loading message template: {error}", "メッセージテンプレートの読み込みエラー: {error}");
        m.insert("_{count} file conflict_::_{count} file conflicts_", "{count} ファイルが競合");
        m.insert("One file conflict", "1ファイルが競合");
        m.insert("Which files do you want to keep?", "どちらのファイルを保持したいですか？");
        m.insert("If you select both versions, the copied file will have a number added to its name.", "両方のバージョンを選択した場合は、ファイル名の後ろに数字を追加したファイルのコピーを作成します。");
        m.insert("Cancel", "キャンセル");
        m.insert("Continue", "続ける");
        m.insert("(all selected)", "(全て選択)");
        m.insert("({count} selected)", "({count} 選択)");
        m.insert("Error loading file exists template", "既存ファイルのテンプレートの読み込みエラー");
        m.insert("Shared", "共有中");
        m.insert("Share", "共有");
        m.insert("Error", "エラー");
        m.insert("Error while sharing", "共有でエラー発生");
        m.insert("Error while unsharing", "共有解除でエラー発生");
        m.insert("Error while changing permissions", "権限変更でエラー発生");
        m.insert("Shared with you and the group {group} by {owner}", "あなたと {owner} のグループ {group} で共有中");
        m.insert("Shared with you by {owner}", "{owner} と共有中");
        m.insert("Share with user or group …", "ユーザもしくはグループと共有 ...");
        m.insert("Share link", "共有リンク");
        m.insert("Password protect", "パスワード保護");
        m.insert("Password", "パスワード");
        m.insert("Allow Public Upload", "アップロードを許可");
        m.insert("Email link to person", "メールリンク");
        m.insert("Send", "送信");
        m.insert("Set expiration date", "有効期限を設定");
        m.insert("Expiration date", "有効期限");
        m.insert("Share via email:", "メール経由で共有:");
        m.insert("No people found", "ユーザーが見つかりません");
        m.insert("group", "グループ");
        m.insert("Resharing is not allowed", "再共有は許可されていません");
        m.insert("Shared in {item} with {user}", "{item} 内で {user} と共有中");
        m.insert("Unshare", "共有解除");
        m.insert("notify by email", "メールで通知");
        m.insert("can edit", "編集を許可");
        m.insert("access control", "アクセス権限");
        m.insert("create", "作成");
        m.insert("update", "更新");
        m.insert("delete", "削除");
        m.insert("share", "共有");
        m.insert("Password protected", "パスワード保護");
        m.insert("Error unsetting expiration date", "有効期限の未設定エラー");
        m.insert("Error setting expiration date", "有効期限の設定でエラー発生");
        m.insert("Sending ...", "送信中...");
        m.insert("Email sent", "メールを送信しました");
        m.insert("Warning", "警告");
        m.insert("The object type is not specified.", "オブジェクタイプが指定されていません。");
        m.insert("Enter new", "新規に入力");
        m.insert("Delete", "削除");
        m.insert("Add", "追加");
        m.insert("Edit tags", "タグを編集");
        m.insert("Error loading dialog template: {error}", "メッセージテンプレートの読み込みエラー: {error}");
        m.insert("No tags selected for deletion.", "削除するタグが選択されていません。");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "更新に成功しました。この問題を <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a> にレポートしてください。");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "更新に成功しました。今すぐownCloudにリダイレクトします。");
        m.insert("%s password reset", "%s パスワードリセット");
        m.insert("Use the following link to reset your password: {link}", "パスワードをリセットするには次のリンクをクリックして下さい: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "パスワードリセットのリンクをあなたのメールアドレスに送信しました。<br>しばらくたっても受信出来ない場合は、スパム／迷惑メールフォルダを確認して下さい。<br>もしそこにもない場合は、管理者に問い合わせてください。");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "リクエストに失敗しました！<br>あなたのメール／ユーザ名が正しいことを確認しましたか？");
        m.insert("You will receive a link to reset your password via Email.", "メールでパスワードをリセットするリンクが届きます。");
        m.insert("Username", "ユーザー名");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "ファイルが暗号化されています。復旧キーを有効にしていなかった場合、パスワードをリセットしてからデータを復旧する方法はありません。何をすべきかよくわからないなら、続ける前にまず管理者に連絡しましょう。本当に続けますか？");
        m.insert("Yes, I really want to reset my password now", "はい、今すぐパスワードをリセットします。");
        m.insert("Reset", "リセット");
        m.insert("Your password was reset", "あなたのパスワードはリセットされました。");
        m.insert("To login page", "ログインページへ戻る");
        m.insert("New password", "新しいパスワードを入力");
        m.insert("Reset password", "パスワードをリセット");
        m.insert("Personal", "個人");
        m.insert("Users", "ユーザ");
        m.insert("Apps", "アプリ");
        m.insert("Admin", "管理");
        m.insert("Help", "ヘルプ");
        m.insert("Error loading tags", "タグの読み込みエラー");
        m.insert("Tag already exists", "タグはすでに存在します");
        m.insert("Error deleting tag(s)", "タグの削除エラー");
        m.insert("Error tagging", "タグの付与エラー");
        m.insert("Error untagging", "タグの解除エラー");
        m.insert("Error favoriting", "お気に入りに追加エラー");
        m.insert("Error unfavoriting", "お気に入りから削除エラー");
        m.insert("Access forbidden", "アクセスが禁止されています");
        m.insert("Cloud not found", "見つかりません");
        m.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "こんにちは、\n\n%s があなたと %s を共有したことをお知らせします。\nそれを表示: %s\n");
        m.insert("The share will expire on %s.\n\n", "共有は %s で有効期限が切れます。\n\n");
        m.insert("Cheers!", "それでは！");
        m.insert("Security Warning", "セキュリティ警告");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "あなたのPHPのバージョンには、Null Byte攻撃(CVE-2006-7243)という脆弱性が含まれています。");
        m.insert("Please update your PHP installation to use %s securely.", "%s を安全に利用する為に インストールされているPHPをアップデートしてください。");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "セキュアな乱数生成器が利用可能ではありません。PHPのOpenSSL拡張を有効にして下さい。");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "セキュアな乱数生成器が無い場合、攻撃者がパスワードリセットのトークンを予測してアカウントを乗っ取られる可能性があります。");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", ".htaccess ファイルが動作していないため、おそらくあなたのデータディレクトリもしくはファイルはインターネットからアクセス可能です。");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "サーバーを適正に設定する情報は、こちらの<a href=\"%s\" target=\"_blank\">ドキュメント</a>を参照してください。");
        m.insert("Create an <strong>admin account</strong>", "<strong>管理者アカウント</strong>を作成してください");
        m.insert("Advanced", "詳細設定");
        m.insert("Data folder", "データフォルダ");
        m.insert("Configure the database", "データベースを設定してください");
        m.insert("will be used", "が使用されます");
        m.insert("Database user", "データベースのユーザ名");
        m.insert("Database password", "データベースのパスワード");
        m.insert("Database name", "データベース名");
        m.insert("Database tablespace", "データベースの表領域");
        m.insert("Database host", "データベースのホスト名");
        m.insert("Finish setup", "セットアップを完了します");
        m.insert("Finishing …", "終了しています ...");
        m.insert("%s is available. Get more information on how to update.", "%s が利用可能です。更新方法に関してさらに情報を取得して下さい。");
        m.insert("Log out", "ログアウト");
        m.insert("Automatic logon rejected!", "自動ログインは拒否されました！");
        m.insert("If you did not change your password recently, your account may be compromised!", "最近パスワードを変更していない場合、あなたのアカウントは危険にさらされているかもしれません。");
        m.insert("Please change your password to secure your account again.", "アカウント保護の為、パスワードを再度の変更をお願いいたします。");
        m.insert("Server side authentication failed!", "サーバサイドの認証に失敗しました！");
        m.insert("Please contact your administrator.", "管理者に問い合わせてください。");
        m.insert("Lost your password?", "パスワードを忘れましたか？");
        m.insert("remember", "パスワードを記憶する");
        m.insert("Log in", "ログイン");
        m.insert("Alternative Logins", "代替ログイン");
        m.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "こんにちは、<br><br>%sがあなたと »%s« を共有したことをお知らせします。<br><a href=\"%s\">それを表示</a><br><br>");
        m.insert("The share will expire on %s.<br><br>", "共有は %s で有効期限が切れます。<br><br>");
        m.insert("Updating ownCloud to version %s, this may take a while.", "ownCloud をバージョン %s に更新しています、しばらくお待ち下さい。");
        m.insert("This ownCloud instance is currently being updated, which may take a while.", "この ownCloud インスタンスは現在更新中であり、しばらく時間がかかります。");
        m.insert("Please reload this page after a short time to continue using ownCloud.", "ownCloud を続けて利用するには、しばらくした後でページをリロードしてください。");
        m.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "このメッセージが引き続きもしくは予期せず現れる場合は、システム管理者に連絡してください。");
        m.insert("Thank you for your patience.", "しばらくお待ちください。");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}