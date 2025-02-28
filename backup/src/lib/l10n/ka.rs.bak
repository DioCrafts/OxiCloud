use rust_i18n::i18n;

i18n!("ka", {
    "Help": "შველა",
    "Personal": "პერსონა",
    "Users": "მომხმარებლები",
    "Admin": "ადმინისტრატორი",
    "ZIP download is turned off.": "ZIP გადმოწერა გამორთულია",
    "Files": "ფაილები",
    "seconds ago": "წამის წინ",
    "_{n} minute ago_::_{n} minutes ago_": "",
    "_{n} hour ago_::_{n} hours ago_": "",
    "today": "დღეს",
    "yesterday": "გუშინ",
    "_{n} day go_::_{n} days ago_": "",
    "_{n} month ago_::_{n} months ago_": ""
});

#[no_mangle]
pub extern "C" fn get_plural_forms() -> *const std::os::raw::c_char {
    let plural_forms = "nplurals=1; plural=0;";
    std::ffi::CString::new(plural_forms)
        .expect("Failed to create CString")
        .into_raw()
}

// Free the memory allocated for the C string
#[no_mangle]
pub extern "C" fn free_plural_forms(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(ptr);
        }
    }
}