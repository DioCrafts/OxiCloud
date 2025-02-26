use rust_i18n::static_localize;

// Define the l10n for ml_IN
static_localize! {
    static ML_IN {
        en {
            "_%n folder_::_%n folders_" = ["{} folder", "{} folders"],
            "_%n file_::_%n files_" = ["{} file", "{} files"],
            "_Uploading %n file_::_Uploading %n files_" = ["Uploading {} file", "Uploading {} files"],
        }
        
        ml_IN {
            // Placeholder translations (empty in the original)
            "_%n folder_::_%n folders_" = ["", ""],
            "_%n file_::_%n files_" = ["", ""],
            "_Uploading %n file_::_Uploading %n files_" = ["", ""],
        }
    }
}

// Define the plural rules for ml_IN
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}