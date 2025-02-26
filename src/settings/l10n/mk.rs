use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Неможам да вчитам листа од App Store");
        m.insert("Authentication error", "Грешка во автентикација");
        m.insert("Group already exists", "Групата веќе постои");
        m.insert("Unable to add group", "Неможе да додадам група");
        m.insert("Email saved", "Електронската пошта е снимена");
        m.insert("Invalid email", "Неисправна електронска пошта");
        m.insert("Unable to delete group", "Неможе да избришам група");
        m.insert("Unable to delete user", "Неможам да избришам корисник");
        m.insert("Language changed", "Јазикот е сменет");
        m.insert("Invalid request", "Неправилно барање");
        m.insert("Admins can't remove themself from the admin group", "Администраторите неможе да се избришат себеси од админ групата");
        m.insert("Unable to add user to group %s", "Неможе да додадам корисник во група %s");
        m.insert("Unable to remove user from group %s", "Неможе да избришам корисник од група %s");
        m.insert("Couldn't update app.", "Не можам да ја надградам апликацијата.");
        m.insert("Wrong password", "Погрешна лозинка");
        m.insert("No user supplied", "Нема корисничко име");
        m.insert("Unable to change password", "Вашата лозинка неможе да се смени");
        m.insert("Update to {appversion}", "Надгради на {appversion}");
        m.insert("Disable", "Оневозможи");
        m.insert("Enable", "Овозможи");
        m.insert("Please wait....", "Ве молам почекајте ...");
        m.insert("Error while disabling app", "Грешка при исклучувањето на апликацијата");
        m.insert("Error while enabling app", "Грешка при вклучувањето на апликацијата");
        m.insert("Updating....", "Надградувам ...");
        m.insert("Error while updating app", "Грешка додека ја надградувам апликацијата");
        m.insert("Error", "Грешка");
        m.insert("Update", "Ажурирај");
        m.insert("Updated", "Надграден");
        m.insert("Select a profile picture", "Одбери фотографија за профилот");
        m.insert("Saving...", "Снимам...");
        m.insert("deleted", "избришан");
        m.insert("undo", "врати");
        m.insert("Unable to remove user", "Не можам да го одстранам корисникот");
        m.insert("Groups", "Групи");
        m.insert("Group Admin", "Администратор на група");
        m.insert("Delete", "Избриши");
        m.insert("add group", "додади група");
        m.insert("A valid username must be provided", "Мора да се обезбеди валидно корисничко име ");
        m.insert("Error creating user", "Грешка при креирање на корисникот");
        m.insert("A valid password must be provided", "Мора да се обезбеди валидна лозинка");
        m.insert("__language_name__", "__language_name__");
        m.insert("Security Warning", "Безбедносно предупредување");
        m.insert("Setup Warning", "Предупредување при подесување");
        m.insert("Locale not working", "Локалето не функционира");
        m.insert("Sharing", "Споделување");
        m.insert("Enable Share API", "Овозможи го API-то за споделување");
        m.insert("Allow apps to use the Share API", "Дозволете апликациите да го користат API-то за споделување");
        m.insert("Allow links", "Дозволи врски");
        m.insert("Allow public uploads", "Дозволи јавен аплоуд");
        m.insert("Allow resharing", "Овозможи повторно споделување");
        m.insert("Allow users to share with anyone", "Овозможи корисниците да споделуваат со секого");
        m.insert("Allow users to only share with users in their groups", "Овозможи корисниците да споделуваат со корисници од своите групи");
        m.insert("Allow mail notification", "Овозможи известување по електронска пошта");
        m.insert("Allow user to send mail notification for shared files", "Овозможи корисник да испраќа известување по електронска пошта за споделени датотеки");
        m.insert("Security", "Безбедност");
        m.insert("Enforce HTTPS", "Наметни HTTPS");
        m.insert("Log", "Записник");
        m.insert("Log level", "Ниво на логирање");
        m.insert("More", "Повеќе");
        m.insert("Less", "Помалку");
        m.insert("Version", "Верзија");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Развој од <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud заедницата</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">изворниот код</a> е лиценциран со<a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "Додадете ја Вашата апликација");
        m.insert("More Apps", "Повеќе аппликации");
        m.insert("Select an App", "Избери аппликација");
        m.insert("See application page at apps.owncloud.com", "Види ја страницата со апликации на apps.owncloud.com");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-лиценцирано од <span class=\"author\"></span>");
        m.insert("User Documentation", "Корисничка документација");
        m.insert("Administrator Documentation", "Администраторска документација");
        m.insert("Online Documentation", "Документација на интернет");
        m.insert("Forum", "Форум");
        m.insert("Commercial Support", "Комерцијална подршка");
        m.insert("Get the apps to sync your files", "Преземете апликации за синхронизирање на вашите датотеки");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Имате искористено <strong>%s</strong> од достапните <strong>%s</strong>");
        m.insert("Password", "Лозинка");
        m.insert("Your password was changed", "Вашата лозинка беше променета.");
        m.insert("Unable to change your password", "Вашата лозинка неможе да се смени");
        m.insert("Current password", "Моментална лозинка");
        m.insert("New password", "Нова лозинка");
        m.insert("Change password", "Смени лозинка");
        m.insert("Email", "Е-пошта");
        m.insert("Your email address", "Вашата адреса за е-пошта");
        m.insert("Fill in an email address to enable password recovery", "Пополни ја адресата за е-пошта за да може да ја обновуваш лозинката");
        m.insert("Profile picture", "Фотографија за профил");
        m.insert("Upload new", "Префрли нова");
        m.insert("Select new from Files", "Одбери нова од датотеките");
        m.insert("Remove image", "Отстрани ја фотографијата");
        m.insert("Either png or jpg. Ideally square but you will be able to crop it.", "Мора де биде png или jpg. Идеално квадрат, но ќе бидете во можност да ја исечете.");
        m.insert("Abort", "Прекини");
        m.insert("Choose as profile image", "Одбери фотографија за профилот");
        m.insert("Language", "Јазик");
        m.insert("Help translate", "Помогни во преводот");
        m.insert("WebDAV", "WebDAV");
        m.insert("Encryption", "Енкрипција");
        m.insert("Log-in password", "Лозинка за најавување");
        m.insert("Decrypt all Files", "Дешифрирај ги сите датотеки");
        m.insert("Login Name", "Име за најава");
        m.insert("Create", "Создај");
        m.insert("Default Storage", "Предефинирано складиште ");
        m.insert("Unlimited", "Неограничено");
        m.insert("Other", "Останато");
        m.insert("Username", "Корисничко име");
        m.insert("Storage", "Складиште");
        m.insert("set new password", "постави нова лозинка");
        m.insert("Default", "Предефиниран");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}