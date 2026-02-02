use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub revoked: bool,
}

impl Session {
    pub fn new(
        user_id: String,
        refresh_token: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        expires_in_days: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            refresh_token,
            expires_at: now + Duration::days(expires_in_days),
            ip_address,
            user_agent,
            created_at: now,
            revoked: false,
        }
    }
    
    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
    
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
    
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    pub fn is_revoked(&self) -> bool {
        self.revoked
    }
    
    pub fn revoke(&mut self) {
        self.revoked = true;
    }
}