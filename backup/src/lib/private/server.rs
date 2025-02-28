use crate::app_framework::http::request::Request;
use crate::app_framework::utility::simple_container::SimpleContainer;
use crate::cache::user_cache::UserCache;
use crate::db::connection_wrapper::ConnectionWrapper;
use crate::files::node::root::Root;
use crate::files::view::View;
use crate::contacts_manager::ContactsManager;
use crate::preview_manager::PreviewManager;
use crate::tag_manager::TagManager;
use crate::user::manager::Manager as UserManager;
use crate::user::session::Session as UserSession;
use crate::navigation_manager::NavigationManager;
use crate::all_config::AllConfig;
use crate::l10n::factory::Factory as L10NFactory;
use crate::url_generator::URLGenerator;
use crate::app_helper::AppHelper;
use crate::activity_manager::ActivityManager;
use crate::files::filesystem::Filesystem;
use crate::files::folder::Folder;
use crate::oc;
use crate::oc_hook;
use crate::oc_app;
use crate::oc_db;
use crate::oc_user;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

/// Server container implementing all server interfaces
///
/// TODO: hookup all manager classes
pub struct Server {
    container: SimpleContainer,
}

#[async_trait]
impl IServerContainer for Server {
    async fn get_contacts_manager(&self) -> Arc<dyn IContactsManager> {
        self.container.query::<dyn IContactsManager>("ContactsManager").await
    }

    async fn get_request(&self) -> Option<Arc<dyn IRequest>> {
        self.container.query::<dyn IRequest>("Request").await
    }

    async fn get_preview_manager(&self) -> Arc<dyn IPreview> {
        self.container.query::<dyn IPreview>("PreviewManager").await
    }

    async fn get_tag_manager(&self) -> Arc<dyn ITagManager> {
        self.container.query::<dyn ITagManager>("TagManager").await
    }

    async fn get_root_folder(&self) -> Arc<dyn Folder> {
        self.container.query::<dyn Folder>("RootFolder").await
    }

    async fn get_user_folder(&self) -> Arc<dyn Folder> {
        let dir = "/files";
        let root = self.get_root_folder().await;
        
        if !root.node_exists(dir).await {
            root.new_folder(dir).await
        } else {
            root.get(dir).await
        }
    }

    async fn get_app_folder(&self) -> Arc<dyn Folder> {
        let dir = format!("/{}", oc_app::get_current_app().await);
        let root = self.get_root_folder().await;
        
        if !root.node_exists(&dir).await {
            root.new_folder(&dir).await
        } else {
            root.get(&dir).await
        }
    }

    async fn get_user_manager(&self) -> Arc<UserManager> {
        self.container.query::<UserManager>("UserManager").await
    }

    async fn get_user_session(&self) -> Arc<UserSession> {
        self.container.query::<UserSession>("UserSession").await
    }

    async fn get_navigation_manager(&self) -> Arc<NavigationManager> {
        self.container.query::<NavigationManager>("NavigationManager").await
    }

    async fn get_config(&self) -> Arc<dyn IConfig> {
        self.container.query::<dyn IConfig>("AllConfig").await
    }

    async fn get_l10n(&self, app: &str) -> Arc<dyn L10N> {
        let factory = self.container.query::<L10NFactory>("L10NFactory").await;
        factory.get(app).await
    }

    async fn get_url_generator(&self) -> Arc<URLGenerator> {
        self.container.query::<URLGenerator>("URLGenerator").await
    }

    async fn get_helper(&self) -> Arc<dyn IHelper> {
        self.container.query::<dyn IHelper>("AppHelper").await
    }

    async fn get_cache(&self) -> Arc<dyn ICache> {
        self.container.query::<dyn ICache>("UserCache").await
    }

    async fn get_session(&self) -> Arc<dyn ISession> {
        oc::get_session().await
    }

    async fn get_database_connection(&self) -> Arc<dyn IDBConnection> {
        let conn = oc_db::get_connection().await;
        Arc::new(ConnectionWrapper::new(conn))
    }

    async fn get_activity_manager(&self) -> Arc<dyn IActivityManager> {
        self.container.query::<dyn IActivityManager>("ActivityManager").await
    }
}

impl Server {
    pub async fn new() -> Self {
        let mut server = Self {
            container: SimpleContainer::new(),
        };
        
        server.register_services().await;
        server
    }
    
    async fn register_services(&mut self) {
        self.container.register_service("ContactsManager", |_c| {
            Box::pin(async { Arc::new(ContactsManager::new()) as Arc<dyn IContactsManager> })
        });
        
        self.container.register_service("Request", |c| {
            Box::pin(async move {
                let url_params = match c.get::<HashMap<String, String>>("urlParams").await {
                    Some(params) => params,
                    None => HashMap::new(),
                };
                
                let session = oc::get_session().await;
                let request_token = if session.exists("requesttoken").await {
                    session.get("requesttoken").await
                } else {
                    None
                };
                
                // Note: In a real implementation, we'd need to access these globals differently
                // This is a simplified version to match the PHP code
                let request_params = web::RequestParams {
                    get: web::get_query_params().await,
                    post: web::get_post_params().await,
                    files: web::get_files().await,
                    server: web::get_server_vars().await,
                    env: web::get_env_vars().await,
                    cookies: web::get_cookies().await,
                    method: web::get_request_method().await,
                    url_params,
                    request_token,
                };
                
                Arc::new(Request::new(request_params)) as Arc<dyn IRequest>
            })
        });
        
        self.container.register_service("PreviewManager", |_c| {
            Box::pin(async { Arc::new(PreviewManager::new()) as Arc<dyn IPreview> })
        });
        
        self.container.register_service("TagManager", |_c| {
            Box::pin(async {
                let user = oc_user::get_user().await;
                Arc::new(TagManager::new(&user)) as Arc<dyn ITagManager>
            })
        });
        
        self.container.register_service("RootFolder", |c| {
            Box::pin(async move {
                let user_id = oc_user::get_user().await;
                let user_manager = c.query::<UserManager>("UserManager").await;
                let user = user_manager.get(&user_id).await;
                let manager = Filesystem::get_mount_manager().await;
                let view = View::new();
                Arc::new(Root::new(manager, view, user)) as Arc<dyn Folder>
            })
        });
        
        self.container.register_service("UserManager", |_c| {
            Box::pin(async { Arc::new(UserManager::new()) })
        });
        
        self.container.register_service("UserSession", |c| {
            Box::pin(async move {
                let manager = c.query::<UserManager>("UserManager").await;
                let session = oc::get_session().await;
                let user_session = Arc::new(UserSession::new(manager, session));
                
                // Set up event listeners
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "preCreateUser", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(uid), Some(password)) = (args.get("uid"), args.get("password")) {
                                oc_hook::emit("OC_User", "pre_createUser", HashMap::from([
                                    ("run".to_string(), "true".to_string()),
                                    ("uid".to_string(), uid.clone()),
                                    ("password".to_string(), password.clone()),
                                ])).await;
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "postCreateUser", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(user), Some(password)) = (args.get("user"), args.get("password")) {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "post_createUser", HashMap::from([
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                        ("password".to_string(), password.clone()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "preDelete", Box::new(move |args| {
                        Box::pin(async move {
                            if let Some(user) = args.get("user") {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "pre_deleteUser", HashMap::from([
                                        ("run".to_string(), "true".to_string()),
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "postDelete", Box::new(move |args| {
                        Box::pin(async move {
                            if let Some(user) = args.get("user") {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "post_deleteUser", HashMap::from([
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "preSetPassword", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(user), Some(password), Some(recovery_password)) = 
                                (args.get("user"), args.get("password"), args.get("recoveryPassword")) {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "pre_setPassword", HashMap::from([
                                        ("run".to_string(), "true".to_string()),
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                        ("password".to_string(), password.clone()),
                                        ("recoveryPassword".to_string(), recovery_password.clone()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "postSetPassword", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(user), Some(password), Some(recovery_password)) = 
                                (args.get("user"), args.get("password"), args.get("recoveryPassword")) {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "post_setPassword", HashMap::from([
                                        ("run".to_string(), "true".to_string()),
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                        ("password".to_string(), password.clone()),
                                        ("recoveryPassword".to_string(), recovery_password.clone()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "preLogin", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(uid), Some(password)) = (args.get("uid"), args.get("password")) {
                                oc_hook::emit("OC_User", "pre_login", HashMap::from([
                                    ("run".to_string(), "true".to_string()),
                                    ("uid".to_string(), uid.clone()),
                                    ("password".to_string(), password.clone()),
                                ])).await;
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "postLogin", Box::new(move |args| {
                        Box::pin(async move {
                            if let (Some(user), Some(password)) = (args.get("user"), args.get("password")) {
                                if let Some(user_obj) = user.downcast_ref::<User>() {
                                    oc_hook::emit("OC_User", "post_login", HashMap::from([
                                        ("run".to_string(), "true".to_string()),
                                        ("uid".to_string(), user_obj.get_uid().to_string()),
                                        ("password".to_string(), password.clone()),
                                    ])).await;
                                }
                            }
                        })
                    })).await;
                }
                
                {
                    let user_session_clone = Arc::clone(&user_session);
                    user_session_clone.listen(r"\OC\User", "logout", Box::new(move |_args| {
                        Box::pin(async move {
                            oc_hook::emit("OC_User", "logout", HashMap::new()).await;
                        })
                    })).await;
                }
                
                user_session
            })
        });
        
        self.container.register_service("NavigationManager", |_c| {
            Box::pin(async { Arc::new(NavigationManager::new()) })
        });
        
        self.container.register_service("AllConfig", |_c| {
            Box::pin(async { Arc::new(AllConfig::new()) as Arc<dyn IConfig> })
        });
        
        self.container.register_service("L10NFactory", |_c| {
            Box::pin(async { Arc::new(L10NFactory::new()) })
        });
        
        self.container.register_service("URLGenerator", |_c| {
            Box::pin(async { Arc::new(URLGenerator::new()) })
        });
        
        self.container.register_service("AppHelper", |_c| {
            Box::pin(async { Arc::new(AppHelper::new()) as Arc<dyn IHelper> })
        });
        
        self.container.register_service("UserCache", |_c| {
            Box::pin(async { Arc::new(UserCache::new()) as Arc<dyn ICache> })
        });
        
        self.container.register_service("ActivityManager", |_c| {
            Box::pin(async { Arc::new(ActivityManager::new()) as Arc<dyn IActivityManager> })
        });
    }
}

// The required interfaces
#[async_trait]
pub trait IServerContainer: Send + Sync {
    async fn get_contacts_manager(&self) -> Arc<dyn IContactsManager>;
    async fn get_request(&self) -> Option<Arc<dyn IRequest>>;
    async fn get_preview_manager(&self) -> Arc<dyn IPreview>;
    async fn get_tag_manager(&self) -> Arc<dyn ITagManager>;
    async fn get_root_folder(&self) -> Arc<dyn Folder>;
    async fn get_user_folder(&self) -> Arc<dyn Folder>;
    async fn get_app_folder(&self) -> Arc<dyn Folder>;
    async fn get_user_manager(&self) -> Arc<UserManager>;
    async fn get_user_session(&self) -> Arc<UserSession>;
    async fn get_navigation_manager(&self) -> Arc<NavigationManager>;
    async fn get_config(&self) -> Arc<dyn IConfig>;
    async fn get_l10n(&self, app: &str) -> Arc<dyn L10N>;
    async fn get_url_generator(&self) -> Arc<URLGenerator>;
    async fn get_helper(&self) -> Arc<dyn IHelper>;
    async fn get_cache(&self) -> Arc<dyn ICache>;
    async fn get_session(&self) -> Arc<dyn ISession>;
    async fn get_database_connection(&self) -> Arc<dyn IDBConnection>;
    async fn get_activity_manager(&self) -> Arc<dyn IActivityManager>;
}

// These would be defined elsewhere
pub trait IContactsManager: Send + Sync {}
pub trait IRequest: Send + Sync {}
pub trait IPreview: Send + Sync {}
pub trait ITagManager: Send + Sync {}
pub trait IConfig: Send + Sync {}
pub trait L10N: Send + Sync {}
pub trait IHelper: Send + Sync {}
pub trait ICache: Send + Sync {}
pub trait ISession: Send + Sync {}
pub trait IDBConnection: Send + Sync {}
pub trait IActivityManager: Send + Sync {}

// For the web interface
mod web {
    use std::collections::HashMap;
    
    pub struct RequestParams {
        pub get: HashMap<String, String>,
        pub post: HashMap<String, String>,
        pub files: HashMap<String, String>,
        pub server: HashMap<String, String>,
        pub env: HashMap<String, String>,
        pub cookies: HashMap<String, String>,
        pub method: Option<String>,
        pub url_params: HashMap<String, String>,
        pub request_token: Option<String>,
    }
    
    pub async fn get_query_params() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_post_params() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_files() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_server_vars() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_env_vars() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_cookies() -> HashMap<String, String> {
        // Implementation would connect to the web framework
        HashMap::new()
    }
    
    pub async fn get_request_method() -> Option<String> {
        // Implementation would connect to the web framework
        None
    }
}