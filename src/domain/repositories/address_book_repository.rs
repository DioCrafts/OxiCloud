use async_trait::async_trait;
use std::result::Result;
use uuid::Uuid;

use crate::common::errors::DomainError;
use crate::domain::entities::contact::AddressBook;

pub type AddressBookRepositoryResult<T> = Result<T, DomainError>;

#[async_trait]
pub trait AddressBookRepository: Send + Sync + 'static {
    async fn create_address_book(
        &self,
        address_book: AddressBook,
    ) -> AddressBookRepositoryResult<AddressBook>;
    async fn update_address_book(
        &self,
        address_book: AddressBook,
    ) -> AddressBookRepositoryResult<AddressBook>;
    async fn delete_address_book(&self, id: &Uuid) -> AddressBookRepositoryResult<()>;
    async fn get_address_book_by_id(
        &self,
        id: &Uuid,
    ) -> AddressBookRepositoryResult<Option<AddressBook>>;
    async fn get_address_books_by_owner(
        &self,
        owner_id: &str,
    ) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn get_shared_address_books(
        &self,
        user_id: &str,
    ) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn get_public_address_books(&self) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn share_address_book(
        &self,
        address_book_id: &Uuid,
        user_id: &str,
        can_write: bool,
    ) -> AddressBookRepositoryResult<()>;
    async fn unshare_address_book(
        &self,
        address_book_id: &Uuid,
        user_id: &str,
    ) -> AddressBookRepositoryResult<()>;
    async fn get_address_book_shares(
        &self,
        address_book_id: &Uuid,
    ) -> AddressBookRepositoryResult<Vec<(String, bool)>>;
}
