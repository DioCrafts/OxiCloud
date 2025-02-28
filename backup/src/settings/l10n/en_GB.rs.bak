use std::collections::HashMap;

pub struct EnGB {
    translations: HashMap<&'static str, &'static str>,
    plural_forms: &'static str,
}

impl EnGB {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // Populate the translations HashMap
        translations.insert("Unable to load list from App Store", "Unable to load list from App Store");
        translations.insert("Authentication error", "Authentication error");
        translations.insert("Group already exists", "Group already exists");
        translations.insert("Unable to add group", "Unable to add group");
        translations.insert("Email saved", "Email saved");
        translations.insert("Invalid email", "Invalid email");
        translations.insert("Unable to delete group", "Unable to delete group");
        translations.insert("Unable to delete user", "Unable to delete user");
        translations.insert("Language changed", "Language changed");
        translations.insert("Invalid request", "Invalid request");
        translations.insert("Admins can't remove themself from the admin group", "Admins can't remove themselves from the admin group");
        translations.insert("Unable to add user to group %s", "Unable to add user to group %s");
        translations.insert("Unable to remove user from group %s", "Unable to remove user from group %s");
        translations.insert("Couldn't update app.", "Couldn't update app.");
        translations.insert("Wrong password", "Incorrect password");
        translations.insert("No user supplied", "No user supplied");
        translations.insert("Please provide an admin recovery password, otherwise all user data will be lost", "Please provide an admin recovery password, otherwise all user data will be lost");
        translations.insert("Wrong admin recovery password. Please check the password and try again.", "Incorrect admin recovery password. Please check the password and try again.");
        translations.insert("Back-end doesn't support password change, but the users encryption key was successfully updated.", "Back-end doesn't support password change, but the user's encryption key was successfully updated.");
        translations.insert("Unable to change password", "Unable to change password");
        translations.insert("Update to {appversion}", "Update to {appversion}");
        translations.insert("Disable", "Disable");
        translations.insert("Enable", "Enable");
        translations.insert("Please wait....", "Please wait....");
        translations.insert("Error while disabling app", "Error whilst disabling app");
        translations.insert("Error while enabling app", "Error whilst enabling app");
        translations.insert("Updating....", "Updating....");
        translations.insert("Error while updating app", "Error whilst updating app");
        translations.insert("Error", "Error");
        translations.insert("Update", "Update");
        translations.insert("Updated", "Updated");
        translations.insert("Select a profile picture", "Select a profile picture");
        translations.insert("Decrypting files... Please wait, this can take some time.", "Decrypting files... Please wait, this can take some time.");
        translations.insert("Saving...", "Saving...");
        translations.insert("deleted", "deleted");
        translations.insert("undo", "undo");
        translations.insert("Unable to remove user", "Unable to remove user");
        translations.insert("Groups", "Groups");
        translations.insert("Group Admin", "Group Admin");
        translations.insert("Delete", "Delete");
        translations.insert("add group", "add group");
        translations.insert("A valid username must be provided", "A valid username must be provided");
        translations.insert("Error creating user", "Error creating user");
        translations.insert("A valid password must be provided", "A valid password must be provided");
        translations.insert("Warning: Home directory for user \"{user}\" already exists", "Warning: Home directory for user \"{user}\" already exists");
        translations.insert("__language_name__", "__language_name__");
        translations.insert("Everything (fatal issues, errors, warnings, info, debug)", "Everything (fatal issues, errors, warnings, info, debug)");
        translations.insert("Info, warnings, errors and fatal issues", "Info, warnings, errors and fatal issues");
        translations.insert("Warnings, errors and fatal issues", "Warnings, errors and fatal issues");
        translations.insert("Errors and fatal issues", "Errors and fatal issues");
        translations.insert("Fatal issues only", "Fatal issues only");
        translations.insert("Security Warning", "Security Warning");
        translations.insert("Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.", "Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.");
        translations.insert("Setup Warning", "Setup Warning");
        translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Your web server is not yet properly setup to allow files synchronisation because the WebDAV interface seems to be broken.");
        translations.insert("Please double check the <a href=\"%s\">installation guides</a>.", "Please double check the <a href=\"%s\">installation guides</a>.");
        translations.insert("Module 'fileinfo' missing", "Module 'fileinfo' missing");
        translations.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "The PHP module 'fileinfo' is missing. We strongly recommend enabling this module to get best results with mime-type detection.");
        translations.insert("Locale not working", "Locale not working");
        translations.insert("System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.", "System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.");
        translations.insert("Internet connection not working", "Internet connection not working");
        translations.insert("This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don´t work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.", "This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don't work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.");
        translations.insert("Cron", "Cron");
        translations.insert("Execute one task with each page loaded", "Execute one task with each page loaded");
        translations.insert("cron.php is registered at a webcron service to call cron.php every 15 minutes over http.", "cron.php is registered at a webcron service to call cron.php every 15 minutes over http.");
        translations.insert("Use systems cron service to call the cron.php file every 15 minutes.", "Use system's cron service to call the cron.php file every 15 minutes.");
        translations.insert("Sharing", "Sharing");
        translations.insert("Enable Share API", "Enable Share API");
        translations.insert("Allow apps to use the Share API", "Allow apps to use the Share API");
        translations.insert("Allow links", "Allow links");
        translations.insert("Allow users to share items to the public with links", "Allow users to share items to the public with links");
        translations.insert("Allow public uploads", "Allow public uploads");
        translations.insert("Allow users to enable others to upload into their publicly shared folders", "Allow users to enable others to upload into their publicly shared folders");
        translations.insert("Allow resharing", "Allow resharing");
        translations.insert("Allow users to share items shared with them again", "Allow users to share items shared with them again");
        translations.insert("Allow users to share with anyone", "Allow users to share with anyone");
        translations.insert("Allow users to only share with users in their groups", "Allow users to only share with users in their groups");
        translations.insert("Allow mail notification", "Allow mail notification");
        translations.insert("Allow user to send mail notification for shared files", "Allow user to send mail notification for shared files");
        translations.insert("Security", "Security");
        translations.insert("Enforce HTTPS", "Enforce HTTPS");
        translations.insert("Forces the clients to connect to %s via an encrypted connection.", "Forces the clients to connect to %s via an encrypted connection.");
        translations.insert("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.", "Please connect to your %s via HTTPS to enable or disable the SSL enforcement.");
        translations.insert("Log", "Log");
        translations.insert("Log level", "Log level");
        translations.insert("More", "More");
        translations.insert("Less", "Less");
        translations.insert("Version", "Version");
        translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public Licence\">AGPL</abbr></a>.");
        translations.insert("Add your App", "Add your App");
        translations.insert("More Apps", "More Apps");
        translations.insert("Select an App", "Select an App");
        translations.insert("See application page at apps.owncloud.com", "See application page at apps.owncloud.com");
        translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>");
        translations.insert("User Documentation", "User Documentation");
        translations.insert("Administrator Documentation", "Administrator Documentation");
        translations.insert("Online Documentation", "Online Documentation");
        translations.insert("Forum", "Forum");
        translations.insert("Bugtracker", "Bugtracker");
        translations.insert("Commercial Support", "Commercial Support");
        translations.insert("Get the apps to sync your files", "Get the apps to sync your files");
        translations.insert("Show First Run Wizard again", "Show First Run Wizard again");
        translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "You have used <strong>%s</strong> of the available <strong>%s</strong>");
        translations.insert("Password", "Password");
        translations.insert("Your password was changed", "Your password was changed");
        translations.insert("Unable to change your password", "Unable to change your password");
        translations.insert("Current password", "Current password");
        translations.insert("New password", "New password");
        translations.insert("Change password", "Change password");
        translations.insert("Email", "Email");
        translations.insert("Your email address", "Your email address");
        translations.insert("Fill in an email address to enable password recovery", "Fill in an email address to enable password recovery");
        translations.insert("Profile picture", "Profile picture");
        translations.insert("Upload new", "Upload new");
        translations.insert("Select new from Files", "Select new from Files");
        translations.insert("Remove image", "Remove image");
        translations.insert("Either png or jpg. Ideally square but you will be able to crop it.", "Either png or jpg. Ideally square but you will be able to crop it.");
        translations.insert("Abort", "Abort");
        translations.insert("Choose as profile image", "Choose as profile image");
        translations.insert("Language", "Language");
        translations.insert("Help translate", "Help translate");
        translations.insert("WebDAV", "WebDAV");
        translations.insert("Use this address to <a href=\"%s\" target=\"_blank\">access your Files via WebDAV</a>", "Use this address to <a href=\"%s\" target=\"_blank\">access your Files via WebDAV</a>");
        translations.insert("Encryption", "Encryption");
        translations.insert("The encryption app is no longer enabled, decrypt all your file", "The encryption app is no longer enabled, decrypt all your files");
        translations.insert("Log-in password", "Log-in password");
        translations.insert("Decrypt all Files", "Decrypt all Files");
        translations.insert("Login Name", "Login Name");
        translations.insert("Create", "Create");
        translations.insert("Admin Recovery Password", "Admin Recovery Password");
        translations.insert("Enter the recovery password in order to recover the users files during password change", "Enter the recovery password in order to recover the user's files during password change");
        translations.insert("Default Storage", "Default Storage");
        translations.insert("Please enter storage quota (ex: \"512 MB\" or \"12 GB\")", "Please enter storage quota (e.g. \"512 MB\" or \"12 GB\")");
        translations.insert("Unlimited", "Unlimited");
        translations.insert("Other", "Other");
        translations.insert("Username", "Username");
        translations.insert("Storage", "Storage");
        translations.insert("set new password", "set new password");
        translations.insert("Default", "Default");

        EnGB {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
        }
    }

    pub fn get_translation(&self, key: &str) -> Option<&'static str> {
        self.translations.get(key).copied()
    }

    pub fn get_plural_forms(&self) -> &'static str {
        self.plural_forms
    }
}