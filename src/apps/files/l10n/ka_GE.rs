use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "%s –ის გადატანა ვერ მოხერხდა – ფაილი ამ სახელით უკვე არსებობს");
        m.insert("Could not move %s", "%s –ის გადატანა ვერ მოხერხდა");
        m.insert("File name cannot be empty.", "ფაილის სახელი არ შეიძლება იყოს ცარიელი.");
        m.insert("No file was uploaded. Unknown error", "ფაილი არ აიტვირთა. უცნობი შეცდომა");
        m.insert("There is no error, the file uploaded with success", "ჭოცდომა არ დაფიქსირდა, ფაილი წარმატებით აიტვირთა");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "ატვირთული ფაილი აჭარბებს upload_max_filesize დირექტივას php.ini ფაილში");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "ატვირთული ფაილი აჭარბებს  MAX_FILE_SIZE დირექტივას, რომელიც მითითებულია HTML ფორმაში");
        m.insert("The uploaded file was only partially uploaded", "ატვირთული ფაილი მხოლოდ ნაწილობრივ აიტვირთა");
        m.insert("No file was uploaded", "ფაილი არ აიტვირთა");
        m.insert("Missing a temporary folder", "დროებითი საქაღალდე არ არსებობს");
        m.insert("Failed to write to disk", "შეცდომა დისკზე ჩაწერისას");
        m.insert("Not enough storage available", "საცავში საკმარისი ადგილი არ არის");
        m.insert("Invalid directory.", "დაუშვებელი დირექტორია.");
        m.insert("Files", "ფაილები");
        m.insert("Not enough space available", "საკმარისი ადგილი არ არის");
        m.insert("Upload cancelled.", "ატვირთვა შეჩერებულ იქნა.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "მიმდინარეობს ფაილის ატვირთვა. სხვა გვერდზე გადასვლა გამოიწვევს ატვირთვის შეჩერებას");
        m.insert("{new_name} already exists", "{new_name} უკვე არსებობს");
        m.insert("Share", "გაზიარება");
        m.insert("Delete permanently", "სრულად წაშლა");
        m.insert("Rename", "გადარქმევა");
        m.insert("Pending", "მოცდის რეჟიმში");
        m.insert("replaced {new_name} with {old_name}", "{new_name} შეცვლილია {old_name}–ით");
        m.insert("undo", "დაბრუნება");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("'.' is an invalid file name.", "'.' არის დაუშვებელი ფაილის სახელი.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "არადაშვებადი სახელი, '\\', '/', '<', '>', ':', '\"', '|', '?' და  '*' არ არის დაიშვებული.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "თქვენი საცავი გადაივსო. ფაილების განახლება და სინქრონიზირება ვერ მოხერხდება!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "თქვენი საცავი თითქმის გადაივსო ({usedSpacePercent}%)");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "გადმოწერის მოთხოვნა მუშავდება. ის მოითხოვს გარკვეულ დროს რაგდან ფაილები არის დიდი ზომის.");
        m.insert("Error", "შეცდომა");
        m.insert("Name", "სახელი");
        m.insert("Size", "ზომა");
        m.insert("Modified", "შეცვლილია");
        m.insert("Upload", "ატვირთვა");
        m.insert("File handling", "ფაილის დამუშავება");
        m.insert("Maximum upload size", "მაქსიმუმ ატვირთის ზომა");
        m.insert("max. possible: ", "მაქს. შესაძლებელი:");
        m.insert("Needed for multi-file and folder downloads.", "საჭიროა მულტი ფაილ ან საქაღალდის ჩამოტვირთვა.");
        m.insert("Enable ZIP-download", "ZIP-Download–ის ჩართვა");
        m.insert("0 is unlimited", "0 is unlimited");
        m.insert("Maximum input size for ZIP files", "ZIP ფაილების მაქსიმუმ დასაშვები ზომა");
        m.insert("Save", "შენახვა");
        m.insert("New", "ახალი");
        m.insert("Text file", "ტექსტური ფაილი");
        m.insert("Folder", "საქაღალდე");
        m.insert("From link", "მისამართიდან");
        m.insert("Deleted files", "წაშლილი ფაილები");
        m.insert("Cancel upload", "ატვირთვის გაუქმება");
        m.insert("Nothing in here. Upload something!", "აქ არაფერი არ არის. ატვირთე რამე!");
        m.insert("Download", "ჩამოტვირთვა");
        m.insert("Unshare", "გაუზიარებადი");
        m.insert("Delete", "წაშლა");
        m.insert("Upload too large", "ასატვირთი ფაილი ძალიან დიდია");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "ფაილის ზომა რომლის ატვირთვასაც თქვენ აპირებთ, აჭარბებს სერვერზე დაშვებულ მაქსიმუმს.");
        m.insert("Files are being scanned, please wait.", "მიმდინარეობს ფაილების სკანირება, გთხოვთ დაელოდოთ.");
        m.insert("Current scanning", "მიმდინარე სკანირება");
        m.insert("Upgrading filesystem cache...", "ფაილური სისტემის ქეშის განახლება....");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}