use rust_i18n::translations;

translations! {
    // Plural forms for Slovak
    plural_forms: "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;",
    
    sk {
        // Folders
        "_n_folder_::_n_folders_": ["", "", ""],
        // Files
        "_n_file_::_n_files_": ["", "", ""],
    }
}