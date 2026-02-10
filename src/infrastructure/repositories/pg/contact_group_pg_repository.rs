use async_trait::async_trait;
use sqlx::{PgPool, Row, types::Uuid};
use std::sync::Arc;
use chrono::Utc;
use serde_json::Value as JsonValue;

use crate::domain::entities::contact::{Contact, ContactGroup};
use crate::domain::repositories::contact_repository::{ContactGroupRepository, ContactRepositoryResult};
use crate::common::errors::{DomainError, ErrorKind};
use super::contact_persistence_dto::{
    emails_from_persistence, phones_from_persistence, addresses_from_persistence,
    EmailPersistenceDto, PhonePersistenceDto, AddressPersistenceDto,
};

pub struct ContactGroupPgRepository {
    pool: Arc<PgPool>,
}

impl ContactGroupPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactGroupRepository for ContactGroupPgRepository {
    async fn create_group(&self, group: ContactGroup) -> ContactRepositoryResult<ContactGroup> {
        sqlx::query(
            "INSERT INTO carddav.contact_groups (id, address_book_id, name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(group.id())
        .bind(group.address_book_id())
        .bind(group.name())
        .bind(group.created_at())
        .bind(group.updated_at())
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to create group: {}", e)))?;
        
        Ok(group)
    }

    async fn update_group(&self, group: ContactGroup) -> ContactRepositoryResult<ContactGroup> {
        sqlx::query(
            "UPDATE carddav.contact_groups SET name = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(group.name())
        .bind(Utc::now())
        .bind(group.id())
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to update group: {}", e)))?;
        
        Ok(group)
    }

    async fn delete_group(&self, id: &Uuid) -> ContactRepositoryResult<()> {
        // Delete memberships first
        sqlx::query("DELETE FROM carddav.group_memberships WHERE group_id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to delete group memberships: {}", e)))?;
        
        sqlx::query("DELETE FROM carddav.contact_groups WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to delete group: {}", e)))?;
        
        Ok(())
    }

    async fn get_group_by_id(&self, id: &Uuid) -> ContactRepositoryResult<Option<ContactGroup>> {
        let row = sqlx::query(
            "SELECT id, address_book_id, name, created_at, updated_at FROM carddav.contact_groups WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to get group: {}", e)))?;
        
        match row {
            Some(row) => {
                let group = ContactGroup::from_raw(
                    row.get::<Uuid, _>("id"),
                    row.get::<Uuid, _>("address_book_id"),
                    row.get::<String, _>("name"),
                    row.get("created_at"),
                    row.get("updated_at"),
                );
                Ok(Some(group))
            },
            None => Ok(None),
        }
    }

    async fn get_groups_by_address_book(&self, address_book_id: &Uuid) -> ContactRepositoryResult<Vec<ContactGroup>> {
        let rows = sqlx::query(
            "SELECT id, address_book_id, name, created_at, updated_at FROM carddav.contact_groups WHERE address_book_id = $1 ORDER BY name"
        )
        .bind(address_book_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to list groups: {}", e)))?;
        
        Ok(rows.into_iter().map(|row| {
            ContactGroup::from_raw(
                row.get::<Uuid, _>("id"),
                row.get::<Uuid, _>("address_book_id"),
                row.get::<String, _>("name"),
                row.get("created_at"),
                row.get("updated_at"),
            )
        }).collect())
    }

    async fn add_contact_to_group(&self, group_id: &Uuid, contact_id: &Uuid) -> ContactRepositoryResult<()> {
        sqlx::query(
            "INSERT INTO carddav.group_memberships (group_id, contact_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(group_id)
        .bind(contact_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to add contact to group: {}", e)))?;
        
        Ok(())
    }

    async fn remove_contact_from_group(&self, group_id: &Uuid, contact_id: &Uuid) -> ContactRepositoryResult<()> {
        sqlx::query(
            "DELETE FROM carddav.group_memberships WHERE group_id = $1 AND contact_id = $2"
        )
        .bind(group_id)
        .bind(contact_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to remove contact from group: {}", e)))?;
        
        Ok(())
    }

    async fn get_contacts_in_group(&self, group_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id, c.address_book_id, c.uid, c.full_name, c.first_name, c.last_name, c.nickname,
                c.email, c.phone, c.address, c.organization, c.title, c.notes, c.photo_url,
                c.birthday, c.anniversary, c.vcard, c.etag, c.created_at, c.updated_at
            FROM carddav.contacts c
            INNER JOIN carddav.group_memberships gm ON c.id = gm.contact_id
            WHERE gm.group_id = $1
            ORDER BY c.full_name, c.first_name, c.last_name
            "#
        )
        .bind(group_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to get contacts in group: {}", e)))?;

        let mut contacts = Vec::new();
        for row in &rows {
            let email_json: JsonValue = row.get("email");
            let phone_json: JsonValue = row.get("phone");
            let address_json: JsonValue = row.get("address");

            let emails = serde_json::from_value::<Vec<EmailPersistenceDto>>(email_json)
                .map(emails_from_persistence)
                .unwrap_or_default();
            let phones = serde_json::from_value::<Vec<PhonePersistenceDto>>(phone_json)
                .map(phones_from_persistence)
                .unwrap_or_default();
            let addresses = serde_json::from_value::<Vec<AddressPersistenceDto>>(address_json)
                .map(addresses_from_persistence)
                .unwrap_or_default();

            contacts.push(Contact::from_raw(
                row.get("id"),
                row.get("address_book_id"),
                row.get("uid"),
                row.get::<Option<String>, _>("full_name"),
                row.get::<Option<String>, _>("first_name"),
                row.get::<Option<String>, _>("last_name"),
                row.get::<Option<String>, _>("nickname"),
                emails,
                phones,
                addresses,
                row.get::<Option<String>, _>("organization"),
                row.get::<Option<String>, _>("title"),
                row.get::<Option<String>, _>("notes"),
                row.get::<Option<String>, _>("photo_url"),
                row.get("birthday"),
                row.get("anniversary"),
                row.get("vcard"),
                row.get("etag"),
                row.get("created_at"),
                row.get("updated_at"),
            ));
        }
        Ok(contacts)
    }

    async fn get_groups_for_contact(&self, contact_id: &Uuid) -> ContactRepositoryResult<Vec<ContactGroup>> {
        let rows = sqlx::query(
            "SELECT g.id, g.address_book_id, g.name, g.created_at, g.updated_at FROM carddav.contact_groups g INNER JOIN carddav.group_memberships gm ON g.id = gm.group_id WHERE gm.contact_id = $1 ORDER BY g.name"
        )
        .bind(contact_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ContactGroup", format!("Failed to get groups for contact: {}", e)))?;
        
        Ok(rows.into_iter().map(|row| {
            ContactGroup::from_raw(
                row.get::<Uuid, _>("id"),
                row.get::<Uuid, _>("address_book_id"),
                row.get::<String, _>("name"),
                row.get("created_at"),
                row.get("updated_at"),
            )
        }).collect())
    }
}
