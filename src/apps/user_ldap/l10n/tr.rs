use std::collections::HashMap;
use fluent::FluentResource;
use unic_langid::LanguageIdentifier;

pub struct Turkish {
    translations: HashMap<String, String>,
    plural_forms: String,
}

impl Turkish {
    pub fn new() -> Self {
        let mut tr = Self {
            translations: HashMap::new(),
            plural_forms: "nplurals=2; plural=(n > 1);".to_string(),
        };
        tr.init_translations();
        tr
    }

    fn init_translations(&mut self) {
        let translations = [
            ("Failed to clear the mappings.", "Dönüşümleri(mapping) temizleme hata oluştu."),
            ("Failed to delete the server configuration", "Sunucu yapılandırmasını silme başarısız oldu"),
            ("The configuration is valid and the connection could be established!", "Yapılandırma geçerli ve bağlantı kuruldu!"),
            ("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Yapılandırma geçerli fakat bağlama (bind) başarısız. Lütfen Sunucu ayarları ve kimlik bilgilerini kontrol edin."),
            ("The configuration is invalid. Please have a look at the logs for further details.", "Yapılandırma geçersiz. Lütfen detaylar için günlüklere bakınız."),
            ("No action specified", "Eylem belirtilmedi"),
            ("No configuration specified", "Yapılandırma belirtilmemiş"),
            ("No data specified", "Veri belirtilmemiş"),
            (" Could not set configuration %s", "Yapılandırma %s olarak ayarlanamadı"),
            ("Deletion failed", "Silme başarısız oldu"),
            ("Take over settings from recent server configuration?", "Ayarlar son sunucu yapılandırmalarından devralınsın mı?"),
            ("Keep settings?", "Ayarlar kalsın mı?"),
            ("Cannot add server configuration", "Sunucu yapılandırması eklenemedi"),
            ("mappings cleared", "Dönüşümler temizlendi"),
            ("Success", "Başarılı"),
            ("Error", "Hata"),
            ("Select groups", "Grupları seç"),
            ("Select object classes", "Nesne sınıflarını seç"),
            ("Select attributes", "Nitelikleri seç"),
            ("Connection test succeeded", "Bağlantı testi başarılı oldu"),
            ("Connection test failed", "Bağlantı testi başarısız oldu"),
            ("Do you really want to delete the current Server Configuration?", "Şu anki sunucu yapılandırmasını silmek istediğinizden emin misiniz?"),
            ("Confirm Deletion", "Silmeyi onayla"),
            ("_%s group found_::_%s groups found_", "%s grup bulundu;%s grup bulundu"),
            ("_%s user found_::_%s users found_", "%s kullanıcı bulundu;%s kullanıcı bulundu"),
            ("Invalid Host", "Geçersiz Makine"),
            ("Could not find the desired feature", "İstenen özellik bulunamadı"),
            ("Save", "Kaydet"),
            ("Test Configuration", "Test Yapılandırması"),
            ("Help", "Yardım"),
            ("Limit the access to %s to groups meeting this criteria:", "%s erişimini, şu kriterle eşleşen gruplara sınırla:"),
            ("only those object classes:", "sadece bu nesne sınıflarına:"),
            ("only from those groups:", "sadece bu gruplardan:"),
            ("Edit raw filter instead", "Bunun yerine ham filtreyi düzenle"),
            ("Raw LDAP filter", "Ham LDAP filtresi"),
            ("The filter specifies which LDAP groups shall have access to the %s instance.", "Filtre, %s örneğine erişmesi gereken LDAP gruplarını belirtir."),
            ("groups found", "grup bulundu"),
            ("What attribute shall be used as login name:", "Oturum ismi olarak hangi nitelik kullanılmalı:"),
            ("LDAP Username:", "LDAP Kullanıcı Adı:"),
            ("LDAP Email Address:", "LDAP E-posta Adresi:"),
            ("Other Attributes:", "Diğer Nitelikler"),
            ("Add Server Configuration", "Sunucu Uyunlama birlemek "),
            ("Host", "Sunucu"),
            ("You can omit the protocol, except you require SSL. Then start with ldaps://", "Protokol atlamak edesin, sadece SSL istiyorsaniz. O zaman, idapsile baslamak. "),
            ("Port", "Port"),
            ("User DN", "Kullanıcı DN"),
            ("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN musterinin, kimle baglamaya yapacagiz,meselâ uid=agent.dc mesela, dc=com Gecinme adisiz ici, DN  ve Parola bos birakmak. "),
            ("Password", "Parola"),
            ("For anonymous access, leave DN and Password empty.", "Anonim erişim için DN ve Parola alanlarını boş bırakın."),
            ("One Base DN per line", "Bir Tabani DN herbir dizi. "),
            ("You can specify Base DN for users and groups in the Advanced tab", "Base DN kullanicileri  ve kaynaklari icin tablosu Advanced tayin etmek ederiz. "),
            ("Limit the access to %s to users meeting this criteria:", "%s erişimini, şu kriterle eşleşen kullanıcılara sınırla:"),
            ("The filter specifies which LDAP users shall have access to the %s instance.", "Filtre, %s örneğine erişmesi gereken LDAP kullanıcılarını belirtir."),
            ("users found", "kullanıcı bulundu"),
            ("Back", "Geri"),
            ("Continue", "Devam et"),
            ("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Uyarı:</b> user_ldap ve user_webdavauth uygulamaları uyumlu değil. Beklenmedik bir davranışla karşılaşabilirsiniz. Lütfen ikisinden birini devre dışı bırakmak için sistem yöneticinizle iletişime geçin."),
            ("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Ihbar <b> Modulu PHP LDAP yuklemdi degil, backend calismacak. Lutfen sistem yonetici sormak  yuklemek icin."),
            ("Connection Settings", "Bağlantı ayarları"),
            ("Configuration Active", "Yapılandırma Etkin"),
            ("When unchecked, this configuration will be skipped.", "Ne zaman iptal, bu uynnlama isletici "),
            ("User Login Filter", "Kullanıcı Oturum Filtresi"),
            ("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Oturum açma girişimi olduğunda uygulanacak filtreyi tanımlar. %%uid, oturum işleminde kullanıcı adı ile değiştirilir. Örneğin: \"uid=%%uid\""),
            ("Backup (Replica) Host", "Sigorta Kopya Cephe "),
            ("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Bir kopya cevre vermek, kopya sunucu onemli olmali. "),
            ("Backup (Replica) Port", "Kopya Port "),
            ("Disable Main Server", "Ana sunucuyu devredışı birak"),
            ("Only connect to the replica server.", "Sadece kopya sunucuya bağlan."),
            ("Case insensitve LDAP server (Windows)", "Dusme sunucu LDAP zor degil. (Windows)"),
            ("Turn off SSL certificate validation.", "SSL sertifika doğrulamasını kapat."),
            ("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Önerilmez, sadece test için kullanın! Eğer bağlantı sadece bu seçenekle çalışıyorsa %s sunucunuza LDAP sunucusunun SSL sertifikasını ekleyin."),
            ("Cache Time-To-Live", "Cache Time-To-Live "),
            ("in seconds. A change empties the cache.", "saniye cinsinden. Bir değişiklik önbelleği temizleyecektir."),
            ("Directory Settings", "Parametrar Listesin Adresinin "),
            ("User Display Name Field", "Ekran Adi Kullanici, (Alan Adi Kullanici Ekrane)"),
            ("The LDAP attribute to use to generate the user's display name.", "Kullanıcının görünen adını oluşturmak için kullanılacak LDAP niteliği."),
            ("Base User Tree", "Temel Kullanıcı Ağacı"),
            ("One User Base DN per line", "Bir Temel Kullanici DN her dizgi "),
            ("User Search Attributes", "Kategorii Arama Kullanici "),
            ("Optional; one attribute per line", "Tercihe bağlı; her bir satırd bir öznitelik"),
            ("Group Display Name Field", "Grub Ekrane Alani Adi"),
            ("The LDAP attribute to use to generate the groups's display name.", "Grubun görünen adını oluşturmak için kullanılacak LDAP niteliği."),
            ("Base Group Tree", "Temel Grup Ağacı"),
            ("One Group Base DN per line", "Bir Grubu Tabani DN her dizgi. "),
            ("Group Search Attributes", "Kategorii Arama Grubu"),
            ("Group-Member association", "Grup-Üye işbirliği"),
            ("Special Attributes", "Özel Öznitelikler"),
            ("Quota Field", "Kota alanı"),
            ("Quota Default", "Öntanımlı Kota"),
            ("in bytes", "byte cinsinden"),
            ("Email Field", "E-posta Alanı"),
            ("User Home Folder Naming Rule", "Kullanıcı Ana Dizini İsimlendirm Kuralı"),
            ("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Kullanıcı adı bölümünü boş bırakın (varsayılan). "),
            ("Internal Username", "Dahili Kullanıcı Adı"),
            ("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Öntanımlı olarak UUID niteliğinden dahili bir kullanıcı adı oluşturulacak. Bu, kullanıcı adının benzersiz ve karakterlerinin dönüştürme gereksinimini ortadan kaldırır. Dahili kullanıcı adı, sadece bu karakterlerin izin verildiği kısıtlamaya sahip: [ a-zA-Z0-9_.@- ]. Diğer karakterler ise ASCII karşılıkları ile yer değiştirilir veya basitçe yoksayılır. Çakışmalar olduğunda ise bir numara eklenir veya arttırılır. Dahili kullanıcı adı, bir kullanıcıyı dahili olarak tanımlamak için kullanılır. Ayrıca kullanıcı ev klasörü için öntanımlı bir isimdir. Bu ayrıca uzak adreslerin (örneğin tüm *DAV hizmetleri) bir parçasıdır. Bu yar ise, öntanımlı davranışın üzerine yazılabilir. ownCloud 5'ten önce benzer davranışı yapabilmek için aşağıdaki alana bir kullanıcı görünen adı niteliği girin. Öntanımlı davranış için boş bırakın. Değişiklikler, sadece yeni eşleştirilen (eklenen) LDAP kullanıcılarında etkili olacaktır."),
            ("Internal Username Attribute:", "Dahili Kullanıcı adı Özniteliği:"),
            ("Override UUID detection", "UUID tespitinin üzerine yaz"),
            ("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Öntanımlı olarak, UUID niteliği otomatik olarak tespit edilmez. UUID niteliği LDAP kullanıcılarını ve gruplarını şüphesiz biçimde tanımlamak için kullanılır. Ayrıca yukarıda belirtilmemişse, bu UUID'ye bağlı olarak dahili bir kullanıcı adı oluşturulacaktır. Bu ayarın üzerine yazabilir ve istediğiniz bir nitelik belirtebilirsiniz. Ancak istediğiniz niteliğin benzersiz olduğundan ve hem kullanıcı hem de gruplar tarafından getirilebileceğinden emin olmalısınız. Öntanımlı davranış için boş bırakın. Değişiklikler sadece yeni eşleştirilen (eklenen) LDAP kullanıcı ve gruplarında etkili olacaktır."),
            ("UUID Attribute for Users:", "Kullanıcılar için UUID Özniteliği:"),
            ("UUID Attribute for Groups:", "Gruplar için UUID Özniteliği:"),
            ("Username-LDAP User Mapping", "Kullanıcı Adı-LDAP Kullanıcısı dönüşümü"),
            ("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Kullanıcı adları, (üst) veri depolaması ve ataması için kullanılır. Kullanıcıları kesin olarak tanımlamak ve algılamak için, her LDAP kullanıcısı bir dahili kullanıcı adına sahip olacak. Bu kullanıcı adı ile LDAP kullanıcısı arasında bir eşleşme gerektirir. Oluşturulan kullanıcı adı LDAP kullanıcısının UUID'si ile eşleştirilir. Ek olarak LDAP etkileşimini azaltmak için DN de önbelleğe alınır ancak bu kimlik tanıma için kullanılmaz. Eğer DN değişirse, değişiklikler tespit edilir. Dahili kullanıcı her yerde kullanılır. Eşleştirmeleri temizlemek, her yerde kalıntılar bırakacaktır. Eşleştirmeleri temizlemek yapılandırmaya hassas bir şekilde bağlı değildir, tüm LDAP yapılandırmalarını etkiler! Üretim ortamında eşleştirmeleri asla temizlemeyin, sadece sınama veya deneysel aşamada kullanın."),
            ("Clear Username-LDAP User Mapping", "Kullanıcı Adı-LDAP Kullanıcısı Dönüşümünü Temizle"),
            ("Clear Groupname-LDAP Group Mapping", "Grup Adı-LDAP Grubu Dönüşümü"),
        ];

        for (key, value) in translations.iter() {
            self.translations.insert(key.to_string(), value.to_string());
        }
    }

    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}

pub fn create_tr_translations() -> Turkish {
    Turkish::new()
}