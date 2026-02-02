use async_trait::async_trait;
use uuid::Uuid;
use std::result::Result;

use crate::common::errors::DomainError;
use crate::domain::entities::contact::{Contact, ContactGroup};

pub type ContactRepositoryResult<T> = Result<T, DomainError>;

#[async_trait]
pub trait ContactRepository: Send + Sync + 'static {
    async fn create_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact>;
    async fn update_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact>;
    async fn delete_contact(&self, id: &Uuid) -> ContactRepositoryResult<()>;
    async fn get_contact_by_id(&self, id: &Uuid) -> ContactRepositoryResult<Option<Contact>>;
    async fn get_contact_by_uid(&self, address_book_id: &Uuid, uid: &str) -> ContactRepositoryResult<Option<Contact>>;
    async fn get_contacts_by_address_book(&self, address_book_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>>;
    async fn get_contacts_by_email(&self, email: &str) -> ContactRepositoryResult<Vec<Contact>>;
    async fn get_contacts_by_group(&self, group_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>>;
    async fn search_contacts(&self, address_book_id: &Uuid, query: &str) -> ContactRepositoryResult<Vec<Contact>>;
}

#[async_trait]
pub trait ContactGroupRepository: Send + Sync + 'static {
    async fn create_group(&self, group: ContactGroup) -> ContactRepositoryResult<ContactGroup>;
    async fn update_group(&self, group: ContactGroup) -> ContactRepositoryResult<ContactGroup>;
    async fn delete_group(&self, id: &Uuid) -> ContactRepositoryResult<()>;
    async fn get_group_by_id(&self, id: &Uuid) -> ContactRepositoryResult<Option<ContactGroup>>;
    async fn get_groups_by_address_book(&self, address_book_id: &Uuid) -> ContactRepositoryResult<Vec<ContactGroup>>;
    async fn add_contact_to_group(&self, group_id: &Uuid, contact_id: &Uuid) -> ContactRepositoryResult<()>;
    async fn remove_contact_from_group(&self, group_id: &Uuid, contact_id: &Uuid) -> ContactRepositoryResult<()>;
    async fn get_contacts_in_group(&self, group_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>>;
    async fn get_groups_for_contact(&self, contact_id: &Uuid) -> ContactRepositoryResult<Vec<ContactGroup>>;
}