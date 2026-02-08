use async_trait::async_trait;
use chrono::Utc;
use sqlx::types::Uuid;
use std::sync::Arc;

use crate::application::dtos::address_book_dto::{
    AddressBookDto, CreateAddressBookDto, UpdateAddressBookDto,
    ShareAddressBookDto, UnshareAddressBookDto
};
use crate::application::dtos::contact_dto::{
    ContactDto, CreateContactDto, UpdateContactDto, CreateContactVCardDto,
    ContactGroupDto, CreateContactGroupDto, UpdateContactGroupDto, GroupMembershipDto
};
use crate::application::ports::carddav_ports::{AddressBookUseCase, ContactUseCase};
use crate::application::ports::storage_ports::StorageUseCase;
use crate::common::errors::DomainError;
use crate::domain::entities::contact::{AddressBook, Contact, ContactGroup, Email, Phone, Address};
use crate::domain::repositories::address_book_repository::AddressBookRepository;
use crate::domain::repositories::contact_repository::{ContactRepository, ContactGroupRepository};

pub struct ContactService {
    address_book_repository: Arc<dyn AddressBookRepository>,
    contact_repository: Arc<dyn ContactRepository>,
    contact_group_repository: Arc<dyn ContactGroupRepository>,
}

impl ContactService {
    pub fn new(
        address_book_repository: Arc<dyn AddressBookRepository>,
        contact_repository: Arc<dyn ContactRepository>,
        contact_group_repository: Arc<dyn ContactGroupRepository>,
    ) -> Self {
        Self {
            address_book_repository,
            contact_repository,
            contact_group_repository,
        }
    }

    // Helper methods
    async fn check_address_book_access(&self, address_book_id: &Uuid, user_id: &str) -> Result<AddressBook, DomainError> {
        let address_book = self.address_book_repository.get_address_book_by_id(address_book_id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        // Check if user is owner
        if address_book.owner_id() == user_id {
            return Ok(address_book);
        }

        // Check if address book is shared with user
        let shares = self.address_book_repository.get_address_book_shares(address_book_id).await?;
        if shares.iter().any(|(id, _)| id == user_id) {
            return Ok(address_book);
        }

        // Check if address book is public
        if address_book.is_public() {
            return Ok(address_book);
        }

        Err(DomainError::unauthorized("You don't have access to this address book"))
    }

    async fn check_address_book_write_access(&self, address_book_id: &Uuid, user_id: &str) -> Result<AddressBook, DomainError> {
        let address_book = self.address_book_repository.get_address_book_by_id(address_book_id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        // Check if user is owner
        if address_book.owner_id() == user_id {
            return Ok(address_book);
        }

        // Check if address book is shared with user with write access
        let shares = self.address_book_repository.get_address_book_shares(address_book_id).await?;
        if shares.iter().any(|(id, can_write)| id == user_id && *can_write) {
            return Ok(address_book);
        }

        Err(DomainError::unauthorized("You don't have write access to this address book"))
    }

    fn parse_vcard(&self, vcard_data: &str) -> Result<Contact, DomainError> {
        // This is a simplified vCard parser - a real implementation would use a proper vCard library
        // For now, we'll create a basic contact with minimal data
        
        let mut contact = Contact::default();
        
        let lines: Vec<&str> = vcard_data.lines().collect();
        
        for i in 0..lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("FN:") {
                contact.set_full_name(Some(line[3..].to_string()));
            } else if line.starts_with("N:") {
                let parts: Vec<&str> = line[2..].split(';').collect();
                if parts.len() >= 2 {
                    contact.set_last_name(Some(parts[0].to_string()));
                    contact.set_first_name(Some(parts[1].to_string()));
                }
            } else if line.starts_with("EMAIL") {
                let value = line.split(':').nth(1).unwrap_or("");
                if !value.is_empty() {
                    let email_type = if line.contains("TYPE=HOME") {
                        "home"
                    } else if line.contains("TYPE=WORK") {
                        "work"
                    } else {
                        "other"
                    };
                    
                    contact.push_email(Email {
                        email: value.to_string(),
                        r#type: email_type.to_string(),
                        is_primary: contact.email_is_empty(), // First one is primary
                    });
                }
            } else if line.starts_with("TEL") {
                let value = line.split(':').nth(1).unwrap_or("");
                if !value.is_empty() {
                    let phone_type = if line.contains("TYPE=CELL") || line.contains("TYPE=MOBILE") {
                        "mobile"
                    } else if line.contains("TYPE=HOME") {
                        "home"
                    } else if line.contains("TYPE=WORK") {
                        "work"
                    } else if line.contains("TYPE=FAX") {
                        "fax"
                    } else {
                        "other"
                    };
                    
                    contact.push_phone(Phone {
                        number: value.to_string(),
                        r#type: phone_type.to_string(),
                        is_primary: contact.phone_is_empty(), // First one is primary
                    });
                }
            } else if line.starts_with("ORG:") {
                contact.set_organization(Some(line[4..].to_string()));
            } else if line.starts_with("TITLE:") {
                contact.set_title(Some(line[6..].to_string()));
            } else if line.starts_with("NOTE:") {
                contact.set_notes(Some(line[5..].to_string()));
            } else if line.starts_with("UID:") {
                contact.set_uid(line[4..].to_string());
            }
        }
        
        // Store the original vCard data
        contact.set_vcard(vcard_data.to_string());
        contact.set_etag(Uuid::new_v4().to_string());
        
        Ok(contact)
    }

    fn generate_vcard(&self, contact: &Contact) -> String {
        let mut vcard = String::from("BEGIN:VCARD\r\nVERSION:3.0\r\n");
        
        // UID
        vcard.push_str(&format!("UID:{}\r\n", contact.uid()));
        
        // Name fields
        if let Some(full_name) = contact.full_name() {
            vcard.push_str(&format!("FN:{}\r\n", full_name));
        }
        
        let last_name = contact.last_name().unwrap_or_default().to_string();
        let first_name = contact.first_name().unwrap_or_default().to_string();
        vcard.push_str(&format!("N:{};{};;;\r\n", last_name, first_name));
        
        // Email addresses
        for email in contact.email() {
            vcard.push_str(&format!("EMAIL;TYPE={}:{}\r\n", 
                email.r#type.to_uppercase(),
                email.email));
        }
        
        // Phone numbers
        for phone in contact.phone() {
            let tel_type = match phone.r#type.as_str() {
                "mobile" => "CELL",
                "home" => "HOME",
                "work" => "WORK",
                "fax" => "FAX",
                _ => "OTHER",
            };
            vcard.push_str(&format!("TEL;TYPE={}:{}\r\n", tel_type, phone.number));
        }
        
        // Addresses
        for addr in contact.address() {
            let addr_type = addr.r#type.to_uppercase();
            let street = addr.street.clone().unwrap_or_default();
            let city = addr.city.clone().unwrap_or_default();
            let state = addr.state.clone().unwrap_or_default();
            let postal_code = addr.postal_code.clone().unwrap_or_default();
            let country = addr.country.clone().unwrap_or_default();
            
            vcard.push_str(&format!("ADR;TYPE={}:;;{};{};{};{};{}\r\n", 
                addr_type, street, city, state, postal_code, country));
        }
        
        // Organization
        if let Some(org) = contact.organization() {
            vcard.push_str(&format!("ORG:{}\r\n", org));
        }
        
        // Title
        if let Some(title) = contact.title() {
            vcard.push_str(&format!("TITLE:{}\r\n", title));
        }
        
        // Notes
        if let Some(notes) = contact.notes() {
            vcard.push_str(&format!("NOTE:{}\r\n", notes));
        }
        
        // Birthday
        if let Some(birthday) = contact.birthday() {
            vcard.push_str(&format!("BDAY:{}\r\n", birthday.format("%Y%m%d")));
        }
        
        // Revision (last update)
        vcard.push_str(&format!("REV:{}\r\n", contact.updated_at().format("%Y%m%dT%H%M%SZ")));
        
        vcard.push_str("END:VCARD\r\n");
        
        vcard
    }
}

#[async_trait]
impl AddressBookUseCase for ContactService {
    async fn create_address_book(&self, dto: CreateAddressBookDto) -> Result<AddressBookDto, DomainError> {
        let address_book = AddressBook::new(
            dto.name,
            dto.owner_id,
            dto.description,
            dto.color,
            dto.is_public.unwrap_or(false),
        );

        let created_address_book = self.address_book_repository.create_address_book(address_book).await?;
        Ok(AddressBookDto::from(created_address_book))
    }

    async fn update_address_book(&self, address_book_id: &str, update: UpdateAddressBookDto) -> Result<AddressBookDto, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has write access to the address book
        let address_book = self.check_address_book_write_access(&id, &update.user_id).await?;

        // Apply updates
        let updated_address_book = AddressBook::from_raw(
            id,
            update.name.unwrap_or_else(|| address_book.name().to_string()),
            address_book.owner_id().to_string(),
            update.description.or_else(|| address_book.description().map(|s| s.to_string())),
            update.color.or_else(|| address_book.color().map(|s| s.to_string())),
            update.is_public.unwrap_or(address_book.is_public()),
            *address_book.created_at(),
            Utc::now(),
        );

        let result = self.address_book_repository.update_address_book(updated_address_book).await?;
        Ok(AddressBookDto::from(result))
    }

    async fn delete_address_book(&self, address_book_id: &str, user_id: &str) -> Result<(), DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Verify that the user is the owner of the address book
        let address_book = self.address_book_repository.get_address_book_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::unauthorized("Only the owner can delete an address book"));
        }

        self.address_book_repository.delete_address_book(&id).await?;
        Ok(())
    }

    async fn get_address_book(&self, address_book_id: &str, user_id: &str) -> Result<AddressBookDto, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        let address_book = self.check_address_book_access(&id, user_id).await?;
        Ok(AddressBookDto::from(address_book))
    }

    async fn list_user_address_books(&self, user_id: &str) -> Result<Vec<AddressBookDto>, DomainError> {
        // Get address books owned by the user
        let owned_address_books = self.address_book_repository.get_address_books_by_owner(user_id).await?;
        
        // Get address books shared with the user
        let shared_address_books = self.address_book_repository.get_shared_address_books(user_id).await?;
        
        // Get public address books
        let public_address_books = self.address_book_repository.get_public_address_books().await?;
        
        // Combine all address books, avoiding duplicates
        let mut address_book_map = std::collections::HashMap::new();
        
        for address_book in owned_address_books {
            address_book_map.insert(*address_book.id(), address_book);
        }
        
        for address_book in shared_address_books {
            address_book_map.insert(*address_book.id(), address_book);
        }
        
        for address_book in public_address_books {
            if address_book.owner_id() != user_id && !address_book_map.contains_key(address_book.id()) {
                address_book_map.insert(*address_book.id(), address_book);
            }
        }
        
        let address_books: Vec<AddressBookDto> = address_book_map.values()
            .cloned()
            .map(AddressBookDto::from)
            .collect();
        
        Ok(address_books)
    }

    async fn list_public_address_books(&self) -> Result<Vec<AddressBookDto>, DomainError> {
        let address_books = self.address_book_repository.get_public_address_books().await?;
        let dtos: Vec<AddressBookDto> = address_books.into_iter().map(AddressBookDto::from).collect();
        Ok(dtos)
    }

    async fn share_address_book(&self, dto: ShareAddressBookDto, user_id: &str) -> Result<(), DomainError> {
        let id = Uuid::parse_str(&dto.address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Verify that the user is the owner of the address book
        let address_book = self.address_book_repository.get_address_book_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::unauthorized("Only the owner can share an address book"));
        }

        // Don't allow sharing with yourself
        if dto.user_id == user_id {
            return Err(DomainError::validation_error("Cannot share an address book with yourself"));
        }

        self.address_book_repository.share_address_book(&id, &dto.user_id, dto.can_write).await?;
        Ok(())
    }

    async fn unshare_address_book(&self, dto: UnshareAddressBookDto, user_id: &str) -> Result<(), DomainError> {
        let id = Uuid::parse_str(&dto.address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Verify that the user is the owner of the address book
        let address_book = self.address_book_repository.get_address_book_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::unauthorized("Only the owner can unshare an address book"));
        }

        self.address_book_repository.unshare_address_book(&id, &dto.user_id).await?;
        Ok(())
    }

    async fn get_address_book_shares(&self, address_book_id: &str, user_id: &str) -> Result<Vec<(String, bool)>, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Verify that the user is the owner of the address book
        let address_book = self.address_book_repository.get_address_book_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Address book", "not found"))?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::unauthorized("Only the owner can view address book shares"));
        }

        let shares = self.address_book_repository.get_address_book_shares(&id).await?;
        Ok(shares)
    }
}

#[async_trait]
impl ContactUseCase for ContactService {
    async fn create_contact(&self, dto: CreateContactDto) -> Result<ContactDto, DomainError> {
        let address_book_id = Uuid::parse_str(&dto.address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(&address_book_id, &dto.user_id).await?;

        // Convert DTOs to domain entities
        let email: Vec<Email> = dto.email.into_iter()
            .map(|e| Email {
                email: e.email,
                r#type: e.r#type,
                is_primary: e.is_primary,
            })
            .collect();

        let phone: Vec<Phone> = dto.phone.into_iter()
            .map(|p| Phone {
                number: p.number,
                r#type: p.r#type,
                is_primary: p.is_primary,
            })
            .collect();

        let address: Vec<Address> = dto.address.into_iter()
            .map(|a| Address {
                street: a.street,
                city: a.city,
                state: a.state,
                postal_code: a.postal_code,
                country: a.country,
                r#type: a.r#type,
                is_primary: a.is_primary,
            })
            .collect();

        let mut contact = Contact::new(
            address_book_id,
            dto.full_name,
            dto.first_name,
            dto.last_name,
            dto.nickname,
            email,
            phone,
            address,
            dto.organization,
            dto.title,
            dto.notes,
            dto.photo_url,
            dto.birthday,
            dto.anniversary,
            String::new(), // Will be generated after creation
        );

        // Generate vCard data
        let vcard = self.generate_vcard(&contact);
        contact.set_vcard(vcard);
        let contact_with_vcard = contact;

        // Create the contact
        let created_contact = self.contact_repository.create_contact(contact_with_vcard).await?;
        Ok(ContactDto::from(created_contact))
    }

    async fn create_contact_from_vcard(&self, dto: CreateContactVCardDto) -> Result<ContactDto, DomainError> {
        let address_book_id = Uuid::parse_str(&dto.address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(&address_book_id, &dto.user_id).await?;

        // Parse vCard data
        let mut contact = self.parse_vcard(&dto.vcard)?;
        
        // Set address book ID
        contact.set_address_book_id(address_book_id);
        
        // The contact was created with Contact::default() which generates a new ID
        // Set creation and update timestamps
        let now = Utc::now();
        contact.set_updated_at(now);
        
        // Create the contact
        let created_contact = self.contact_repository.create_contact(contact).await?;
        Ok(ContactDto::from(created_contact))
    }

    async fn update_contact(&self, contact_id: &str, update: UpdateContactDto) -> Result<ContactDto, DomainError> {
        let id = Uuid::parse_str(contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the current contact
        let contact = self.contact_repository.get_contact_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(contact.address_book_id(), &update.user_id).await?;

        // Destructure contact into owned parts for updates
        let parts = contact.into_parts();

        // Convert DTO fields to domain entities
        let email = if let Some(email_dtos) = update.email {
            email_dtos.into_iter()
                .map(|e| Email {
                    email: e.email,
                    r#type: e.r#type,
                    is_primary: e.is_primary,
                })
                .collect()
        } else {
            parts.email
        };

        let phone = if let Some(phone_dtos) = update.phone {
            phone_dtos.into_iter()
                .map(|p| Phone {
                    number: p.number,
                    r#type: p.r#type,
                    is_primary: p.is_primary,
                })
                .collect()
        } else {
            parts.phone
        };

        let address = if let Some(address_dtos) = update.address {
            address_dtos.into_iter()
                .map(|a| Address {
                    street: a.street,
                    city: a.city,
                    state: a.state,
                    postal_code: a.postal_code,
                    country: a.country,
                    r#type: a.r#type,
                    is_primary: a.is_primary,
                })
                .collect()
        } else {
            parts.address
        };

        // Update the contact object
        let mut updated_contact = Contact::from_raw(
            id,
            parts.address_book_id,
            parts.uid,
            update.full_name.or(parts.full_name),
            update.first_name.or(parts.first_name),
            update.last_name.or(parts.last_name),
            update.nickname.or(parts.nickname),
            email,
            phone,
            address,
            update.organization.or(parts.organization),
            update.title.or(parts.title),
            update.notes.or(parts.notes),
            update.photo_url.or(parts.photo_url),
            update.birthday.or(parts.birthday),
            update.anniversary.or(parts.anniversary),
            parts.vcard, // Will be regenerated
            Uuid::new_v4().to_string(), // Generate new ETag
            parts.created_at,
            Utc::now(),
        );

        // Generate new vCard data
        let vcard = self.generate_vcard(&updated_contact);
        updated_contact.set_vcard(vcard);
        let contact_with_vcard = updated_contact;

        // Update the contact
        let result = self.contact_repository.update_contact(contact_with_vcard).await?;
        Ok(ContactDto::from(result))
    }

    async fn delete_contact(&self, contact_id: &str, user_id: &str) -> Result<(), DomainError> {
        let id = Uuid::parse_str(contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the current contact
        let contact = self.contact_repository.get_contact_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(contact.address_book_id(), user_id).await?;

        // Delete the contact
        self.contact_repository.delete_contact(&id).await?;
        Ok(())
    }

    async fn get_contact(&self, contact_id: &str, user_id: &str) -> Result<ContactDto, DomainError> {
        let id = Uuid::parse_str(contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the contact
        let contact = self.contact_repository.get_contact_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact", "not found"))?;

        // Check if user has access to the address book
        self.check_address_book_access(contact.address_book_id(), user_id).await?;

        Ok(ContactDto::from(contact))
    }

    async fn list_contacts(&self, address_book_id: &str, user_id: &str) -> Result<Vec<ContactDto>, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has access to the address book
        self.check_address_book_access(&id, user_id).await?;

        // Get contacts
        let contacts = self.contact_repository.get_contacts_by_address_book(&id).await?;
        let dtos = contacts.into_iter().map(ContactDto::from).collect();
        
        Ok(dtos)
    }

    async fn search_contacts(&self, address_book_id: &str, query: &str, user_id: &str) -> Result<Vec<ContactDto>, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has access to the address book
        self.check_address_book_access(&id, user_id).await?;

        // Search contacts
        let contacts = self.contact_repository.search_contacts(&id, query).await?;
        let dtos = contacts.into_iter().map(ContactDto::from).collect();
        
        Ok(dtos)
    }

    async fn create_group(&self, dto: CreateContactGroupDto) -> Result<ContactGroupDto, DomainError> {
        let address_book_id = Uuid::parse_str(&dto.address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(&address_book_id, &dto.user_id).await?;

        let group = ContactGroup::new(
            address_book_id,
            dto.name,
        );

        let created_group = self.contact_group_repository.create_group(group).await?;
        Ok(ContactGroupDto::from(created_group))
    }

    async fn update_group(&self, group_id: &str, update: UpdateContactGroupDto) -> Result<ContactGroupDto, DomainError> {
        let id = Uuid::parse_str(group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;

        // Get the current group
        let group = self.contact_group_repository.get_group_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(group.address_book_id(), &update.user_id).await?;

        // Update the group
        let updated_group = ContactGroup::from_raw(
            id,
            *group.address_book_id(),
            update.name,
            *group.created_at(),
            Utc::now(),
        );

        let result = self.contact_group_repository.update_group(updated_group).await?;
        Ok(ContactGroupDto::from(result))
    }

    async fn delete_group(&self, group_id: &str, user_id: &str) -> Result<(), DomainError> {
        let id = Uuid::parse_str(group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;

        // Get the current group
        let group = self.contact_group_repository.get_group_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(group.address_book_id(), user_id).await?;

        // Delete the group
        self.contact_group_repository.delete_group(&id).await?;
        Ok(())
    }

    async fn get_group(&self, group_id: &str, user_id: &str) -> Result<ContactGroupDto, DomainError> {
        let id = Uuid::parse_str(group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;

        // Get the group
        let group = self.contact_group_repository.get_group_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has access to the address book
        self.check_address_book_access(group.address_book_id(), user_id).await?;

        // Get the number of contacts in the group
        let contacts = self.contact_group_repository.get_contacts_in_group(&id).await?;
        
        let mut dto = ContactGroupDto::from(group);
        dto.members_count = Some(contacts.len() as i32);
        
        Ok(dto)
    }

    async fn list_groups(&self, address_book_id: &str, user_id: &str) -> Result<Vec<ContactGroupDto>, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has access to the address book
        self.check_address_book_access(&id, user_id).await?;

        // Get groups
        let groups = self.contact_group_repository.get_groups_by_address_book(&id).await?;
        let dtos = groups.into_iter().map(ContactGroupDto::from).collect();
        
        Ok(dtos)
    }

    async fn add_contact_to_group(&self, dto: GroupMembershipDto, user_id: &str) -> Result<(), DomainError> {
        let group_id = Uuid::parse_str(&dto.group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;
            
        let contact_id = Uuid::parse_str(&dto.contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the group
        let group = self.contact_group_repository.get_group_by_id(&group_id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(group.address_book_id(), user_id).await?;

        // Add contact to group
        self.contact_group_repository.add_contact_to_group(&group_id, &contact_id).await?;
        Ok(())
    }

    async fn remove_contact_from_group(&self, dto: GroupMembershipDto, user_id: &str) -> Result<(), DomainError> {
        let group_id = Uuid::parse_str(&dto.group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;
            
        let contact_id = Uuid::parse_str(&dto.contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the group
        let group = self.contact_group_repository.get_group_by_id(&group_id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has write access to the address book
        self.check_address_book_write_access(group.address_book_id(), user_id).await?;

        // Remove contact from group
        self.contact_group_repository.remove_contact_from_group(&group_id, &contact_id).await?;
        Ok(())
    }

    async fn list_contacts_in_group(&self, group_id: &str, user_id: &str) -> Result<Vec<ContactDto>, DomainError> {
        let id = Uuid::parse_str(group_id)
            .map_err(|_| DomainError::validation_error("Invalid group ID format"))?;

        // Get the group
        let group = self.contact_group_repository.get_group_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact group", "not found"))?;

        // Check if user has access to the address book
        self.check_address_book_access(group.address_book_id(), user_id).await?;

        // Get contacts in group
        let contacts = self.contact_group_repository.get_contacts_in_group(&id).await?;
        let dtos = contacts.into_iter().map(ContactDto::from).collect();
        
        Ok(dtos)
    }

    async fn list_groups_for_contact(&self, contact_id: &str, user_id: &str) -> Result<Vec<ContactGroupDto>, DomainError> {
        let id = Uuid::parse_str(contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the contact
        let contact = self.contact_repository.get_contact_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact", "not found"))?;

        // Check if user has access to the address book
        self.check_address_book_access(contact.address_book_id(), user_id).await?;

        // Get groups for contact
        let groups = self.contact_group_repository.get_groups_for_contact(&id).await?;
        let dtos = groups.into_iter().map(ContactGroupDto::from).collect();
        
        Ok(dtos)
    }

    async fn get_contact_vcard(&self, contact_id: &str, user_id: &str) -> Result<String, DomainError> {
        let id = Uuid::parse_str(contact_id)
            .map_err(|_| DomainError::validation_error("Invalid contact ID format"))?;

        // Get the contact
        let contact = self.contact_repository.get_contact_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::not_found("Contact", "not found"))?;

        // Check if user has access to the address book
        self.check_address_book_access(contact.address_book_id(), user_id).await?;

        // Return the vCard data
        Ok(contact.vcard().to_string())
    }

    async fn get_contacts_as_vcards(&self, address_book_id: &str, user_id: &str) -> Result<Vec<(String, String)>, DomainError> {
        let id = Uuid::parse_str(address_book_id)
            .map_err(|_| DomainError::validation_error("Invalid address book ID format"))?;

        // Check if user has access to the address book
        self.check_address_book_access(&id, user_id).await?;

        // Get all contacts in the address book
        let contacts = self.contact_repository.get_contacts_by_address_book(&id).await?;
        
        // Convert to Vec<(id, vcard)>
        let vcards = contacts.into_iter()
            .map(|contact| (contact.id().to_string(), contact.vcard().to_string()))
            .collect();
        
        Ok(vcards)
    }
}

#[async_trait]
impl StorageUseCase for ContactService {
    async fn handle_request(&self, action: &str, params: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        match action {
            // Address Book operations
            "create_address_book" => {
                let dto: CreateAddressBookDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.create_address_book(dto).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "update_address_book" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let update: UpdateAddressBookDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.update_address_book(address_book_id, update).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "delete_address_book" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.delete_address_book(address_book_id, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "get_address_book" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_address_book(address_book_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "list_user_address_books" => {
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.list_user_address_books(user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "list_public_address_books" => {
                let result = self.list_public_address_books().await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "share_address_book" => {
                let dto: ShareAddressBookDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.share_address_book(dto, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "unshare_address_book" => {
                let dto: UnshareAddressBookDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.unshare_address_book(dto, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "get_address_book_shares" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_address_book_shares(address_book_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },

            // Contact operations
            "create_contact" => {
                let dto: CreateContactDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.create_contact(dto).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "create_contact_from_vcard" => {
                let dto: CreateContactVCardDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.create_contact_from_vcard(dto).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "update_contact" => {
                let contact_id = params["contact_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing contact_id parameter"))?;
                
                let update: UpdateContactDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.update_contact(contact_id, update).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "delete_contact" => {
                let contact_id = params["contact_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing contact_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.delete_contact(contact_id, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "get_contact" => {
                let contact_id = params["contact_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing contact_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_contact(contact_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "list_contacts" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.list_contacts(address_book_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "search_contacts" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let query = params["query"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing query parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.search_contacts(address_book_id, query, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },

            // Group operations
            "create_group" => {
                let dto: CreateContactGroupDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.create_group(dto).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "update_group" => {
                let group_id = params["group_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing group_id parameter"))?;
                
                let update: UpdateContactGroupDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let result = self.update_group(group_id, update).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "delete_group" => {
                let group_id = params["group_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing group_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.delete_group(group_id, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "get_group" => {
                let group_id = params["group_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing group_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_group(group_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "list_groups" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.list_groups(address_book_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },

            // Group membership operations
            "add_contact_to_group" => {
                let dto: GroupMembershipDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.add_contact_to_group(dto, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "remove_contact_from_group" => {
                let dto: GroupMembershipDto = serde_json::from_value(params.clone())
                    .map_err(|e| DomainError::validation_error(format!("Invalid parameters: {}", e)))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                self.remove_contact_from_group(dto, user_id).await?;
                Ok(serde_json::Value::Null)
            },
            "list_contacts_in_group" => {
                let group_id = params["group_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing group_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.list_contacts_in_group(group_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "list_groups_for_contact" => {
                let contact_id = params["contact_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing contact_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.list_groups_for_contact(contact_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },

            // vCard operations
            "get_contact_vcard" => {
                let contact_id = params["contact_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing contact_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_contact_vcard(contact_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            "get_contacts_as_vcards" => {
                let address_book_id = params["address_book_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing address_book_id parameter"))?;
                
                let user_id = params["user_id"].as_str()
                    .ok_or_else(|| DomainError::validation_error("Missing user_id parameter"))?;
                
                let result = self.get_contacts_as_vcards(address_book_id, user_id).await?;
                Ok(serde_json::to_value(result).unwrap())
            },
            
            _ => Err(DomainError::validation_error(format!("Unknown action: {}", action))),
        }
    }
}