//! Persistence DTOs for Contact entities
//! 
//! These DTOs are used for JSONB serialization/deserialization in PostgreSQL.
//! They mirror the domain entities but include serde traits required for persistence.
//! This keeps the domain layer free of infrastructure concerns (serde dependency).

use serde::{Deserialize, Serialize};
use crate::domain::entities::contact::{Email, Phone, Address};

/// Persistence DTO for Email - used for JSONB serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailPersistenceDto {
    pub email: String,
    pub r#type: String,
    pub is_primary: bool,
}

impl From<&Email> for EmailPersistenceDto {
    fn from(email: &Email) -> Self {
        Self {
            email: email.email.clone(),
            r#type: email.r#type.clone(),
            is_primary: email.is_primary,
        }
    }
}

impl From<EmailPersistenceDto> for Email {
    fn from(dto: EmailPersistenceDto) -> Self {
        Self {
            email: dto.email,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }
}

/// Persistence DTO for Phone - used for JSONB serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonePersistenceDto {
    pub number: String,
    pub r#type: String,
    pub is_primary: bool,
}

impl From<&Phone> for PhonePersistenceDto {
    fn from(phone: &Phone) -> Self {
        Self {
            number: phone.number.clone(),
            r#type: phone.r#type.clone(),
            is_primary: phone.is_primary,
        }
    }
}

impl From<PhonePersistenceDto> for Phone {
    fn from(dto: PhonePersistenceDto) -> Self {
        Self {
            number: dto.number,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }
}

/// Persistence DTO for Address - used for JSONB serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressPersistenceDto {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub r#type: String,
    pub is_primary: bool,
}

impl From<&Address> for AddressPersistenceDto {
    fn from(addr: &Address) -> Self {
        Self {
            street: addr.street.clone(),
            city: addr.city.clone(),
            state: addr.state.clone(),
            postal_code: addr.postal_code.clone(),
            country: addr.country.clone(),
            r#type: addr.r#type.clone(),
            is_primary: addr.is_primary,
        }
    }
}

impl From<AddressPersistenceDto> for Address {
    fn from(dto: AddressPersistenceDto) -> Self {
        Self {
            street: dto.street,
            city: dto.city,
            state: dto.state,
            postal_code: dto.postal_code,
            country: dto.country,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }
}

/// Helper functions to convert collections
pub fn emails_to_persistence(emails: &[Email]) -> Vec<EmailPersistenceDto> {
    emails.iter().map(EmailPersistenceDto::from).collect()
}

pub fn emails_from_persistence(dtos: Vec<EmailPersistenceDto>) -> Vec<Email> {
    dtos.into_iter().map(Email::from).collect()
}

pub fn phones_to_persistence(phones: &[Phone]) -> Vec<PhonePersistenceDto> {
    phones.iter().map(PhonePersistenceDto::from).collect()
}

pub fn phones_from_persistence(dtos: Vec<PhonePersistenceDto>) -> Vec<Phone> {
    dtos.into_iter().map(Phone::from).collect()
}

pub fn addresses_to_persistence(addresses: &[Address]) -> Vec<AddressPersistenceDto> {
    addresses.iter().map(AddressPersistenceDto::from).collect()
}

pub fn addresses_from_persistence(dtos: Vec<AddressPersistenceDto>) -> Vec<Address> {
    dtos.into_iter().map(Address::from).collect()
}
