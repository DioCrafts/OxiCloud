use crate::domain::entities::contact::AddressBook;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBookDto {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for AddressBookDto {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
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

impl From<AddressBook> for AddressBookDto {
    fn from(book: AddressBook) -> Self {
        Self {
            id: book.id().to_string(),
            name: book.name().to_string(),
            owner_id: book.owner_id().to_string(),
            description: book.description().map(|s| s.to_string()),
            color: book.color().map(|s| s.to_string()),
            is_public: book.is_public(),
            created_at: *book.created_at(),
            updated_at: *book.updated_at(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddressBookDto {
    pub name: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAddressBookDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: Option<bool>,
    pub user_id: String, // Current user making the update
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareAddressBookDto {
    pub address_book_id: String,
    pub user_id: String,
    pub can_write: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnshareAddressBookDto {
    pub address_book_id: String,
    pub user_id: String,
}
