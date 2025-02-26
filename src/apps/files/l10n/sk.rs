use rust_i18n::i18n;

i18n!("sk", {
    "folder": {
        "zero": "{count} folder",
        "one": "{count} folder",
        "two": "{count} folders",
        "few": "{count} folders",
        "many": "{count} folders",
        "other": "{count} folders"
    },
    "file": {
        "zero": "{count} file",
        "one": "{count} file",
        "two": "{count} files",
        "few": "{count} files",
        "many": "{count} files",
        "other": "{count} files"
    },
    "uploading": {
        "zero": "Uploading {count} file",
        "one": "Uploading {count} file",
        "two": "Uploading {count} files",
        "few": "Uploading {count} files",
        "many": "Uploading {count} files",
        "other": "Uploading {count} files"
    }
});

pub fn plural_form(n: usize) -> &'static str {
    if n == 1 {
        "one"
    } else if n >= 2 && n <= 4 {
        "few"
    } else {
        "many"
    }
}