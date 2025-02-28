use std::error::Error;

/// Installs necessary configuration for the LDAP app
pub async fn install() -> Result<(), Box<dyn Error>> {
    let config = crate::config::SystemConfig::new();
    
    let state = config.get_system_value("ldapIgnoreNamingRules", "doSet".to_string())?;
    
    if state == "doSet" {
        config.set_system_value("ldapIgnoreNamingRules", false)?;
    }
    
    Ok(())
}