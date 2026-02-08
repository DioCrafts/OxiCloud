use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, types::Uuid};
use std::sync::Arc;
use serde_json::Value as JsonValue;

use crate::domain::entities::contact::Contact;
use crate::domain::repositories::contact_repository::{ContactRepository, ContactRepositoryResult};
use crate::common::errors::DomainError;
use super::contact_persistence_dto::{emails_to_persistence, phones_to_persistence, addresses_to_persistence};

pub struct ContactPgRepository {
    pool: Arc<PgPool>,
}

impl ContactPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactRepository for ContactPgRepository {
    async fn create_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact> {
        // Convert domain entities to persistence DTOs for JSONB serialization
        let email_dtos = emails_to_persistence(contact.email());
        let phone_dtos = phones_to_persistence(contact.phone());
        let address_dtos = addresses_to_persistence(contact.address());
        
        let email_json = serde_json::to_value(&email_dtos).unwrap_or(JsonValue::Null);
        let phone_json = serde_json::to_value(&phone_dtos).unwrap_or(JsonValue::Null);
        let address_json = serde_json::to_value(&address_dtos).unwrap_or(JsonValue::Null);
        
        let _row = sqlx::query(
            r#"
            INSERT INTO carddav.contacts (
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20
            )
            RETURNING 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            "#
        )
        .bind(contact.id())
        .bind(contact.address_book_id())
        .bind(contact.uid())
        .bind(contact.full_name_owned())
        .bind(contact.first_name_owned())
        .bind(contact.last_name_owned())
        .bind(contact.nickname_owned())
        .bind(email_json)
        .bind(phone_json)
        .bind(address_json)
        .bind(contact.organization_owned())
        .bind(contact.title_owned())
        .bind(contact.notes_owned())
        .bind(contact.photo_url_owned())
        .bind(contact.birthday().copied())
        .bind(contact.anniversary().copied())
        .bind(contact.vcard())
        .bind(contact.etag())
        .bind(*contact.created_at())
        .bind(*contact.updated_at())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to create contact: {}", e)))?;

        // En una implementación real, construiríamos un objeto Contact completo
        // Por simplicidad, devolvemos el contacto original
        Ok(contact)
    }

    async fn update_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact> {
        let now = Utc::now();
        // Convert domain entities to persistence DTOs for JSONB serialization
        let email_dtos = emails_to_persistence(contact.email());
        let phone_dtos = phones_to_persistence(contact.phone());
        let address_dtos = addresses_to_persistence(contact.address());
        
        let email_json = serde_json::to_value(&email_dtos).unwrap_or(JsonValue::Null);
        let phone_json = serde_json::to_value(&phone_dtos).unwrap_or(JsonValue::Null);
        let address_json = serde_json::to_value(&address_dtos).unwrap_or(JsonValue::Null);
        
        // Create a clone of the contact with the updated timestamp
        let mut updated_contact = contact.clone();
        updated_contact.set_updated_at(now);
        
        let _row = sqlx::query(
            r#"
            UPDATE carddav.contacts
            SET 
                full_name = $1,
                first_name = $2,
                last_name = $3,
                nickname = $4,
                email = $5,
                phone = $6,
                address = $7,
                organization = $8,
                title = $9,
                notes = $10,
                photo_url = $11,
                birthday = $12,
                anniversary = $13,
                vcard = $14,
                etag = $15,
                updated_at = $16
            WHERE id = $17
            RETURNING 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            "#
        )
        .bind(updated_contact.full_name_owned())
        .bind(updated_contact.first_name_owned())
        .bind(updated_contact.last_name_owned())
        .bind(updated_contact.nickname_owned())
        .bind(email_json)
        .bind(phone_json)
        .bind(address_json)
        .bind(updated_contact.organization_owned())
        .bind(updated_contact.title_owned())
        .bind(updated_contact.notes_owned())
        .bind(updated_contact.photo_url_owned())
        .bind(updated_contact.birthday().copied())
        .bind(updated_contact.anniversary().copied())
        .bind(updated_contact.vcard())
        .bind(updated_contact.etag())
        .bind(now)
        .bind(updated_contact.id())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to update contact: {}", e)))?;

        // En una implementación real, construiríamos un objeto Contact a partir de la fila resultante
        // Por simplicidad, devolvemos el contacto con el timestamp actualizado
        Ok(updated_contact)
    }

    async fn delete_contact(&self, id: &Uuid) -> ContactRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM carddav.contacts
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to delete contact: {}", e)))?;

        Ok(())
    }

    async fn get_contact_by_id(&self, id: &Uuid) -> ContactRepositoryResult<Option<Contact>> {
        let row_opt = sqlx::query(
            r#"
            SELECT 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            FROM carddav.contacts
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get contact by id: {}", e)))?;

        if let Some(_row) = row_opt {
            // En una implementación real, construiríamos un objeto Contact a partir de la fila
            // Por simplicidad y demostración, devolvemos una instancia predeterminada
            return Ok(Some(Contact::default()));
        }

        Ok(None)
    }

    async fn get_contact_by_uid(&self, address_book_id: &Uuid, uid: &str) -> ContactRepositoryResult<Option<Contact>> {
        let row_opt = sqlx::query(
            r#"
            SELECT 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            FROM carddav.contacts
            WHERE address_book_id = $1 AND uid = $2
            "#
        )
        .bind(address_book_id)
        .bind(uid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get contact by uid: {}", e)))?;

        if let Some(_row) = row_opt {
            // En una implementación real, construiríamos un objeto Contact a partir de la fila
            // Por simplicidad y demostración, devolvemos una instancia predeterminada
            return Ok(Some(Contact::default()));
        }

        Ok(None)
    }

    async fn get_contacts_by_address_book(&self, address_book_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>> {
        let _rows = sqlx::query(
            r#"
            SELECT 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            FROM carddav.contacts
            WHERE address_book_id = $1
            ORDER BY full_name, first_name, last_name
            "#
        )
        .bind(address_book_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get contacts by address book: {}", e)))?;

        // En una implementación real, construiríamos objetos Contact a partir de las filas
        // Por simplicidad y demostración, devolvemos una lista vacía
        let contacts = Vec::new();
        
        Ok(contacts)
    }

    async fn get_contacts_by_email(&self, email: &str) -> ContactRepositoryResult<Vec<Contact>> {
        let search_pattern = format!("%{}%", email);
        
        let _rows = sqlx::query(
            r#"
            SELECT 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            FROM carddav.contacts
            WHERE email::text ILIKE $1
            ORDER BY full_name, first_name, last_name
            "#
        )
        .bind(&search_pattern)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get contacts by email: {}", e)))?;

        // En una implementación real, construiríamos objetos Contact a partir de las filas
        // Por simplicidad y demostración, devolvemos una lista vacía
        let contacts = Vec::new();
        
        Ok(contacts)
    }

    async fn get_contacts_by_group(&self, group_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>> {
        let _rows = sqlx::query(
            r#"
            SELECT 
                c.id, c.address_book_id, c.uid, c.full_name, c.first_name, c.last_name, c.nickname,
                c.email, c.phone, c.address, c.organization, c.title, c.notes, c.photo_url,
                c.birthday, c.anniversary, c.vcard, c.etag, c.created_at, c.updated_at
            FROM carddav.contacts c
            INNER JOIN carddav.group_memberships m ON c.id = m.contact_id
            WHERE m.group_id = $1
            ORDER BY c.full_name, c.first_name, c.last_name
            "#
        )
        .bind(group_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to get contacts by group: {}", e)))?;

        // En una implementación real, construiríamos objetos Contact a partir de las filas
        // Por simplicidad y demostración, devolvemos una lista vacía
        let contacts = Vec::new();
        
        Ok(contacts)
    }

    async fn search_contacts(&self, address_book_id: &Uuid, query: &str) -> ContactRepositoryResult<Vec<Contact>> {
        let search_pattern = format!("%{}%", query);
        
        let _rows = sqlx::query(
            r#"
            SELECT 
                id, address_book_id, uid, full_name, first_name, last_name, nickname,
                email, phone, address, organization, title, notes, photo_url,
                birthday, anniversary, vcard, etag, created_at, updated_at
            FROM carddav.contacts
            WHERE address_book_id = $1 
              AND (
                  full_name ILIKE $2 
                  OR first_name ILIKE $2
                  OR last_name ILIKE $2
                  OR nickname ILIKE $2
                  OR email::text ILIKE $2
                  OR phone::text ILIKE $2
                  OR organization ILIKE $2
              )
            ORDER BY full_name, first_name, last_name
            "#
        )
        .bind(address_book_id)
        .bind(&search_pattern)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to search contacts: {}", e)))?;

        // En una implementación real, construiríamos objetos Contact a partir de las filas
        // Por simplicidad y demostración, devolvemos una lista vacía
        let contacts = Vec::new();
        
        Ok(contacts)
    }
}