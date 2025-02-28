use std::collections::HashMap;
use rust_i18n::i18n;

// English (UK) translations
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("%s shared »%s« with you".to_string(), "%s shared \"%s\" with you".to_string());
    translations.insert("Couldn't send mail to following users: %s ".to_string(), "Couldn't send mail to following users: %s ".to_string());
    translations.insert("Turned on maintenance mode".to_string(), "Turned on maintenance mode".to_string());
    translations.insert("Turned off maintenance mode".to_string(), "Turned off maintenance mode".to_string());
    translations.insert("Updated database".to_string(), "Updated database".to_string());
    translations.insert("Updating filecache, this may take really long...".to_string(), "Updating filecache, this may take a really long time...".to_string());
    translations.insert("Updated filecache".to_string(), "Updated filecache".to_string());
    translations.insert("... %d%% done ...".to_string(), "... %d%% done ...".to_string());
    translations.insert("No image or file provided".to_string(), "No image or file provided".to_string());
    translations.insert("Unknown filetype".to_string(), "Unknown filetype".to_string());
    translations.insert("Invalid image".to_string(), "Invalid image".to_string());
    translations.insert("No temporary profile picture available, try again".to_string(), "No temporary profile picture available, try again".to_string());
    translations.insert("No crop data provided".to_string(), "No crop data provided".to_string());
    translations.insert("Sunday".to_string(), "Sunday".to_string());
    translations.insert("Monday".to_string(), "Monday".to_string());
    translations.insert("Tuesday".to_string(), "Tuesday".to_string());
    translations.insert("Wednesday".to_string(), "Wednesday".to_string());
    translations.insert("Thursday".to_string(), "Thursday".to_string());
    translations.insert("Friday".to_string(), "Friday".to_string());
    translations.insert("Saturday".to_string(), "Saturday".to_string());
    translations.insert("January".to_string(), "January".to_string());
    translations.insert("February".to_string(), "February".to_string());
    translations.insert("March".to_string(), "March".to_string());
    translations.insert("April".to_string(), "April".to_string());
    translations.insert("May".to_string(), "May".to_string());
    translations.insert("June".to_string(), "June".to_string());
    translations.insert("July".to_string(), "July".to_string());
    translations.insert("August".to_string(), "August".to_string());
    translations.insert("September".to_string(), "September".to_string());
    translations.insert("October".to_string(), "October".to_string());
    translations.insert("November".to_string(), "November".to_string());
    translations.insert("December".to_string(), "December".to_string());
    translations.insert("Settings".to_string(), "Settings".to_string());
    translations.insert("seconds ago".to_string(), "seconds ago".to_string());
    translations.insert("today".to_string(), "today".to_string());
    translations.insert("yesterday".to_string(), "yesterday".to_string());
    translations.insert("last month".to_string(), "last month".to_string());
    translations.insert("months ago".to_string(), "months ago".to_string());
    translations.insert("last year".to_string(), "last year".to_string());
    translations.insert("years ago".to_string(), "years ago".to_string());
    translations.insert("Choose".to_string(), "Choose".to_string());
    translations.insert("Error loading file picker template: {error}".to_string(), "Error loading file picker template: {error}".to_string());
    translations.insert("Yes".to_string(), "Yes".to_string());
    translations.insert("No".to_string(), "No".to_string());
    translations.insert("Ok".to_string(), "OK".to_string());
    translations.insert("Error loading message template: {error}".to_string(), "Error loading message template: {error}".to_string());
    translations.insert("One file conflict".to_string(), "One file conflict".to_string());
    translations.insert("Which files do you want to keep?".to_string(), "Which files do you wish to keep?".to_string());
    translations.insert("If you select both versions, the copied file will have a number added to its name.".to_string(), "If you select both versions, the copied file will have a number added to its name.".to_string());
    translations.insert("Cancel".to_string(), "Cancel".to_string());
    translations.insert("Continue".to_string(), "Continue".to_string());
    translations.insert("(all selected)".to_string(), "(all selected)".to_string());
    translations.insert("({count} selected)".to_string(), "({count} selected)".to_string());
    translations.insert("Error loading file exists template".to_string(), "Error loading file exists template".to_string());
    translations.insert("Shared".to_string(), "Shared".to_string());
    translations.insert("Share".to_string(), "Share".to_string());
    translations.insert("Error".to_string(), "Error".to_string());
    translations.insert("Error while sharing".to_string(), "Error whilst sharing".to_string());
    translations.insert("Error while unsharing".to_string(), "Error whilst unsharing".to_string());
    translations.insert("Error while changing permissions".to_string(), "Error whilst changing permissions".to_string());
    translations.insert("Shared with you and the group {group} by {owner}".to_string(), "Shared with you and the group {group} by {owner}".to_string());
    translations.insert("Shared with you by {owner}".to_string(), "Shared with you by {owner}".to_string());
    translations.insert("Share with user or group …".to_string(), "Share with user or group …".to_string());
    translations.insert("Share link".to_string(), "Share link".to_string());
    translations.insert("Password protect".to_string(), "Password protect".to_string());
    translations.insert("Password".to_string(), "Password".to_string());
    translations.insert("Allow Public Upload".to_string(), "Allow Public Upload".to_string());
    translations.insert("Email link to person".to_string(), "Email link to person".to_string());
    translations.insert("Send".to_string(), "Send".to_string());
    translations.insert("Set expiration date".to_string(), "Set expiration date".to_string());
    translations.insert("Expiration date".to_string(), "Expiration date".to_string());
    translations.insert("Share via email:".to_string(), "Share via email:".to_string());
    translations.insert("No people found".to_string(), "No people found".to_string());
    translations.insert("group".to_string(), "group".to_string());
    translations.insert("Resharing is not allowed".to_string(), "Resharing is not allowed".to_string());
    translations.insert("Shared in {item} with {user}".to_string(), "Shared in {item} with {user}".to_string());
    translations.insert("Unshare".to_string(), "Unshare".to_string());
    translations.insert("notify by email".to_string(), "notify by email".to_string());
    translations.insert("can edit".to_string(), "can edit".to_string());
    translations.insert("access control".to_string(), "access control".to_string());
    translations.insert("create".to_string(), "create".to_string());
    translations.insert("update".to_string(), "update".to_string());
    translations.insert("delete".to_string(), "delete".to_string());
    translations.insert("share".to_string(), "share".to_string());
    translations.insert("Password protected".to_string(), "Password protected".to_string());
    translations.insert("Error unsetting expiration date".to_string(), "Error unsetting expiration date".to_string());
    translations.insert("Error setting expiration date".to_string(), "Error setting expiration date".to_string());
    translations.insert("Sending ...".to_string(), "Sending ...".to_string());
    translations.insert("Email sent".to_string(), "Email sent".to_string());
    translations.insert("Warning".to_string(), "Warning".to_string());
    translations.insert("The object type is not specified.".to_string(), "The object type is not specified.".to_string());
    translations.insert("Enter new".to_string(), "Enter new".to_string());
    translations.insert("Delete".to_string(), "Delete".to_string());
    translations.insert("Add".to_string(), "Add".to_string());
    translations.insert("Edit tags".to_string(), "Edit tags".to_string());
    translations.insert("Error loading dialog template: {error}".to_string(), "Error loading dialog template: {error}".to_string());
    translations.insert("No tags selected for deletion.".to_string(), "No tags selected for deletion.".to_string());
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.".to_string(), "The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.".to_string());
    translations.insert("The update was successful. Redirecting you to ownCloud now.".to_string(), "The update was successful. Redirecting you to ownCloud now.".to_string());
    translations.insert("%s password reset".to_string(), "%s password reset".to_string());
    translations.insert("Use the following link to reset your password: {link}".to_string(), "Use the following link to reset your password: {link}".to_string());
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .".to_string(), "The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .".to_string());
    translations.insert("Request failed!<br>Did you make sure your email/username was right?".to_string(), "Request failed!<br>Did you make sure your email/username was correct?".to_string());
    translations.insert("You will receive a link to reset your password via Email.".to_string(), "You will receive a link to reset your password via email.".to_string());
    translations.insert("Username".to_string(), "Username".to_string());
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?".to_string(), "Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?".to_string());
    translations.insert("Yes, I really want to reset my password now".to_string(), "Yes, I really want to reset my password now".to_string());
    translations.insert("Reset".to_string(), "Reset".to_string());
    translations.insert("Your password was reset".to_string(), "Your password was reset".to_string());
    translations.insert("To login page".to_string(), "To login page".to_string());
    translations.insert("New password".to_string(), "New password".to_string());
    translations.insert("Reset password".to_string(), "Reset password".to_string());
    translations.insert("Personal".to_string(), "Personal".to_string());
    translations.insert("Users".to_string(), "Users".to_string());
    translations.insert("Apps".to_string(), "Apps".to_string());
    translations.insert("Admin".to_string(), "Admin".to_string());
    translations.insert("Help".to_string(), "Help".to_string());
    translations.insert("Error loading tags".to_string(), "Error loading tags".to_string());
    translations.insert("Tag already exists".to_string(), "Tag already exists".to_string());
    translations.insert("Error deleting tag(s)".to_string(), "Error deleting tag(s)".to_string());
    translations.insert("Error tagging".to_string(), "Error tagging".to_string());
    translations.insert("Error untagging".to_string(), "Error untagging".to_string());
    translations.insert("Error favoriting".to_string(), "Error favouriting".to_string());
    translations.insert("Error unfavoriting".to_string(), "Error unfavouriting".to_string());
    translations.insert("Access forbidden".to_string(), "Access denied".to_string());
    translations.insert("Cloud not found".to_string(), "Cloud not found".to_string());
    translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n".to_string(), "Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n".to_string());
    translations.insert("The share will expire on %s.\n\n".to_string(), "The share will expire on %s.\n\n".to_string());
    translations.insert("Cheers!".to_string(), "Cheers!".to_string());
    translations.insert("Security Warning".to_string(), "Security Warning".to_string());
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)".to_string(), "Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)".to_string());
    translations.insert("Please update your PHP installation to use %s securely.".to_string(), "Please update your PHP installation to use %s securely.".to_string());
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.".to_string(), "No secure random number generator is available, please enable the PHP OpenSSL extension.".to_string());
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.".to_string(), "Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.".to_string());
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.".to_string(), "Your data directory and files are probably accessible from the internet because the .htaccess file does not work.".to_string());
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.".to_string(), "For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.".to_string());
    translations.insert("Create an <strong>admin account</strong>".to_string(), "Create an <strong>admin account</strong>".to_string());
    translations.insert("Advanced".to_string(), "Advanced".to_string());
    translations.insert("Data folder".to_string(), "Data folder".to_string());
    translations.insert("Configure the database".to_string(), "Configure the database".to_string());
    translations.insert("will be used".to_string(), "will be used".to_string());
    translations.insert("Database user".to_string(), "Database user".to_string());
    translations.insert("Database password".to_string(), "Database password".to_string());
    translations.insert("Database name".to_string(), "Database name".to_string());
    translations.insert("Database tablespace".to_string(), "Database tablespace".to_string());
    translations.insert("Database host".to_string(), "Database host".to_string());
    translations.insert("Finish setup".to_string(), "Finish setup".to_string());
    translations.insert("Finishing …".to_string(), "Finishing …".to_string());
    translations.insert("%s is available. Get more information on how to update.".to_string(), "%s is available. Get more information on how to update.".to_string());
    translations.insert("Log out".to_string(), "Log out".to_string());
    translations.insert("Automatic logon rejected!".to_string(), "Automatic logon rejected!".to_string());
    translations.insert("If you did not change your password recently, your account may be compromised!".to_string(), "If you did not change your password recently, your account may be compromised!".to_string());
    translations.insert("Please change your password to secure your account again.".to_string(), "Please change your password to secure your account again.".to_string());
    translations.insert("Server side authentication failed!".to_string(), "Server side authentication failed!".to_string());
    translations.insert("Please contact your administrator.".to_string(), "Please contact your administrator.".to_string());
    translations.insert("Lost your password?".to_string(), "Lost your password?".to_string());
    translations.insert("remember".to_string(), "remember".to_string());
    translations.insert("Log in".to_string(), "Log in".to_string());
    translations.insert("Alternative Logins".to_string(), "Alternative Logins".to_string());
    translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>".to_string(), "Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>".to_string());
    translations.insert("The share will expire on %s.<br><br>".to_string(), "The share will expire on %s.<br><br>".to_string());
    translations.insert("Updating ownCloud to version %s, this may take a while.".to_string(), "Updating ownCloud to version %s, this may take a while.".to_string());
    translations.insert("This ownCloud instance is currently being updated, which may take a while.".to_string(), "This ownCloud instance is currently being updated, which may take a while.".to_string());
    translations.insert("Please reload this page after a short time to continue using ownCloud.".to_string(), "Please reload this page after a short time to continue using ownCloud.".to_string());
    translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.".to_string(), "Contact your system administrator if this message persists or appeared unexpectedly.".to_string());
    translations.insert("Thank you for your patience.".to_string(), "Thank you for your patience.".to_string());
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Función para manejar traducciones plurales
pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(), 
        vec!["%n minute ago".to_string(), "%n minutes ago".to_string()]
    );
    
    plural_translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(), 
        vec!["%n hour ago".to_string(), "%n hours ago".to_string()]
    );
    
    plural_translations.insert(
        "_%n day ago_::_%n days ago_".to_string(), 
        vec!["%n day ago".to_string(), "%n days ago".to_string()]
    );
    
    plural_translations.insert(
        "_%n month ago_::_%n months ago_".to_string(), 
        vec!["%n month ago".to_string(), "%n months ago".to_string()]
    );
    
    plural_translations.insert(
        "_{count} file conflict_::_{count} file conflicts_".to_string(), 
        vec!["{count} file conflict".to_string(), "{count} file conflicts".to_string()]
    );
    
    plural_translations
}