use std::collections::HashMap;
use phf::phf_map;

pub static LV_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "%s shared »%s« with you" => "%s kopīgots »%s« ar jums",
    "Sunday" => "Svētdiena",
    "Monday" => "Pirmdiena",
    "Tuesday" => "Otrdiena",
    "Wednesday" => "Trešdiena",
    "Thursday" => "Ceturtdiena",
    "Friday" => "Piektdiena",
    "Saturday" => "Sestdiena",
    "January" => "Janvāris",
    "February" => "Februāris",
    "March" => "Marts",
    "April" => "Aprīlis",
    "May" => "Maijs",
    "June" => "Jūnijs",
    "July" => "Jūlijs",
    "August" => "Augusts",
    "September" => "Septembris",
    "October" => "Oktobris",
    "November" => "Novembris",
    "December" => "Decembris",
    "Settings" => "Iestatījumi",
    "seconds ago" => "sekundes atpakaļ",
    "today" => "šodien",
    "yesterday" => "vakar",
    "last month" => "pagājušajā mēnesī",
    "months ago" => "mēnešus atpakaļ",
    "last year" => "gājušajā gadā",
    "years ago" => "gadus atpakaļ",
    "Choose" => "Izvēlieties",
    "Yes" => "Jā",
    "No" => "Nē",
    "Ok" => "Labi",
    "Cancel" => "Atcelt",
    "Shared" => "Kopīgs",
    "Share" => "Dalīties",
    "Error" => "Kļūda",
    "Error while sharing" => "Kļūda, daloties",
    "Error while unsharing" => "Kļūda, beidzot dalīties",
    "Error while changing permissions" => "Kļūda, mainot atļaujas",
    "Shared with you and the group {group} by {owner}" => "{owner} dalījās ar jums un grupu {group}",
    "Shared with you by {owner}" => "{owner} dalījās ar jums",
    "Password protect" => "Aizsargāt ar paroli",
    "Password" => "Parole",
    "Allow Public Upload" => "Ļaut publisko augšupielādi.",
    "Email link to person" => "Sūtīt saiti personai pa e-pastu",
    "Send" => "Sūtīt",
    "Set expiration date" => "Iestaties termiņa datumu",
    "Expiration date" => "Termiņa datums",
    "Share via email:" => "Dalīties, izmantojot e-pastu:",
    "No people found" => "Nav atrastu cilvēku",
    "group" => "grupa",
    "Resharing is not allowed" => "Atkārtota dalīšanās nav atļauta",
    "Shared in {item} with {user}" => "Dalījās ar {item} ar {user}",
    "Unshare" => "Pārtraukt dalīšanos",
    "can edit" => "var rediģēt",
    "access control" => "piekļuves vadība",
    "create" => "izveidot",
    "update" => "atjaunināt",
    "delete" => "dzēst",
    "share" => "dalīties",
    "Password protected" => "Aizsargāts ar paroli",
    "Error unsetting expiration date" => "Kļūda, noņemot termiņa datumu",
    "Error setting expiration date" => "Kļūda, iestatot termiņa datumu",
    "Sending ..." => "Sūta...",
    "Email sent" => "Vēstule nosūtīta",
    "Warning" => "Brīdinājums",
    "The object type is not specified." => "Nav norādīts objekta tips.",
    "Delete" => "Dzēst",
    "Add" => "Pievienot",
    "The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>." => "Atjaunināšana beidzās nesekmīgi. Lūdzu, ziņojiet par šo problēmu <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud kopienai</a>.",
    "The update was successful. Redirecting you to ownCloud now." => "Atjaunināšana beidzās sekmīgi. Tagad pārsūta jūs uz ownCloud.",
    "%s password reset" => "%s paroles maiņa",
    "Use the following link to reset your password: {link}" => "Izmantojiet šo saiti, lai mainītu paroli: {link}",
    "The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator ." => "Saite uz paroles atjaunošanas vietu ir nosūtīta uz epastu.<br>Ja vēstu nav atnākusi, pārbaudiet epasta mēstuļu mapi.<br>Jā tās tur nav, jautājiet sistēmas administratoram.",
    "Request failed!<br>Did you make sure your email/username was right?" => "Pieprasījums neizdevās!<br>Vai Jūs pārliecinājāties ka epasts/lietotājvārds ir pareizi?",
    "You will receive a link to reset your password via Email." => "Jūs savā epastā saņemsiet interneta saiti, caur kuru varēsiet atjaunot paroli.",
    "Username" => "Lietotājvārds",
    "Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?" => "Jūsu faili ir šifrēti. Ja nav iespējota atgūšanas kods, tad nebūs iespēja atjaunot jūsu failus pēc tam kad tiks mainīta parole. ja neesat pārliecināts kā rīkoties, jautājiet administratoram. Vai tiešam vēlaties turpināt?",
    "Yes, I really want to reset my password now" => "Jā, Es tiešām vēlos mainīt savu paroli",
    "Your password was reset" => "Jūsu parole tika nomainīta",
    "To login page" => "Uz ielogošanās lapu",
    "New password" => "Jauna parole",
    "Reset password" => "Mainīt paroli",
    "Personal" => "Personīgi",
    "Users" => "Lietotāji",
    "Apps" => "Lietotnes",
    "Admin" => "Administratori",
    "Help" => "Palīdzība",
    "Access forbidden" => "Pieeja ir liegta",
    "Cloud not found" => "Mākonis netika atrasts",
    "Security Warning" => "Brīdinājums par drošību",
    "Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)" => "Jūsu PHP ir ievainojamība pret NULL Byte uzbrukumiem (CVE-2006-7243)",
    "Please update your PHP installation to use %s securely." => "Lūdzu atjauniniet PHP instalāciju lai varētu droši izmantot %s.",
    "No secure random number generator is available, please enable the PHP OpenSSL extension." => "Nav pieejams drošs nejaušu skaitļu ģenerators. Lūdzu, aktivējiet PHP OpenSSL paplašinājumu.",
    "Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account." => "Bez droša nejaušu skaitļu ģeneratora uzbrucējs var paredzēt paroļu atjaunošanas marķierus un pārņem jūsu kontu.",
    "Your data directory and files are probably accessible from the internet because the .htaccess file does not work." => "Visticamāk, jūsu datu direktorija un datnes ir pieejamas no interneta, jo .htaccess datne nedarbojas.",
    "For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>." => "Vairāk informācijai kā konfigurēt serveri, lūdzu skatiet <a href=\"%s\" target=\"_blank\">dokumentāciju</a>.",
    "Create an <strong>admin account</strong>" => "Izveidot <strong>administratora kontu</strong>",
    "Advanced" => "Paplašināti",
    "Data folder" => "Datu mape",
    "Configure the database" => "Konfigurēt datubāzi",
    "will be used" => "tiks izmantots",
    "Database user" => "Datubāzes lietotājs",
    "Database password" => "Datubāzes parole",
    "Database name" => "Datubāzes nosaukums",
    "Database tablespace" => "Datubāzes tabulas telpa",
    "Database host" => "Datubāzes serveris",
    "Finish setup" => "Pabeigt iestatīšanu",
    "%s is available. Get more information on how to update." => "%s ir pieejams. Uzziniet vairāk kā atjaunināt.",
    "Log out" => "Izrakstīties",
    "Automatic logon rejected!" => "Automātiskā ierakstīšanās ir noraidīta!",
    "If you did not change your password recently, your account may be compromised!" => "Ja neesat pēdējā laikā mainījis paroli, iespējams, ka jūsu konts ir kompromitēts.",
    "Please change your password to secure your account again." => "Lūdzu, nomainiet savu paroli, lai atkal nodrošinātu savu kontu.",
    "Lost your password?" => "Aizmirsāt paroli?",
    "remember" => "atcerēties",
    "Log in" => "Ierakstīties",
    "Alternative Logins" => "Alternatīvās pieteikšanās",
    "Updating ownCloud to version %s, this may take a while." => "Atjaunina ownCloud uz versiju %s. Tas var aizņemt kādu laiciņu."
};

pub static LV_PLURAL_TRANSLATIONS: phf::Map<&'static str, [&'static str; 3]> = phf_map! {
    "_%n minute ago_::_%n minutes ago_" => ["Tagad, %n minūtes", "Pirms %n minūtes", "Pirms %n minūtēm"],
    "_%n hour ago_::_%n hours ago_" => ["Šodien, %n stundas", "Pirms %n stundas", "Pirms %n stundām"],
    "_%n day ago_::_%n days ago_" => ["Šodien, %n dienas", "Pirms %n dienas", "Pirms %n dienām"],
    "_%n month ago_::_%n months ago_" => ["Šomēnes, %n mēneši", "Pirms %n mēneša", "Pirms %n mēnešiem"],
    "_{count} file conflict_::_{count} file conflicts_" => ["", "", ""]
};

// Defines the plural form rule for Latvian
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n != 0 {
        1
    } else {
        2
    }
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    LV_TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: i64) -> Option<&'static str> {
    LV_PLURAL_TRANSLATIONS.get(key).map(|forms| {
        let index = get_plural_form(count);
        forms[index]
    })
}

pub fn format_translation(translation: &str, args: &[&str]) -> String {
    let mut result = translation.to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%{}", i + 1), arg);
        result = result.replace(&format!("%s"), arg); // Only replaces the first occurrence
    }
    result
}