use rust_gettext::prelude::*;

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_translations() -> rust_gettext::Catalog {
    let mut catalog = rust_gettext::Catalog::new();
    
    catalog.add_message(
        "_%n folder_::_%n folders_",
        vec!["", ""],
    );
    
    catalog.add_message(
        "_%n file_::_%n files_",
        vec!["", ""],
    );
    
    catalog.add_message(
        "_Uploading %n file_::_Uploading %n files_",
        vec!["", ""],
    );
    
    catalog
}