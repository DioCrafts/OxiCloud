use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "%s 항목을 이동시키지 못하였음 - 파일 이름이 이미 존재함");
        m.insert("Could not move %s", "%s 항목을 이딩시키지 못하였음");
        m.insert("File name cannot be empty.", "파일 이름이 비어 있을 수 없습니다.");
        m.insert("Unable to set upload directory.", "업로드 디렉터리를 정할수 없습니다");
        m.insert("Invalid Token", "잘못된 토큰");
        m.insert("No file was uploaded. Unknown error", "파일이 업로드되지 않았습니다. 알 수 없는 오류입니다");
        m.insert("There is no error, the file uploaded with success", "파일 업로드에 성공하였습니다.");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "업로드한 파일이 php.ini의 upload_max_filesize보다 큽니다:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "업로드한 파일 크기가 HTML 폼의 MAX_FILE_SIZE보다 큼");
        m.insert("The uploaded file was only partially uploaded", "파일의 일부분만 업로드됨");
        m.insert("No file was uploaded", "파일이 업로드되지 않았음");
        m.insert("Missing a temporary folder", "임시 폴더가 없음");
        m.insert("Failed to write to disk", "디스크에 쓰지 못했습니다");
        m.insert("Not enough storage available", "저장소가 용량이 충분하지 않습니다.");
        m.insert("Upload failed. Could not get file info.", "업로드에 실패했습니다. 파일 정보를 가져올수 없습니다.");
        m.insert("Upload failed. Could not find uploaded file", "업로드에 실패했습니다. 업로드할 파일을 찾을수 없습니다");
        m.insert("Invalid directory.", "올바르지 않은 디렉터리입니다.");
        m.insert("Files", "파일");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "{filename}을 업로드 할수 없습니다. 폴더이거나 0 바이트 파일입니다.");
        m.insert("Not enough space available", "여유 공간이 부족합니다");
        m.insert("Upload cancelled.", "업로드가 취소되었습니다.");
        m.insert("Could not get result from server.", "서버에서 결과를 가져올수 없습니다.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "파일 업로드가 진행 중입니다. 이 페이지를 벗어나면 업로드가 취소됩니다.");
        m.insert("{new_name} already exists", "{new_name}이(가) 이미 존재함");
        m.insert("Share", "공유");
        m.insert("Delete permanently", "영원히 삭제");
        m.insert("Rename", "이름 바꾸기");
        m.insert("Pending", "대기 중");
        m.insert("replaced {new_name} with {old_name}", "{old_name}이(가) {new_name}(으)로 대체됨");
        m.insert("undo", "되돌리기");
        m.insert("_%n folder_::_%n folders_", "폴더 %n");
        m.insert("_%n file_::_%n files_", "파일 %n 개");
        m.insert("{dirs} and {files}", "{dirs} 그리고 {files}");
        m.insert("_Uploading %n file_::_Uploading %n files_", "%n 개의 파일을 업로드중");
        m.insert("'.' is an invalid file name.", "'.' 는 올바르지 않은 파일 이름 입니다.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "폴더 이름이 올바르지 않습니다. 이름에 문자 '\\', '/', '<', '>', ':', '\"', '|', '? ', '*'는 사용할 수 없습니다.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "저장 공간이 가득 찼습니다. 파일을 업데이트하거나 동기화할 수 없습니다!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "저장 공간이 거의 가득 찼습니다 ({usedSpacePercent}%)");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "암호화는 해제되어 있지만, 파일은 아직 암호화 되어 있습니다. 개인 설저에 가셔서 암호를 해제하십시오");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "다운로드가 준비 중입니다. 파일 크기가 크다면 시간이 오래 걸릴 수도 있습니다.");
        m.insert("Error moving file", "파일 이동 오류");
        m.insert("Error", "오류");
        m.insert("Name", "이름");
        m.insert("Size", "크기");
        m.insert("Modified", "수정됨");
        m.insert("%s could not be renamed", "%s 의 이름을 변경할수 없습니다");
        m.insert("Upload", "업로드");
        m.insert("File handling", "파일 처리");
        m.insert("Maximum upload size", "최대 업로드 크기");
        m.insert("max. possible: ", "최대 가능:");
        m.insert("Needed for multi-file and folder downloads.", "다중 파일 및 폴더 다운로드에 필요합니다.");
        m.insert("Enable ZIP-download", "ZIP 다운로드 허용");
        m.insert("0 is unlimited", "0은 무제한입니다");
        m.insert("Maximum input size for ZIP files", "ZIP 파일 최대 크기");
        m.insert("Save", "저장");
        m.insert("New", "새로 만들기");
        m.insert("Text file", "텍스트 파일");
        m.insert("Folder", "폴더");
        m.insert("From link", "링크에서");
        m.insert("Deleted files", "파일 삭제됨");
        m.insert("Cancel upload", "업로드 취소");
        m.insert("Nothing in here. Upload something!", "내용이 없습니다. 업로드할 수 있습니다!");
        m.insert("Download", "다운로드");
        m.insert("Unshare", "공유 해제");
        m.insert("Delete", "삭제");
        m.insert("Upload too large", "업로드한 파일이 너무 큼");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "이 파일이 서버에서 허용하는 최대 업로드 가능 용량보다 큽니다.");
        m.insert("Files are being scanned, please wait.", "파일을 검색하고 있습니다. 기다려 주십시오.");
        m.insert("Current scanning", "현재 검색");
        m.insert("Upgrading filesystem cache...", "파일 시스템 캐시 업그레이드 중...");
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