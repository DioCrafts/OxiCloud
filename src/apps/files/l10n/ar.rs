use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "فشل في نقل الملف %s - يوجد ملف بنفس هذا الاسم");
    translations.insert("Could not move %s", "فشل في نقل %s");
    translations.insert("File name cannot be empty.", "اسم الملف لا يجوز أن يكون فارغا");
    translations.insert("Unable to set upload directory.", "غير قادر على تحميل المجلد");
    translations.insert("Invalid Token", "علامة غير صالحة");
    translations.insert("No file was uploaded. Unknown error", "لم يتم رفع أي ملف , خطأ غير معروف");
    translations.insert("There is no error, the file uploaded with success", "تم ترفيع الملفات بنجاح.");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "حجم الملف المرفوع تجاوز قيمة  upload_max_filesize الموجودة في ملف php.ini ");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "حجم الملف الذي تريد ترفيعه أعلى مما MAX_FILE_SIZE يسمح به في واجهة ال HTML.");
    translations.insert("The uploaded file was only partially uploaded", "تم ترفيع جزء من الملفات الذي تريد ترفيعها فقط");
    translations.insert("No file was uploaded", "لم يتم ترفيع أي من الملفات");
    translations.insert("Missing a temporary folder", "المجلد المؤقت غير موجود");
    translations.insert("Failed to write to disk", "خطأ في الكتابة على القرص الصلب");
    translations.insert("Not enough storage available", "لا يوجد مساحة تخزينية كافية");
    translations.insert("Upload failed. Could not get file info.", "فشلت عملية الرفع. تعذر الحصول على معلومات الملف.");
    translations.insert("Upload failed. Could not find uploaded file", "*فشلت علمية الرفع. تعذر إيجاد الملف الذي تم رفعه.\n*فشلت علمية التحميل. تعذر إيجاد الملف الذي تم تحميله.");
    translations.insert("Invalid directory.", "مسار غير صحيح.");
    translations.insert("Files", "الملفات");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "تعذر رفع الملف {filename} إما لأنه مجلد أو لان حجم الملف 0 بايت");
    translations.insert("Not enough space available", "لا توجد مساحة كافية");
    translations.insert("Upload cancelled.", "تم إلغاء عملية رفع الملفات .");
    translations.insert("Could not get result from server.", "تعذر الحصول على نتيجة من الخادم");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "عملية رفع الملفات قيد التنفيذ. اغلاق الصفحة سوف يلغي عملية رفع الملفات.");
    translations.insert("{new_name} already exists", "{new_name} موجود مسبقا");
    translations.insert("Share", "شارك");
    translations.insert("Delete permanently", "حذف بشكل دائم");
    translations.insert("Rename", "إعادة تسميه");
    translations.insert("Pending", "قيد الانتظار");
    translations.insert("replaced {new_name} with {old_name}", "استبدل {new_name}  بـ  {old_name}");
    translations.insert("undo", "تراجع");
    translations.insert("{dirs} and {files}", "{dirs} و {files}");
    translations.insert("'.' is an invalid file name.", "\".\" اسم ملف غير صحيح.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "اسم غير صحيح , الرموز  '\\', '/', '<', '>', ':', '\"', '|', '?' و \"*\" غير مسموح استخدامها");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "مساحتك التخزينية ممتلئة, لا يمكم تحديث ملفاتك أو مزامنتها بعد الآن !");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "مساحتك التخزينية امتلأت تقريبا ");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "تم تمكين تشفير البرامج لكن لم يتم تهيئة المفاتيح لذا يرجى تسجيل الخروج ثم تسجيل الدخول مرة آخرى.");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "المفتاح الخاص بتشفير التطبيقات غير صالح. يرجى تحديث كلمة السر الخاصة بالمفتاح الخاص من الإعدادت الشخصية حتى تتمكن من الوصول للملفات المشفرة.");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "تم تعطيل التشفير لكن ملفاتك لا تزال مشفرة. فضلا اذهب إلى الإعدادات الشخصية لإزالة التشفير عن ملفاتك.");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "جاري تجهيز عملية التحميل. قد تستغرق بعض الوقت اذا كان حجم الملفات كبير.");
    translations.insert("Error moving file", "حدث خطأ أثناء نقل الملف");
    translations.insert("Error", "خطأ");
    translations.insert("Name", "اسم");
    translations.insert("Size", "حجم");
    translations.insert("Modified", "معدل");
    translations.insert("%s could not be renamed", "%s  لا يمكن إعادة تسميته. ");
    translations.insert("Upload", "رفع");
    translations.insert("File handling", "التعامل مع الملف");
    translations.insert("Maximum upload size", "الحد الأقصى لحجم الملفات التي يمكن رفعها");
    translations.insert("max. possible: ", "الحد الأقصى المسموح به");
    translations.insert("Needed for multi-file and folder downloads.", "اجباري للسماح بالتحميل المتعدد للمجلدات والملفات");
    translations.insert("Enable ZIP-download", "تفعيل خاصية تحميل ملفات ZIP");
    translations.insert("0 is unlimited", "0 = غير محدود");
    translations.insert("Maximum input size for ZIP files", "الحد الأقصى المسموح به لملفات ZIP");
    translations.insert("Save", "حفظ");
    translations.insert("New", "جديد");
    translations.insert("Text file", "ملف");
    translations.insert("Folder", "مجلد");
    translations.insert("From link", "من رابط");
    translations.insert("Deleted files", "حذف الملفات");
    translations.insert("Cancel upload", "إلغاء رفع الملفات");
    translations.insert("Nothing in here. Upload something!", "لا يوجد شيء هنا. إرفع بعض الملفات!");
    translations.insert("Download", "تحميل");
    translations.insert("Unshare", "إلغاء مشاركة");
    translations.insert("Delete", "إلغاء");
    translations.insert("Upload too large", "حجم الترفيع أعلى من المسموح");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "حجم الملفات التي تريد ترفيعها أعلى من المسموح على الخادم.");
    translations.insert("Files are being scanned, please wait.", "يرجى الانتظار , جاري فحص الملفات .");
    translations.insert("Current scanning", "الفحص الحالي");
    translations.insert("Upgrading filesystem cache...", "تحديث ذاكرة التخزين المؤقت(الكاش)  الخاصة بملفات النظام ...");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plurals = HashMap::new();
    
    plurals.insert("_%n folder_::_%n folders_", vec![
        "لا يوجد مجلدات %n",
        "1 مجلد %n",
        "2 مجلد %n",
        "عدد قليل من مجلدات %n",
        "عدد كبير من مجلدات %n",
        "مجلدات %n"
    ]);
    
    plurals.insert("_%n file_::_%n files_", vec![
        "لا يوجد ملفات %n",
        "ملف %n",
        "2 ملف %n",
        "قليل من ملفات %n",
        "الكثير من ملفات %n",
        " ملفات %n"
    ]);
    
    plurals.insert("_Uploading %n file_::_Uploading %n files_", vec![
        "لا يوجد ملفات %n لتحميلها",
        "تحميل 1 ملف %n",
        "تحميل 2 ملف %n",
        "يتم تحميل عدد قليل من ملفات %n",
        "يتم تحميل عدد كبير من ملفات %n",
        "يتم تحميل ملفات %n"
    ]);
    
    plurals
}