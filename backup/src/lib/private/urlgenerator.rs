use std::collections::HashMap;
use std::path::Path;
use std::fs;
use url::form_urlencoded;

/// Trait to generate URLs
pub trait IUrlGenerator {
    fn link_to_route(&self, route: &str, parameters: HashMap<String, String>) -> String;
    fn link_to(&self, app: &str, file: &str, args: HashMap<String, String>) -> String;
    fn image_path(&self, app: &str, image: &str) -> Result<String, String>;
    fn get_absolute_url(&self, url: &str) -> String;
}

/// Class to generate URLs
pub struct UrlGenerator {
    webroot: String,
    serverroot: String,
}

impl UrlGenerator {
    pub fn new(webroot: String, serverroot: String) -> Self {
        UrlGenerator {
            webroot,
            serverroot,
        }
    }
}

impl IUrlGenerator for UrlGenerator {
    /// Creates an url using a defined route
    ///
    /// # Arguments
    /// * `route` - The route to generate
    /// * `parameters` - Parameters to pass to the route
    ///
    /// # Returns
    /// The generated URL
    fn link_to_route(&self, route: &str, parameters: HashMap<String, String>) -> String {
        // In Rust implementation, we would call the router's generate method
        // For this translation, assuming a router object would be available
        // through dependency injection or other means
        let router = get_router(); // This would be implemented elsewhere
        router.generate(route, parameters)
    }

    /// Creates an url
    ///
    /// # Arguments
    /// * `app` - Application name
    /// * `file` - File path
    /// * `args` - Query parameters
    ///
    /// # Returns
    /// The URL to the given app and file
    fn link_to(&self, app: &str, file: &str, args: HashMap<String, String>) -> String {
        let mut url_link_to = String::new();

        if !app.is_empty() {
            let app_path = get_app_path(app); // This would be implemented elsewhere
            
            // Check if the app is in the app folder
            if let Some(app_path) = app_path {
                let full_path = format!("{}/{}", app_path, file);
                if Path::new(&full_path).exists() {
                    if file.ends_with(".php") || file.ends_with(".css") {
                        url_link_to = format!("{}/index.php/apps/{}", self.webroot, app);
                        if file != "index.php" {
                            url_link_to = format!("{}/{}", url_link_to, file);
                        }
                    } else {
                        let app_web_path = get_app_web_path(app); // This would be implemented elsewhere
                        url_link_to = format!("{}/{}", app_web_path, file);
                    }
                } else {
                    url_link_to = format!("{}/{}/{}", self.webroot, app, file);
                }
            } else {
                url_link_to = format!("{}/{}/{}", self.webroot, app, file);
            }
        } else {
            let core_path = format!("{}/core/{}", self.serverroot, file);
            if Path::new(&core_path).exists() {
                url_link_to = format!("{}/core/{}", self.webroot, file);
            } else {
                url_link_to = format!("{}/{}", self.webroot, file);
            }
        }

        if !args.is_empty() {
            let query = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(args.iter())
                .finish();
            url_link_to = format!("{}?{}", url_link_to, query);
        }

        url_link_to
    }

    /// Creates path to an image
    ///
    /// # Arguments
    /// * `app` - Application name
    /// * `image` - Image name
    ///
    /// # Returns
    /// The path to the image
    fn image_path(&self, app: &str, image: &str) -> Result<String, String> {
        // Read the selected theme from the config file
        let theme = get_theme(); // This would be implemented elsewhere
        
        // If a theme has a png but not an svg always use the png
        let basename = if image.ends_with(".svg") || image.ends_with(".png") {
            &image[..image.len() - 4]
        } else {
            image
        };

        // Check various paths for the image
        let path_possibilities = [
            // Theme app-specific paths
            (format!("{}/themes/{}/apps/{}/img/{}", self.serverroot, theme, app, image),
             format!("{}/themes/{}/apps/{}/img/{}", self.webroot, theme, app, image)),
            
            (format!("{}/themes/{}/apps/{}/img/{}.png", self.serverroot, theme, app, basename),
             format!("{}/themes/{}/apps/{}/img/{}.png", self.webroot, theme, app, basename)),
            
            // App paths
            (format!("{}/img/{}", get_app_path(app).unwrap_or_default(), image),
             format!("{}/img/{}", get_app_web_path(app), image)),
            
            (format!("{}/img/{}.png", get_app_path(app).unwrap_or_default(), basename),
             format!("{}/img/{}.png", get_app_web_path(app), basename)),
            
            // Theme paths
            (format!("{}/themes/{}/{}/img/{}", self.serverroot, theme, app, image),
             format!("{}/themes/{}/{}/img/{}", self.webroot, theme, app, image)),
            
            (format!("{}/themes/{}/{}/img/{}.png", self.serverroot, theme, app, basename),
             format!("{}/themes/{}/{}/img/{}.png", self.webroot, theme, app, basename)),
            
            // App root paths
            (format!("{}/{}/img/{}", self.serverroot, app, image),
             format!("{}/{}/img/{}", self.webroot, app, image)),
            
            (format!("{}/{}/img/{}.png", self.serverroot, app, basename),
             format!("{}/{}/img/{}.png", self.webroot, app, basename)),
            
            // Core theme paths
            (format!("{}/themes/{}/core/img/{}", self.serverroot, theme, image),
             format!("{}/themes/{}/core/img/{}", self.webroot, theme, image)),
            
            (format!("{}/themes/{}/core/img/{}.png", self.serverroot, theme, basename),
             format!("{}/themes/{}/core/img/{}.png", self.webroot, theme, basename)),
            
            // Core paths
            (format!("{}/core/img/{}", self.serverroot, image),
             format!("{}/core/img/{}", self.webroot, image)),
        ];

        for (file_path, url_path) in path_possibilities.iter() {
            if !file_path.is_empty() && Path::new(file_path).exists() {
                return Ok(url_path.clone());
            }
        }

        // Special case for SVG
        let svg_path = format!("{}/themes/{}/core/img/{}.svg", self.serverroot, theme, basename);
        let png_path = format!("{}/themes/{}/core/img/{}.png", self.serverroot, theme, basename);
        if !Path::new(&svg_path).exists() && Path::new(&png_path).exists() {
            return Ok(format!("{}/themes/{}/core/img/{}.png", self.webroot, theme, basename));
        }

        Err(format!("image not found: image:{} webroot:{} serverroot:{}", 
                   image, self.webroot, self.serverroot))
    }

    /// Makes an URL absolute
    ///
    /// # Arguments
    /// * `url` - The URL in the owncloud host
    ///
    /// # Returns
    /// The absolute version of the URL
    fn get_absolute_url(&self, url: &str) -> String {
        format!("{}://{}{}", server_protocol(), server_host(), url)
    }
}

// Helper functions that would be implemented elsewhere
fn get_router() -> Router {
    // This would be implemented elsewhere
    Router {}
}

fn get_app_path(app: &str) -> Option<String> {
    // This would be implemented elsewhere
    Some(format!("/path/to/apps/{}", app))
}

fn get_app_web_path(app: &str) -> String {
    // This would be implemented elsewhere
    format!("/apps/{}", app)
}

fn get_theme() -> String {
    // This would be implemented elsewhere
    "default".to_string()
}

fn server_protocol() -> String {
    // This would be implemented elsewhere
    "https".to_string()
}

fn server_host() -> String {
    // This would be implemented elsewhere
    "example.com".to_string()
}

// Placeholder struct for the Router
struct Router {}

impl Router {
    fn generate(&self, route: &str, parameters: HashMap<String, String>) -> String {
        // This would be implemented elsewhere
        format!("/index.php/{}", route)
    }
}