//! Contact Storage Adapter
//!
//! This adapter implements the `AddressBookUseCase` and `ContactUseCase` application ports
//! using the domain repositories. It bridges the gap between the application layer
//! and the infrastructure layer for CardDAV functionality.

use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::address_book_dto::{
    AddressBookDto, CreateAddressBookDto, ShareAddressBookDto, UnshareAddressBookDto,
    UpdateAddressBookDto,
};
use crate::application::dtos::contact_dto::{
    AddressDto, ContactDto, ContactGroupDto, CreateContactDto, CreateContactGroupDto,
    CreateContactVCardDto, EmailDto, GroupMembershipDto, PhoneDto, UpdateContactDto,
    UpdateContactGroupDto,
};
use crate::application::ports::carddav_ports::{AddressBookUseCase, ContactUseCase};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::contact::{Address, AddressBook, Contact, ContactGroup, Email, Phone};
use crate::domain::repositories::address_book_repository::AddressBookRepository;
use crate::domain::repositories::contact_repository::{ContactGroupRepository, ContactRepository};

/// Adapter that implements AddressBookUseCase and ContactUseCase using domain repositories
pub struct ContactStorageAdapter {
    address_book_repository: Arc<dyn AddressBookRepository>,
    contact_repository: Arc<dyn ContactRepository>,
    group_repository: Arc<dyn ContactGroupRepository>,
}

impl ContactStorageAdapter {
    /// Creates a new ContactStorageAdapter with the given repositories
    pub fn new(
        address_book_repository: Arc<dyn AddressBookRepository>,
        contact_repository: Arc<dyn ContactRepository>,
        group_repository: Arc<dyn ContactGroupRepository>,
    ) -> Self {
        Self {
            address_book_repository,
            contact_repository,
            group_repository,
        }
    }

    /// Helper to parse UUID from string
    fn parse_uuid(id: &str, entity_name: &'static str) -> Result<Uuid, DomainError> {
        Uuid::parse_str(id).map_err(|_| {
            DomainError::new(
                ErrorKind::InvalidInput,
                entity_name,
                format!("Invalid {} ID format", entity_name),
            )
        })
    }

    /// Helper to check if user has access to an address book
    async fn check_address_book_access(
        &self,
        address_book_id: &Uuid,
        user_id: &str,
    ) -> Result<AddressBook, DomainError> {
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(address_book_id)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        // Check if user is owner
        if address_book.owner_id() == user_id {
            return Ok(address_book);
        }

        // Check if address book is public
        if address_book.is_public() {
            return Ok(address_book);
        }

        // Check if address book is shared with user
        let shares = self
            .address_book_repository
            .get_address_book_shares(address_book_id)
            .await?;
        if shares.iter().any(|(shared_user, _)| shared_user == user_id) {
            return Ok(address_book);
        }

        Err(DomainError::new(
            ErrorKind::AccessDenied,
            "AddressBook",
            "Access denied to address book",
        ))
    }

    /// Helper to check write access
    async fn check_write_access(
        &self,
        address_book_id: &Uuid,
        user_id: &str,
    ) -> Result<AddressBook, DomainError> {
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(address_book_id)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        // Owner always has write access
        if address_book.owner_id() == user_id {
            return Ok(address_book);
        }

        // Check shares for write permission
        let shares = self
            .address_book_repository
            .get_address_book_shares(address_book_id)
            .await?;
        if shares
            .iter()
            .any(|(shared_user, can_write)| shared_user == user_id && *can_write)
        {
            return Ok(address_book);
        }

        Err(DomainError::new(
            ErrorKind::AccessDenied,
            "AddressBook",
            "Write access denied",
        ))
    }

    /// Convert EmailDto to domain Email
    fn dto_to_email(dto: EmailDto) -> Email {
        Email {
            email: dto.email,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }

    /// Convert PhoneDto to domain Phone
    fn dto_to_phone(dto: PhoneDto) -> Phone {
        Phone {
            number: dto.number,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }

    /// Convert AddressDto to domain Address
    fn dto_to_address(dto: AddressDto) -> Address {
        Address {
            street: dto.street,
            city: dto.city,
            state: dto.state,
            postal_code: dto.postal_code,
            country: dto.country,
            r#type: dto.r#type,
            is_primary: dto.is_primary,
        }
    }

    /// Generate vCard from contact data
    fn generate_vcard(contact: &Contact) -> String {
        let mut vcard = String::from("BEGIN:VCARD\nVERSION:3.0\n");

        if let Some(full_name) = contact.full_name() {
            vcard.push_str(&format!("FN:{}\n", full_name));
        }

        if contact.first_name().is_some() || contact.last_name().is_some() {
            let last = contact.last_name().unwrap_or("");
            let first = contact.first_name().unwrap_or("");
            vcard.push_str(&format!("N:{};{};;;\n", last, first));
        }

        if let Some(nickname) = contact.nickname() {
            vcard.push_str(&format!("NICKNAME:{}\n", nickname));
        }

        for email in contact.email() {
            vcard.push_str(&format!(
                "EMAIL;TYPE={}:{}\n",
                email.r#type.to_uppercase(),
                email.email
            ));
        }

        for phone in contact.phone() {
            vcard.push_str(&format!(
                "TEL;TYPE={}:{}\n",
                phone.r#type.to_uppercase(),
                phone.number
            ));
        }

        if let Some(org) = contact.organization() {
            vcard.push_str(&format!("ORG:{}\n", org));
        }

        if let Some(title) = contact.title() {
            vcard.push_str(&format!("TITLE:{}\n", title));
        }

        if let Some(notes) = contact.notes() {
            vcard.push_str(&format!("NOTE:{}\n", notes));
        }

        vcard.push_str(&format!("UID:{}\n", contact.uid()));
        vcard.push_str("END:VCARD\n");

        vcard
    }
}

#[async_trait]
impl AddressBookUseCase for ContactStorageAdapter {
    async fn create_address_book(
        &self,
        dto: CreateAddressBookDto,
    ) -> Result<AddressBookDto, DomainError> {
        let address_book = AddressBook::new(
            dto.name,
            dto.owner_id,
            dto.description,
            dto.color,
            dto.is_public.unwrap_or(false),
        );

        let created = self
            .address_book_repository
            .create_address_book(address_book)
            .await?;
        Ok(AddressBookDto::from(created))
    }

    async fn update_address_book(
        &self,
        address_book_id: &str,
        update: UpdateAddressBookDto,
    ) -> Result<AddressBookDto, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Check write access
        let mut address_book = self.check_write_access(&uuid, &update.user_id).await?;

        if let Some(name) = update.name {
            address_book.set_name(name);
        }
        if let Some(description) = update.description {
            address_book.set_description(Some(description));
        }
        if let Some(color) = update.color {
            address_book.set_color(Some(color));
        }
        if let Some(is_public) = update.is_public {
            address_book.set_is_public(is_public);
        }
        address_book.set_updated_at(chrono::Utc::now());

        let updated = self
            .address_book_repository
            .update_address_book(address_book)
            .await?;
        Ok(AddressBookDto::from(updated))
    }

    async fn delete_address_book(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Only owner can delete
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "AddressBook",
                "Only owner can delete address book",
            ));
        }

        self.address_book_repository
            .delete_address_book(&uuid)
            .await
    }

    async fn get_address_book(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<AddressBookDto, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;
        let address_book = self.check_address_book_access(&uuid, user_id).await?;
        Ok(AddressBookDto::from(address_book))
    }

    async fn list_user_address_books(
        &self,
        user_id: &str,
    ) -> Result<Vec<AddressBookDto>, DomainError> {
        let owned = self
            .address_book_repository
            .get_address_books_by_owner(user_id)
            .await?;
        let shared = self
            .address_book_repository
            .get_shared_address_books(user_id)
            .await?;

        let mut all_books: Vec<AddressBook> = owned;
        all_books.extend(shared);

        Ok(all_books.into_iter().map(AddressBookDto::from).collect())
    }

    async fn list_public_address_books(&self) -> Result<Vec<AddressBookDto>, DomainError> {
        let public = self
            .address_book_repository
            .get_public_address_books()
            .await?;
        Ok(public.into_iter().map(AddressBookDto::from).collect())
    }

    async fn share_address_book(
        &self,
        dto: ShareAddressBookDto,
        user_id: &str,
    ) -> Result<(), DomainError> {
        let uuid = Self::parse_uuid(&dto.address_book_id, "AddressBook")?;

        // Only owner can share
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "AddressBook",
                "Only owner can share",
            ));
        }

        self.address_book_repository
            .share_address_book(&uuid, &dto.user_id, dto.can_write)
            .await
    }

    async fn unshare_address_book(
        &self,
        dto: UnshareAddressBookDto,
        user_id: &str,
    ) -> Result<(), DomainError> {
        let uuid = Self::parse_uuid(&dto.address_book_id, "AddressBook")?;

        // Only owner can unshare
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "AddressBook",
                "Only owner can unshare",
            ));
        }

        self.address_book_repository
            .unshare_address_book(&uuid, &dto.user_id)
            .await
    }

    async fn get_address_book_shares(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<(String, bool)>, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Only owner can view shares
        let address_book = self
            .address_book_repository
            .get_address_book_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "AddressBook", "Address book not found")
            })?;

        if address_book.owner_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "AddressBook",
                "Only owner can view shares",
            ));
        }

        self.address_book_repository
            .get_address_book_shares(&uuid)
            .await
    }
}

#[async_trait]
impl ContactUseCase for ContactStorageAdapter {
    async fn create_contact(&self, dto: CreateContactDto) -> Result<ContactDto, DomainError> {
        let address_book_id = Self::parse_uuid(&dto.address_book_id, "AddressBook")?;

        // Check write access
        self.check_write_access(&address_book_id, &dto.user_id)
            .await?;

        let now = chrono::Utc::now();
        let mut contact = Contact::from_raw(
            Uuid::new_v4(),
            address_book_id,
            format!("{}@oxicloud", Uuid::new_v4()),
            dto.full_name,
            dto.first_name,
            dto.last_name,
            dto.nickname,
            dto.email.into_iter().map(Self::dto_to_email).collect(),
            dto.phone.into_iter().map(Self::dto_to_phone).collect(),
            dto.address.into_iter().map(Self::dto_to_address).collect(),
            dto.organization,
            dto.title,
            dto.notes,
            dto.photo_url,
            dto.birthday,
            dto.anniversary,
            String::new(),
            Uuid::new_v4().to_string(),
            now,
            now,
        );

        // Generate vCard
        let vcard = Self::generate_vcard(&contact);
        contact.set_vcard(vcard);

        let created = self.contact_repository.create_contact(contact).await?;
        Ok(ContactDto::from(created))
    }

    async fn create_contact_from_vcard(
        &self,
        dto: CreateContactVCardDto,
    ) -> Result<ContactDto, DomainError> {
        let address_book_id = Self::parse_uuid(&dto.address_book_id, "AddressBook")?;

        // Check write access
        self.check_write_access(&address_book_id, &dto.user_id)
            .await?;

        // Parse vCard fields
        let now = chrono::Utc::now();
        let vcard_data = &dto.vcard;

        let mut uid: Option<String> = None;
        let mut full_name: Option<String> = None;
        let mut first_name: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut nickname: Option<String> = None;
        let mut organization: Option<String> = None;
        let mut title: Option<String> = None;
        let mut notes: Option<String> = None;
        let mut emails: Vec<Email> = Vec::new();
        let mut phones: Vec<Phone> = Vec::new();

        for line in vcard_data.lines() {
            let trimmed = line.trim();
            if let Some(stripped) = trimmed.strip_prefix("UID:") {
                uid = Some(stripped.trim().to_string());
            } else if let Some(stripped) = trimmed.strip_prefix("FN:") {
                full_name = Some(stripped.trim().to_string());
            } else if let Some(stripped) = trimmed.strip_prefix("N:") {
                let parts: Vec<&str> = stripped.split(';').collect();
                if parts.len() >= 2 {
                    last_name = Some(parts[0].trim().to_string()).filter(|s| !s.is_empty());
                    first_name = Some(parts[1].trim().to_string()).filter(|s| !s.is_empty());
                }
            } else if let Some(stripped) = trimmed.strip_prefix("NICKNAME:") {
                nickname = Some(stripped.trim().to_string());
            } else if let Some(stripped) = trimmed.strip_prefix("ORG:") {
                organization = Some(stripped.trim().to_string());
            } else if let Some(stripped) = trimmed.strip_prefix("TITLE:") {
                title = Some(stripped.trim().to_string());
            } else if let Some(stripped) = trimmed.strip_prefix("NOTE:") {
                notes = Some(stripped.trim().to_string());
            } else if trimmed.starts_with("EMAIL") {
                if let Some(value) = trimmed.split(':').nth(1)
                    && !value.is_empty()
                {
                    let email_type = if trimmed.contains("TYPE=HOME") {
                        "home"
                    } else if trimmed.contains("TYPE=WORK") {
                        "work"
                    } else {
                        "other"
                    };
                    emails.push(Email {
                        email: value.trim().to_string(),
                        r#type: email_type.to_string(),
                        is_primary: emails.is_empty(),
                    });
                }
            } else if trimmed.starts_with("TEL")
                && let Some(value) = trimmed.split(':').nth(1)
                && !value.is_empty()
            {
                let phone_type = if trimmed.contains("TYPE=CELL") || trimmed.contains("TYPE=MOBILE")
                {
                    "mobile"
                } else if trimmed.contains("TYPE=HOME") {
                    "home"
                } else if trimmed.contains("TYPE=WORK") {
                    "work"
                } else {
                    "other"
                };
                phones.push(Phone {
                    number: value.trim().to_string(),
                    r#type: phone_type.to_string(),
                    is_primary: phones.is_empty(),
                });
            }
        }

        let contact_uid = uid.unwrap_or_else(|| format!("{}@oxicloud", Uuid::new_v4()));

        let contact = Contact::from_raw(
            Uuid::new_v4(),
            address_book_id,
            contact_uid,
            full_name,
            first_name,
            last_name,
            nickname,
            emails,
            phones,
            Vec::new(), // addresses — simplified for now
            organization,
            title,
            notes,
            None, // photo_url
            None, // birthday
            None, // anniversary
            dto.vcard,
            Uuid::new_v4().to_string(),
            now,
            now,
        );

        let created = self.contact_repository.create_contact(contact).await?;
        Ok(ContactDto::from(created))
    }

    async fn update_contact(
        &self,
        contact_id: &str,
        update: UpdateContactDto,
    ) -> Result<ContactDto, DomainError> {
        let uuid = Self::parse_uuid(contact_id, "Contact")?;

        let mut contact = self
            .contact_repository
            .get_contact_by_id(&uuid)
            .await?
            .ok_or_else(|| DomainError::new(ErrorKind::NotFound, "Contact", "Contact not found"))?;

        // Check write access to the address book
        self.check_write_access(contact.address_book_id(), &update.user_id)
            .await?;

        if let Some(full_name) = update.full_name {
            contact.set_full_name(Some(full_name));
        }
        if let Some(first_name) = update.first_name {
            contact.set_first_name(Some(first_name));
        }
        if let Some(last_name) = update.last_name {
            contact.set_last_name(Some(last_name));
        }
        if let Some(nickname) = update.nickname {
            contact.set_nickname(Some(nickname));
        }
        if let Some(emails) = update.email {
            contact.set_email(emails.into_iter().map(Self::dto_to_email).collect());
        }
        if let Some(phones) = update.phone {
            contact.set_phone(phones.into_iter().map(Self::dto_to_phone).collect());
        }
        if let Some(addresses) = update.address {
            contact.set_address(addresses.into_iter().map(Self::dto_to_address).collect());
        }
        if let Some(organization) = update.organization {
            contact.set_organization(Some(organization));
        }
        if let Some(title) = update.title {
            contact.set_title(Some(title));
        }
        if let Some(notes) = update.notes {
            contact.set_notes(Some(notes));
        }
        if let Some(photo_url) = update.photo_url {
            contact.set_photo_url(Some(photo_url));
        }
        if let Some(birthday) = update.birthday {
            contact.set_birthday(Some(birthday));
        }
        if let Some(anniversary) = update.anniversary {
            contact.set_anniversary(Some(anniversary));
        }

        contact.set_updated_at(chrono::Utc::now());
        contact.set_etag(Uuid::new_v4().to_string());
        let vcard = Self::generate_vcard(&contact);
        contact.set_vcard(vcard);

        let updated = self.contact_repository.update_contact(contact).await?;
        Ok(ContactDto::from(updated))
    }

    async fn delete_contact(&self, contact_id: &str, user_id: &str) -> Result<(), DomainError> {
        let uuid = Self::parse_uuid(contact_id, "Contact")?;

        let contact = self
            .contact_repository
            .get_contact_by_id(&uuid)
            .await?
            .ok_or_else(|| DomainError::new(ErrorKind::NotFound, "Contact", "Contact not found"))?;

        // Check write access
        self.check_write_access(contact.address_book_id(), user_id)
            .await?;

        self.contact_repository.delete_contact(&uuid).await
    }

    async fn get_contact(
        &self,
        contact_id: &str,
        user_id: &str,
    ) -> Result<ContactDto, DomainError> {
        let uuid = Self::parse_uuid(contact_id, "Contact")?;

        let contact = self
            .contact_repository
            .get_contact_by_id(&uuid)
            .await?
            .ok_or_else(|| DomainError::new(ErrorKind::NotFound, "Contact", "Contact not found"))?;

        // Check read access
        self.check_address_book_access(contact.address_book_id(), user_id)
            .await?;

        Ok(ContactDto::from(contact))
    }

    async fn list_contacts(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Check read access
        self.check_address_book_access(&uuid, user_id).await?;

        let contacts = self
            .contact_repository
            .get_contacts_by_address_book(&uuid)
            .await?;
        Ok(contacts.into_iter().map(ContactDto::from).collect())
    }

    async fn search_contacts(
        &self,
        address_book_id: &str,
        query: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Check read access
        self.check_address_book_access(&uuid, user_id).await?;

        let contacts = self
            .contact_repository
            .search_contacts(&uuid, query)
            .await?;
        Ok(contacts.into_iter().map(ContactDto::from).collect())
    }

    async fn create_group(
        &self,
        dto: CreateContactGroupDto,
    ) -> Result<ContactGroupDto, DomainError> {
        let address_book_id = Self::parse_uuid(&dto.address_book_id, "AddressBook")?;

        // Check write access
        self.check_write_access(&address_book_id, &dto.user_id)
            .await?;

        let group = ContactGroup::new(address_book_id, dto.name);

        let created = self.group_repository.create_group(group).await?;
        Ok(ContactGroupDto::from(created))
    }

    async fn update_group(
        &self,
        group_id: &str,
        update: UpdateContactGroupDto,
    ) -> Result<ContactGroupDto, DomainError> {
        let uuid = Self::parse_uuid(group_id, "ContactGroup")?;

        let mut group = self
            .group_repository
            .get_group_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check write access
        self.check_write_access(group.address_book_id(), &update.user_id)
            .await?;

        group.set_name(update.name);
        group.set_updated_at(chrono::Utc::now());

        let updated = self.group_repository.update_group(group).await?;
        Ok(ContactGroupDto::from(updated))
    }

    async fn delete_group(&self, group_id: &str, user_id: &str) -> Result<(), DomainError> {
        let uuid = Self::parse_uuid(group_id, "ContactGroup")?;

        let group = self
            .group_repository
            .get_group_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check write access
        self.check_write_access(group.address_book_id(), user_id)
            .await?;

        self.group_repository.delete_group(&uuid).await
    }

    async fn get_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<ContactGroupDto, DomainError> {
        let uuid = Self::parse_uuid(group_id, "ContactGroup")?;

        let group = self
            .group_repository
            .get_group_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check read access
        self.check_address_book_access(group.address_book_id(), user_id)
            .await?;

        Ok(ContactGroupDto::from(group))
    }

    async fn list_groups(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactGroupDto>, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Check read access
        self.check_address_book_access(&uuid, user_id).await?;

        let groups = self
            .group_repository
            .get_groups_by_address_book(&uuid)
            .await?;
        Ok(groups.into_iter().map(ContactGroupDto::from).collect())
    }

    async fn add_contact_to_group(
        &self,
        dto: GroupMembershipDto,
        user_id: &str,
    ) -> Result<(), DomainError> {
        let group_id = Self::parse_uuid(&dto.group_id, "ContactGroup")?;
        let contact_id = Self::parse_uuid(&dto.contact_id, "Contact")?;

        let group = self
            .group_repository
            .get_group_by_id(&group_id)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check write access
        self.check_write_access(group.address_book_id(), user_id)
            .await?;

        self.group_repository
            .add_contact_to_group(&group_id, &contact_id)
            .await
    }

    async fn remove_contact_from_group(
        &self,
        dto: GroupMembershipDto,
        user_id: &str,
    ) -> Result<(), DomainError> {
        let group_id = Self::parse_uuid(&dto.group_id, "ContactGroup")?;
        let contact_id = Self::parse_uuid(&dto.contact_id, "Contact")?;

        let group = self
            .group_repository
            .get_group_by_id(&group_id)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check write access
        self.check_write_access(group.address_book_id(), user_id)
            .await?;

        self.group_repository
            .remove_contact_from_group(&group_id, &contact_id)
            .await
    }

    async fn list_contacts_in_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError> {
        let uuid = Self::parse_uuid(group_id, "ContactGroup")?;

        let group = self
            .group_repository
            .get_group_by_id(&uuid)
            .await?
            .ok_or_else(|| {
                DomainError::new(ErrorKind::NotFound, "ContactGroup", "Group not found")
            })?;

        // Check read access
        self.check_address_book_access(group.address_book_id(), user_id)
            .await?;

        let contacts = self.group_repository.get_contacts_in_group(&uuid).await?;
        Ok(contacts.into_iter().map(ContactDto::from).collect())
    }

    async fn list_groups_for_contact(
        &self,
        contact_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactGroupDto>, DomainError> {
        let uuid = Self::parse_uuid(contact_id, "Contact")?;

        let contact = self
            .contact_repository
            .get_contact_by_id(&uuid)
            .await?
            .ok_or_else(|| DomainError::new(ErrorKind::NotFound, "Contact", "Contact not found"))?;

        // Check read access
        self.check_address_book_access(contact.address_book_id(), user_id)
            .await?;

        let groups = self.group_repository.get_groups_for_contact(&uuid).await?;
        Ok(groups.into_iter().map(ContactGroupDto::from).collect())
    }

    async fn get_contact_vcard(
        &self,
        contact_id: &str,
        user_id: &str,
    ) -> Result<String, DomainError> {
        let uuid = Self::parse_uuid(contact_id, "Contact")?;

        let contact = self
            .contact_repository
            .get_contact_by_id(&uuid)
            .await?
            .ok_or_else(|| DomainError::new(ErrorKind::NotFound, "Contact", "Contact not found"))?;

        // Check read access
        self.check_address_book_access(contact.address_book_id(), user_id)
            .await?;

        Ok(contact.vcard().to_string())
    }

    async fn get_contacts_as_vcards(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<(String, String)>, DomainError> {
        let uuid = Self::parse_uuid(address_book_id, "AddressBook")?;

        // Check read access
        self.check_address_book_access(&uuid, user_id).await?;

        let contacts = self
            .contact_repository
            .get_contacts_by_address_book(&uuid)
            .await?;

        Ok(contacts
            .into_iter()
            .map(|c| (c.id().to_string(), c.vcard().to_string()))
            .collect())
    }
}
