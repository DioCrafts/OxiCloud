use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "Недеља");
    translations.insert("Monday", "Понедељак");
    translations.insert("Tuesday", "Уторак");
    translations.insert("Wednesday", "Среда");
    translations.insert("Thursday", "Четвртак");
    translations.insert("Friday", "Петак");
    translations.insert("Saturday", "Субота");
    translations.insert("January", "Јануар");
    translations.insert("February", "Фебруар");
    translations.insert("March", "Март");
    translations.insert("April", "Април");
    translations.insert("May", "Мај");
    translations.insert("June", "Јун");
    translations.insert("July", "Јул");
    translations.insert("August", "Август");
    translations.insert("September", "Септембар");
    translations.insert("October", "Октобар");
    translations.insert("November", "Новембар");
    translations.insert("December", "Децембар");
    translations.insert("Settings", "Поставке");
    translations.insert("seconds ago", "пре неколико секунди");
    translations.insert("today", "данас");
    translations.insert("yesterday", "јуче");
    translations.insert("last month", "прошлог месеца");
    translations.insert("months ago", "месеци раније");
    translations.insert("last year", "прошле године");
    translations.insert("years ago", "година раније");
    translations.insert("Choose", "Одабери");
    translations.insert("Yes", "Да");
    translations.insert("No", "Не");
    translations.insert("Ok", "У реду");
    translations.insert("Cancel", "Откажи");
    translations.insert("Share", "Дели");
    translations.insert("Error", "Грешка");
    translations.insert("Error while sharing", "Грешка у дељењу");
    translations.insert("Error while unsharing", "Грешка код искључења дељења");
    translations.insert("Error while changing permissions", "Грешка код промене дозвола");
    translations.insert("Shared with you and the group {group} by {owner}", "Дељено са вама и са групом {group}. Поделио {owner}.");
    translations.insert("Shared with you by {owner}", "Поделио са вама {owner}");
    translations.insert("Password protect", "Заштићено лозинком");
    translations.insert("Password", "Лозинка");
    translations.insert("Send", "Пошаљи");
    translations.insert("Set expiration date", "Постави датум истека");
    translations.insert("Expiration date", "Датум истека");
    translations.insert("Share via email:", "Подели поштом:");
    translations.insert("No people found", "Особе нису пронађене.");
    translations.insert("group", "група");
    translations.insert("Resharing is not allowed", "Поновно дељење није дозвољено");
    translations.insert("Shared in {item} with {user}", "Подељено унутар {item} са {user}");
    translations.insert("Unshare", "Укини дељење");
    translations.insert("can edit", "може да мења");
    translations.insert("access control", "права приступа");
    translations.insert("create", "направи");
    translations.insert("update", "ажурирај");
    translations.insert("delete", "обриши");
    translations.insert("share", "подели");
    translations.insert("Password protected", "Заштићено лозинком");
    translations.insert("Error unsetting expiration date", "Грешка код поништавања датума истека");
    translations.insert("Error setting expiration date", "Грешка код постављања датума истека");
    translations.insert("Sending ...", "Шаљем...");
    translations.insert("Email sent", "Порука је послата");
    translations.insert("Warning", "Упозорење");
    translations.insert("The object type is not specified.", "Врста објекта није подешена.");
    translations.insert("Delete", "Обриши");
    translations.insert("Add", "Додај");
    translations.insert("Use the following link to reset your password: {link}", "Овом везом ресетујте своју лозинку: {link}");
    translations.insert("You will receive a link to reset your password via Email.", "Добићете везу за ресетовање лозинке путем е-поште.");
    translations.insert("Username", "Корисничко име");
    translations.insert("Your password was reset", "Ваша лозинка је ресетована");
    translations.insert("To login page", "На страницу за пријаву");
    translations.insert("New password", "Нова лозинка");
    translations.insert("Reset password", "Ресетуј лозинку");
    translations.insert("Personal", "Лично");
    translations.insert("Users", "Корисници");
    translations.insert("Apps", "Апликације");
    translations.insert("Admin", "Администратор");
    translations.insert("Help", "Помоћ");
    translations.insert("Access forbidden", "Забрањен приступ");
    translations.insert("Cloud not found", "Облак није нађен");
    translations.insert("Security Warning", "Сигурносно упозорење");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Поуздан генератор случајних бројева није доступан, предлажемо да укључите PHP проширење OpenSSL.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Без поузданог генератора случајнох бројева нападач лако може предвидети лозинку за поништавање кључа шифровања и отети вам налог.");
    translations.insert("Create an <strong>admin account</strong>", "Направи <strong>административни налог</strong>");
    translations.insert("Advanced", "Напредно");
    translations.insert("Data folder", "Фацикла података");
    translations.insert("Configure the database", "Подешавање базе");
    translations.insert("will be used", "ће бити коришћен");
    translations.insert("Database user", "Корисник базе");
    translations.insert("Database password", "Лозинка базе");
    translations.insert("Database name", "Име базе");
    translations.insert("Database tablespace", "Радни простор базе података");
    translations.insert("Database host", "Домаћин базе");
    translations.insert("Finish setup", "Заврши подешавање");
    translations.insert("Log out", "Одјава");
    translations.insert("Automatic logon rejected!", "Аутоматска пријава је одбијена!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "Ако ускоро не промените лозинку ваш налог може бити компромитован!");
    translations.insert("Please change your password to secure your account again.", "Промените лозинку да бисте обезбедили налог.");
    translations.insert("Lost your password?", "Изгубили сте лозинку?");
    translations.insert("remember", "упамти");
    translations.insert("Log in", "Пријава");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Надоградња ownCloud-а на верзију %s, сачекајте тренутак.");
    translations
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

// Mapa para plurales
pub static PLURALS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plurals = HashMap::new();
    plurals.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", ""]);
    plurals.insert("_%n hour ago_::_%n hours ago_", vec!["", "", ""]);
    plurals.insert("_%n day ago_::_%n days ago_", vec!["", "", ""]);
    plurals.insert("_%n month ago_::_%n months ago_", vec!["", "", ""]);
    plurals.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", "", ""]);
    plurals
});

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form(key: &str, count: usize) -> Option<&'static str> {
    PLURALS.get(key).and_then(|forms| {
        // Calcular el índice de la forma plural según la regla
        let n = count as i64;
        let idx = if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        };
        
        forms.get(idx).copied()
    })
}