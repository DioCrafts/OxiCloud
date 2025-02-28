use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Recovery key enabled successfully");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Could not enable recovery key. Please check your recovery key password!");
        m.insert("Recovery key successfully disabled", "Recovery key disabled successfully");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Could not disable recovery key. Please check your recovery key password!");
        m.insert("Password successfully changed.", "Password changed successfully.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Could not change the password. Maybe the old password was incorrect.");
        m.insert("Private key password successfully updated.", "Private key password updated successfully.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Could not update the private key password. Maybe the old password was not correct.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Encryption app not initialised! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialise the encryption app.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Cannot decrypt this file, which is probably a shared file. Please ask the file owner to reshare the file with you.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Unknown error. Please check your system settings or contact your administrator");
        m.insert("Missing requirements.", "Missing requirements.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.");
        m.insert("Following users are not set up for encryption:", "Following users are not set up for encryption:");
        m.insert("Saving...", "Saving...");
        m.insert("Go directly to your ", "Go directly to your ");
        m.insert("personal settings", "personal settings");
        m.insert("Encryption", "Encryption");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Enable recovery key (allow to recover users files in case of password loss):");
        m.insert("Recovery key password", "Recovery key password");
        m.insert("Repeat Recovery key password", "Repeat recovery key password");
        m.insert("Enabled", "Enabled");
        m.insert("Disabled", "Disabled");
        m.insert("Change recovery key password:", "Change recovery key password:");
        m.insert("Old Recovery key password", "Old recovery key password");
        m.insert("New Recovery key password", "New recovery key password");
        m.insert("Repeat New Recovery key password", "Repeat new recovery key password");
        m.insert("Change Password", "Change Password");
        m.insert("Your private key password no longer match your log-in password:", "Your private key password no longer matches your login password:");
        m.insert("Set your old private key password to your current log-in password.", "Set your old private key password to your current login password.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", " If you don't remember your old password you can ask your administrator to recover your files.");
        m.insert("Old log-in password", "Old login password");
        m.insert("Current log-in password", "Current login password");
        m.insert("Update Private Key Password", "Update Private Key Password");
        m.insert("Enable password recovery:", "Enable password recovery:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Enabling this option will allow you to reobtain access to your encrypted files in case of password loss");
        m.insert("File recovery settings updated", "File recovery settings updated");
        m.insert("Could not update file recovery", "Could not update file recovery");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}