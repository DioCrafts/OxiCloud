use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AddressBook {
    pub id: Uuid,
    pub name: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for AddressBook {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Default Address Book".to_string(),
            owner_id: "default".to_string(),
            description: None,
            color: None,
            is_public: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    pub email: String,
    pub r#type: String, // home, work, other
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Phone {
    pub number: String,
    pub r#type: String, // mobile, home, work, fax, other
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub r#type: String, // home, work, other
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: Uuid,
    pub address_book_id: Uuid,
    pub uid: String,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub email: Vec<Email>,
    pub phone: Vec<Phone>,
    pub address: Vec<Address>,
    pub organization: Option<String>,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub photo_url: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub anniversary: Option<NaiveDate>,
    pub vcard: String,
    pub etag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            address_book_id: Uuid::new_v4(),
            uid: format!("{}@oxicloud", Uuid::new_v4()),
            full_name: None,
            first_name: None,
            last_name: None,
            nickname: None,
            email: Vec::new(),
            phone: Vec::new(),
            address: Vec::new(),
            organization: None,
            title: None,
            notes: None,
            photo_url: None,
            birthday: None,
            anniversary: None,
            vcard: "BEGIN:VCARD\nVERSION:3.0\nEND:VCARD".to_string(),
            etag: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactGroup {
    pub id: Uuid,
    pub address_book_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ContactGroup {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            address_book_id: Uuid::new_v4(),
            name: "New Group".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}