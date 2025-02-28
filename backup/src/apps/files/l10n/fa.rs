use std::collections::HashMap;
use rust_fluent::FluentBundle;

pub fn create_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "Could not move %s - File with this name already exists".to_string(),
        "%s نمی تواند حرکت کند - در حال حاضر پرونده با این نام وجود دارد. ".to_string(),
    );
    translations.insert(
        "Could not move %s".to_string(), 
        "%s نمی تواند حرکت کند ".to_string(),
    );
    translations.insert(
        "File name cannot be empty.".to_string(),
        "نام پرونده نمی تواند خالی باشد.".to_string(),
    );
    translations.insert(
        "Unable to set upload directory.".to_string(),
        "قادر به تنظیم پوشه آپلود نمی باشد.".to_string(),
    );
    translations.insert(
        "Invalid Token".to_string(),
        "رمز نامعتبر".to_string(),
    );
    translations.insert(
        "No file was uploaded. Unknown error".to_string(),
        "هیچ فایلی آپلود نشد.خطای ناشناس".to_string(),
    );
    translations.insert(
        "There is no error, the file uploaded with success".to_string(),
        "هیچ خطایی نیست بارگذاری پرونده موفقیت آمیز بود".to_string(),
    );
    translations.insert(
        "The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(),
        "پرونده آپلود شده بیش ازدستور  ماکزیمم_حجم فایل_برای آپلود در   php.ini استفاده کرده است.".to_string(),
    );
    translations.insert(
        "The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(),
        "حداکثر حجم قابل بار گذاری از طریق HTML MAX_FILE_SIZE است".to_string(),
    );
    translations.insert(
        "The uploaded file was only partially uploaded".to_string(),
        "پرونده بارگذاری شده فقط تاحدودی بارگذاری شده".to_string(),
    );
    translations.insert(
        "No file was uploaded".to_string(),
        "هیچ پروندهای بارگذاری نشده".to_string(),
    );
    translations.insert(
        "Missing a temporary folder".to_string(),
        "یک پوشه موقت گم شده".to_string(),
    );
    translations.insert(
        "Failed to write to disk".to_string(),
        "نوشتن بر روی دیسک سخت ناموفق بود".to_string(),
    );
    translations.insert(
        "Not enough storage available".to_string(),
        "فضای کافی در دسترس نیست".to_string(),
    );
    translations.insert(
        "Invalid directory.".to_string(),
        "فهرست راهنما نامعتبر می باشد.".to_string(),
    );
    translations.insert(
        "Files".to_string(),
        "پرونده‌ها".to_string(),
    );
    translations.insert(
        "Not enough space available".to_string(),
        "فضای کافی در دسترس نیست".to_string(),
    );
    translations.insert(
        "Upload cancelled.".to_string(),
        "بار گذاری لغو شد".to_string(),
    );
    translations.insert(
        "File upload is in progress. Leaving the page now will cancel the upload.".to_string(),
        "آپلودکردن پرونده در حال پیشرفت است. در صورت خروج از صفحه آپلود لغو میگردد. ".to_string(),
    );
    translations.insert(
        "{new_name} already exists".to_string(),
        "{نام _جدید} در حال حاضر وجود دارد.".to_string(),
    );
    translations.insert(
        "Share".to_string(),
        "اشتراک‌گذاری".to_string(),
    );
    translations.insert(
        "Delete permanently".to_string(),
        "حذف قطعی".to_string(),
    );
    translations.insert(
        "Rename".to_string(),
        "تغییرنام".to_string(),
    );
    translations.insert(
        "Pending".to_string(),
        "در انتظار".to_string(),
    );
    translations.insert(
        "replaced {new_name} with {old_name}".to_string(),
        "{نام_جدید} با { نام_قدیمی} جایگزین شد.".to_string(),
    );
    translations.insert(
        "undo".to_string(),
        "بازگشت".to_string(),
    );
    translations.insert(
        "_%n folder_::_%n folders_".to_string(),
        "".to_string(),
    );
    translations.insert(
        "_%n file_::_%n files_".to_string(),
        "".to_string(),
    );
    translations.insert(
        "_Uploading %n file_::_Uploading %n files_".to_string(),
        "".to_string(),
    );
    translations.insert(
        "'.' is an invalid file name.".to_string(),
        "'.'   یک نام پرونده نامعتبر است.".to_string(),
    );
    translations.insert(
        "Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(),
        "نام نامعتبر ،  '\\', '/', '<', '>', ':', '\"', '|', '?'  و '*'  مجاز نمی باشند.".to_string(),
    );
    translations.insert(
        "Your storage is full, files can not be updated or synced anymore!".to_string(),
        "فضای ذخیره ی شما کاملا پر است، بیش از این فایلها بهنگام یا همگام سازی نمی توانند بشوند!".to_string(),
    );
    translations.insert(
        "Your storage is almost full ({usedSpacePercent}%)".to_string(),
        "فضای ذخیره ی شما تقریبا پر است ({usedSpacePercent}%)".to_string(),
    );
    translations.insert(
        "Your download is being prepared. This might take some time if the files are big.".to_string(),
        "دانلود شما در حال آماده شدن است. در صورتیکه پرونده ها بزرگ باشند ممکن است مدتی طول بکشد.".to_string(),
    );
    translations.insert(
        "Error".to_string(),
        "خطا".to_string(),
    );
    translations.insert(
        "Name".to_string(),
        "نام".to_string(),
    );
    translations.insert(
        "Size".to_string(),
        "اندازه".to_string(),
    );
    translations.insert(
        "Modified".to_string(),
        "تاریخ".to_string(),
    );
    translations.insert(
        "%s could not be renamed".to_string(),
        "%s نمیتواند تغییر نام دهد.".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "بارگزاری".to_string(),
    );
    translations.insert(
        "File handling".to_string(),
        "اداره پرونده ها".to_string(),
    );
    translations.insert(
        "Maximum upload size".to_string(),
        "حداکثر اندازه بارگزاری".to_string(),
    );
    translations.insert(
        "max. possible: ".to_string(),
        "حداکثرمقدارممکن:".to_string(),
    );
    translations.insert(
        "Needed for multi-file and folder downloads.".to_string(),
        "احتیاج پیدا خواهد شد برای چند پوشه و پرونده".to_string(),
    );
    translations.insert(
        "Enable ZIP-download".to_string(),
        "فعال سازی بارگیری پرونده های فشرده".to_string(),
    );
    translations.insert(
        "0 is unlimited".to_string(),
        "0 نامحدود است".to_string(),
    );
    translations.insert(
        "Maximum input size for ZIP files".to_string(),
        "حداکثرمقدار برای بار گزاری پرونده های فشرده".to_string(),
    );
    translations.insert(
        "Save".to_string(),
        "ذخیره".to_string(),
    );
    translations.insert(
        "New".to_string(),
        "جدید".to_string(),
    );
    translations.insert(
        "Text file".to_string(),
        "فایل متنی".to_string(),
    );
    translations.insert(
        "Folder".to_string(),
        "پوشه".to_string(),
    );
    translations.insert(
        "From link".to_string(),
        "از پیوند".to_string(),
    );
    translations.insert(
        "Deleted files".to_string(),
        "فایل های حذف شده".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "متوقف کردن بار گذاری".to_string(),
    );
    translations.insert(
        "Nothing in here. Upload something!".to_string(),
        "اینجا هیچ چیز نیست.".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "دانلود".to_string(),
    );
    translations.insert(
        "Unshare".to_string(),
        "لغو اشتراک".to_string(),
    );
    translations.insert(
        "Delete".to_string(),
        "حذف".to_string(),
    );
    translations.insert(
        "Upload too large".to_string(),
        "سایز فایل برای آپلود زیاد است(م.تنظیمات در php.ini)".to_string(),
    );
    translations.insert(
        "The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(),
        "فایلها بیش از حد تعیین شده در این سرور هستند\nمترجم:با تغییر فایل php,ini میتوان این محدودیت را برطرف کرد".to_string(),
    );
    translations.insert(
        "Files are being scanned, please wait.".to_string(),
        "پرونده ها در حال بازرسی هستند لطفا صبر کنید".to_string(),
    );
    translations.insert(
        "Current scanning".to_string(),
        "بازرسی کنونی".to_string(),
    );
    translations.insert(
        "Upgrading filesystem cache...".to_string(),
        "بهبود فایل سیستمی ذخیره گاه...".to_string(),
    );
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=1; plural=0;".to_string()
}

pub struct FarsiTranslation;

impl FarsiTranslation {
    pub fn new() -> Self {
        FarsiTranslation
    }
    
    pub fn get_translations(&self) -> HashMap<String, String> {
        create_translations()
    }
    
    pub fn get_plural_form(&self) -> String {
        get_plural_form()
    }
}