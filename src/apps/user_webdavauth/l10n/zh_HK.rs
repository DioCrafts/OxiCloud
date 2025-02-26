use rust_i18n::i18n;

i18n!("zh_HK", {
    "WebDAV Authentication": "WebDAV 認證",
    "Address: ": "位址:",
    "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.": "使用者憑證將會被傳送到此位址。此外掛程式將會檢查回應，HTTP狀態碼 401與403將會被理解為無效憑證，而所有其他的回應將會被理解為有效憑證。"
});

pub fn plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}