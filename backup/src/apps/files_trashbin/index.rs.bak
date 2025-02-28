use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use regex::Regex;
use serde::Deserialize;

mod ocp {
    pub struct User;
    pub struct App;
    pub struct Util;
    pub struct Template<'a> {
        pub name: &'a str,
        pub template: &'a str,
        pub type_: &'a str,
        pub vars: std::collections::HashMap<String, String>,
    }

    impl User {
        pub async fn check_logged_in() -> Result<(), String> {
            // Implementación para verificar si el usuario está logueado
            Ok(())
        }
    }

    impl App {
        pub async fn set_active_navigation_entry(entry: &str) -> Result<(), String> {
            // Implementación para establecer la entrada de navegación activa
            Ok(())
        }
    }

    impl Util {
        pub async fn add_script(app: &str, script: &str) -> Result<(), String> {
            // Implementación para agregar scripts
            Ok(())
        }

        pub async fn add_style(app: &str, style: &str) -> Result<(), String> {
            // Implementación para agregar estilos
            Ok(())
        }

        pub fn link_to(app: &str, file: &str) -> String {
            // Implementación para generar enlaces
            format!("/{}/{}", app, file)
        }

        pub fn encode_path(path: &str) -> String {
            // Implementación para codificar rutas
            urlencoding::encode(path).to_string()
        }
    }

    impl<'a> Template<'a> {
        pub fn new(app: &'a str, template: &'a str, type_: &'a str) -> Self {
            Self {
                name: app,
                template,
                type_,
                vars: std::collections::HashMap::new(),
            }
        }

        pub fn assign(&mut self, key: &str, value: String) {
            self.vars.insert(key.to_string(), value);
        }

        pub async fn fetch_page(&self) -> String {
            // Implementación para renderizar la plantilla
            format!("Rendered template: {}/{}", self.name, self.template)
        }

        pub async fn print_page(&self) -> String {
            // Implementación para imprimir la página
            format!("Printed page: {}/{}", self.name, self.template)
        }
    }
}

mod oca {
    pub struct Files_Trashbin {
        pub helper: Helper,
    }

    pub struct Helper;

    impl Helper {
        pub async fn get_trash_files(dir: &str) -> Option<Vec<File>> {
            // Implementación para obtener archivos de la papelera
            Some(vec![])
        }

        pub fn make_breadcrumb(dir: &str) -> Vec<Breadcrumb> {
            // Implementación para crear migas de pan
            vec![]
        }
    }

    pub struct File;
    pub struct Breadcrumb;
}

#[derive(Deserialize)]
struct TrashQuery {
    dir: Option<String>,
}

#[get("/index.php")]
async fn index(query: web::Query<TrashQuery>, req: HttpRequest) -> impl Responder {
    // Verificar si el usuario está logueado
    if let Err(e) = ocp::User::check_logged_in().await {
        return HttpResponse::Forbidden().body(e);
    }

    // Establecer la entrada de navegación activa
    if let Err(e) = ocp::App::set_active_navigation_entry("files_index").await {
        return HttpResponse::InternalServerError().body(e);
    }

    // Agregar scripts y estilos
    let _ = ocp::Util::add_script("files_trashbin", "disableDefaultActions").await;
    let _ = ocp::Util::add_script("files", "fileactions").await;
    let _ = ocp::Util::add_style("files", "files").await;
    let _ = ocp::Util::add_script("files", "filelist").await;
    let _ = ocp::Util::add_script("files_trashbin", "filelist").await;
    let _ = ocp::Util::add_script("files", "files").await;
    let _ = ocp::Util::add_script("files_trashbin", "trash").await;

    // Crear la plantilla principal
    let mut tmpl = ocp::Template::new("files_trashbin", "index", "user");

    // Obtener el directorio de la consulta
    let dir = query.dir.as_deref().unwrap_or("").to_string();
    let dir = dir.trim_matches('/');
    let dir = if dir.is_empty() { "/".to_string() } else { format!("/{}", dir) };

    // Comprobar si es IE8
    let user_agent = req.headers().get("User-Agent")
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");
    let re = Regex::new(r"MSIE (.*?);").unwrap();
    let is_ie8 = re.captures(user_agent)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
        .map(|v| v <= 8)
        .unwrap_or(false);

    // Redirección para IE8
    if is_ie8 && query.dir.is_some() {
        let redirect_dir = if dir == "/" { dir.clone() } else { dir.clone() };
        let redirect_url = format!("{}#?dir={}", 
            ocp::Util::link_to("files_trashbin", "index.php"),
            ocp::Util::encode_path(&redirect_dir));
        
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", redirect_url))
            .finish();
    }

    // Cargar archivos
    let files = if !is_ie8 {
        match oca::Helper::get_trash_files(&dir).await {
            Some(files) => files,
            None => {
                // Redireccionar si el directorio no existe
                let redirect_url = ocp::Util::link_to("files_trashbin", "index.php");
                return HttpResponse::TemporaryRedirect()
                    .append_header(("Location", redirect_url))
                    .finish();
            }
        }
    } else {
        vec![]
    };

    let ajax_load = is_ie8 || true;

    // Determinar si estamos en un directorio
    let dirlisting = dir != "/" && !dir.is_empty();

    // Crear las migas de pan
    let breadcrumb = oca::Helper::make_breadcrumb(&dir);
    let mut breadcrumb_nav = ocp::Template::new("files_trashbin", "part.breadcrumb", "");
    breadcrumb_nav.assign("breadcrumb", serde_json::to_string(&breadcrumb).unwrap());
    breadcrumb_nav.assign("baseURL", format!("{}?dir=", ocp::Util::link_to("files_trashbin", "index.php")));
    breadcrumb_nav.assign("home", ocp::Util::link_to("files", "index.php"));

    // Crear la lista de archivos
    let mut list = ocp::Template::new("files_trashbin", "part.list", "");
    list.assign("files", serde_json::to_string(&files).unwrap());

    let encoded_dir = ocp::Util::encode_path(&dir);
    list.assign("baseURL", format!("{}?dir={}", ocp::Util::link_to("files_trashbin", "index.php"), encoded_dir));
    list.assign("downloadURL", format!("{}?file={}", ocp::Util::link_to("files_trashbin", "download.php"), encoded_dir));
    list.assign("dirlisting", dirlisting.to_string());
    list.assign("disableDownloadActions", "true".to_string());

    // Asignar variables a la plantilla principal
    tmpl.assign("dirlisting", dirlisting.to_string());
    tmpl.assign("breadcrumb", breadcrumb_nav.fetch_page().await);
    tmpl.assign("fileList", list.fetch_page().await);
    tmpl.assign("files", serde_json::to_string(&files).unwrap());
    tmpl.assign("dir", dir);
    tmpl.assign("disableSharing", "true".to_string());
    tmpl.assign("ajaxLoad", ajax_load.to_string());

    // Renderizar la página
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(tmpl.print_page().await)
}

// Configuración de la aplicación
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}