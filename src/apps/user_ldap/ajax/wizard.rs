// Copyright 2013 Arthur Schiwon blizzz@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

mod user_ldap {
    pub mod lib {
        use std::collections::HashMap;
        
        pub struct LDAP;
        
        pub struct Configuration {
            prefix: String,
        }
        
        impl Configuration {
            pub fn new(prefix: String) -> Self {
                Self { prefix }
            }
            
            pub fn set_configuration(&self, cfg: HashMap<String, String>, set_parameters: &mut Vec<String>) -> Result<(), String> {
                // Implementation details
                for (key, _) in cfg.iter() {
                    set_parameters.push(key.clone());
                }
                Ok(())
            }
            
            pub fn save_configuration(&self) -> Result<(), String> {
                // Implementation details
                Ok(())
            }
        }
        
        pub struct Wizard {
            configuration: Configuration,
            ldap_wrapper: LDAP,
        }
        
        pub struct WizardResult {
            result: HashMap<String, serde_json::Value>,
        }
        
        impl WizardResult {
            pub fn get_result_array(&self) -> HashMap<String, serde_json::Value> {
                self.result.clone()
            }
        }
        
        impl Wizard {
            pub fn new(configuration: Configuration, ldap_wrapper: LDAP) -> Self {
                Self {
                    configuration,
                    ldap_wrapper,
                }
            }
            
            pub fn guess_port_and_tls(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn guess_base_dn(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_group_member_assoc(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_user_object_classes(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_group_object_classes(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_groups_for_users(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_groups_for_groups(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn determine_attributes(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn get_user_list_filter(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn get_user_login_filter(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn get_group_filter(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn count_users(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
            
            pub fn count_groups(&self) -> Result<WizardResult, String> {
                // Implementation details
                Ok(WizardResult {
                    result: HashMap::new(),
                })
            }
        }
    }
}

use user_ldap::lib::{Configuration, LDAP, Wizard};

#[derive(Debug, Error)]
enum WizardError {
    #[error("No action specified")]
    NoActionSpecified,
    #[error("No configuration specified")]
    NoConfigurationSpecified,
    #[error("No data specified")]
    NoDataSpecified,
    #[error("{0} Could not set configuration {1}")]
    ConfigurationSetFailed(String, String),
    #[error("Wizard action failed: {0}")]
    WizardActionFailed(String),
    #[error("Invalid action")]
    InvalidAction,
}

#[derive(Deserialize)]
struct WizardRequest {
    action: Option<String>,
    ldap_serverconfig_chooser: Option<String>,
    cfgkey: Option<String>,
    cfgval: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct SuccessResponse<T> {
    #[serde(flatten)]
    data: T,
}

async fn check_admin_user() -> Result<(), WizardError> {
    // Implementation would check admin privileges
    Ok(())
}

async fn check_app_enabled(app: &str) -> Result<(), String> {
    // Implementation would check if app is enabled
    if app == "user_ldap" {
        Ok(())
    } else {
        Err("App not enabled".to_string())
    }
}

async fn wizard_handler(form: web::Form<WizardRequest>) -> impl Responder {
    // Check user and app status
    if let Err(e) = check_admin_user().await {
        return HttpResponse::Forbidden().json(ErrorResponse {
            message: e.to_string(),
        });
    }
    
    if let Err(e) = check_app_enabled("user_ldap").await {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: e,
        });
    }
    
    // Get localization service (simplified)
    let l10n = |key: &str| -> String {
        key.to_string()
    };
    
    let action = match &form.action {
        Some(a) => a,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: l10n("No action specified"),
            });
        }
    };
    
    let prefix = match &form.ldap_serverconfig_chooser {
        Some(p) => p,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: l10n("No configuration specified"),
            });
        }
    };
    
    let ldap_wrapper = LDAP;
    let configuration = Configuration::new(prefix.clone());
    let wizard = Wizard::new(configuration, ldap_wrapper);
    
    match action.as_str() {
        "guessPortAndTLS" => match wizard.guess_port_and_tls() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "guessBaseDN" => match wizard.guess_base_dn() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineGroupMemberAssoc" => match wizard.determine_group_member_assoc() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineUserObjectClasses" => match wizard.determine_user_object_classes() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineGroupObjectClasses" => match wizard.determine_group_object_classes() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineGroupsForUsers" => match wizard.determine_groups_for_users() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineGroupsForGroups" => match wizard.determine_groups_for_groups() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "determineAttributes" => match wizard.determine_attributes() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "getUserListFilter" => match wizard.get_user_list_filter() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "getUserLoginFilter" => match wizard.get_user_login_filter() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "getGroupFilter" => match wizard.get_group_filter() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "countUsers" => match wizard.count_users() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "countGroups" => match wizard.count_groups() {
            Ok(result) => HttpResponse::Ok().json(SuccessResponse { data: result.get_result_array() }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { message: e }),
        },
        "save" => {
            let key = match &form.cfgkey {
                Some(k) => k,
                None => {
                    return HttpResponse::BadRequest().json(ErrorResponse {
                        message: l10n("No data specified"),
                    });
                }
            };
            
            let val = match &form.cfgval {
                Some(v) => v,
                None => {
                    return HttpResponse::BadRequest().json(ErrorResponse {
                        message: l10n("No data specified"),
                    });
                }
            };
            
            let cfg = HashMap::from([(key.clone(), val.clone())]);
            let mut set_parameters = Vec::new();
            
            if let Err(e) = configuration.set_configuration(cfg, &mut set_parameters) {
                return HttpResponse::InternalServerError().json(ErrorResponse { message: e });
            }
            
            if !set_parameters.contains(key) {
                let message = format!("{} Could not set configuration {}", key, set_parameters.get(0).unwrap_or(&String::new()));
                return HttpResponse::InternalServerError().json(ErrorResponse { message });
            }
            
            if let Err(e) = configuration.save_configuration() {
                return HttpResponse::InternalServerError().json(ErrorResponse { message: e });
            }
            
            HttpResponse::Ok().json(SuccessResponse { data: {} })
        },
        _ => HttpResponse::BadRequest().status(400).finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/apps/user_ldap/ajax/wizard.php", web::post().to(wizard_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}