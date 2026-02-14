use crate::application::dtos::address_book_dto::{
    AddressBookDto, CreateAddressBookDto, ShareAddressBookDto, UnshareAddressBookDto,
    UpdateAddressBookDto,
};
use crate::application::dtos::contact_dto::{
    ContactDto, ContactGroupDto, CreateContactDto, CreateContactGroupDto, CreateContactVCardDto,
    GroupMembershipDto, UpdateContactDto, UpdateContactGroupDto,
};
use crate::common::errors::DomainError;
use async_trait::async_trait;

pub type CardDavRepositoryError = DomainError;

#[async_trait]
pub trait AddressBookUseCase: Send + Sync + 'static {
    // Address Book operations
    async fn create_address_book(
        &self,
        dto: CreateAddressBookDto,
    ) -> Result<AddressBookDto, DomainError>;
    async fn update_address_book(
        &self,
        address_book_id: &str,
        update: UpdateAddressBookDto,
    ) -> Result<AddressBookDto, DomainError>;
    async fn delete_address_book(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn get_address_book(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<AddressBookDto, DomainError>;
    async fn list_user_address_books(
        &self,
        user_id: &str,
    ) -> Result<Vec<AddressBookDto>, DomainError>;
    async fn list_public_address_books(&self) -> Result<Vec<AddressBookDto>, DomainError>;

    // Address Book sharing
    async fn share_address_book(
        &self,
        dto: ShareAddressBookDto,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn unshare_address_book(
        &self,
        dto: UnshareAddressBookDto,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn get_address_book_shares(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<(String, bool)>, DomainError>;
}

#[async_trait]
pub trait ContactUseCase: Send + Sync + 'static {
    // Contact operations
    async fn create_contact(&self, dto: CreateContactDto) -> Result<ContactDto, DomainError>;
    async fn create_contact_from_vcard(
        &self,
        dto: CreateContactVCardDto,
    ) -> Result<ContactDto, DomainError>;
    async fn update_contact(
        &self,
        contact_id: &str,
        update: UpdateContactDto,
    ) -> Result<ContactDto, DomainError>;
    async fn delete_contact(&self, contact_id: &str, user_id: &str) -> Result<(), DomainError>;
    async fn get_contact(&self, contact_id: &str, user_id: &str)
    -> Result<ContactDto, DomainError>;
    async fn list_contacts(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError>;
    async fn search_contacts(
        &self,
        address_book_id: &str,
        query: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError>;

    // Contact Group operations
    async fn create_group(
        &self,
        dto: CreateContactGroupDto,
    ) -> Result<ContactGroupDto, DomainError>;
    async fn update_group(
        &self,
        group_id: &str,
        update: UpdateContactGroupDto,
    ) -> Result<ContactGroupDto, DomainError>;
    async fn delete_group(&self, group_id: &str, user_id: &str) -> Result<(), DomainError>;
    async fn get_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<ContactGroupDto, DomainError>;
    async fn list_groups(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactGroupDto>, DomainError>;

    // Group membership
    async fn add_contact_to_group(
        &self,
        dto: GroupMembershipDto,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn remove_contact_from_group(
        &self,
        dto: GroupMembershipDto,
        user_id: &str,
    ) -> Result<(), DomainError>;
    async fn list_contacts_in_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactDto>, DomainError>;
    async fn list_groups_for_contact(
        &self,
        contact_id: &str,
        user_id: &str,
    ) -> Result<Vec<ContactGroupDto>, DomainError>;

    // vCard operations
    async fn get_contact_vcard(
        &self,
        contact_id: &str,
        user_id: &str,
    ) -> Result<String, DomainError>;
    async fn get_contacts_as_vcards(
        &self,
        address_book_id: &str,
        user_id: &str,
    ) -> Result<Vec<(String, String)>, DomainError>;
}
