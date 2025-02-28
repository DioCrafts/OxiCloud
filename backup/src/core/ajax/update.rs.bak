use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;

// Mocks de las estructuras existentes en PHP
struct OC;
struct L10N {
    locale: String,
}
struct EventSource {
    sender: mpsc::Sender<EventMessage>,
}
struct Updater {
    logger: Arc<dyn Logger>,
    listeners: Vec<Box<dyn Fn(&str, &str) + Send + Sync>>,
    cancellation_token: CancellationToken,
}
trait Logger: Send + Sync {
    fn log(&self, message: &str);
}

#[derive(Serialize, Deserialize, Clone)]
struct EventMessage {
    event: String,
    message: String,
}

impl OC {
    async fn check_upgrade(auto_enable: bool) -> bool {
        // Simulación de la función PHP
        // En una implementación real, aquí habría lógica para verificar si es necesaria una actualización
        true
    }
}

impl L10N {
    fn new(module: &str) -> Self {
        // En una implementación real, aquí se cargarían las traducciones para el módulo especificado
        Self {
            locale: "en".to_string(),
        }
    }

    fn t(&self, message: &str) -> String {
        // Simulación simple de traducción
        message.to_string()
    }

    fn t_with_args(&self, message: &str, args: &[(&str, &str)]) -> String {
        let mut result = message.to_string();
        for (key, value) in args {
            result = result.replace(&format!("%{}", key), value);
        }
        result
    }
}

impl EventSource {
    fn new(sender: mpsc::Sender<EventMessage>) -> Self {
        Self { sender }
    }

    async fn send(&self, event: &str, message: &str) {
        if let Err(e) = self.sender
            .send(EventMessage {
                event: event.to_string(),
                message: message.to_string(),
            })
            .await
        {
            error!("Error sending event: {}", e);
        }
    }

    async fn close(&self) {
        // El cierre se maneja automáticamente cuando se descarta el sender
    }
}

impl Updater {
    fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            logger,
            listeners: Vec::new(),
            cancellation_token: CancellationToken::new(),
        }
    }

    fn listen<F>(&mut self, event_type: &str, event_name: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let event_full_name = format!("{}{}", event_type, event_name);
        self.listeners.push(Box::new(move |e_type, e_name| {
            let full_name = format!("{}{}", e_type, e_name);
            if full_name == event_full_name {
                callback();
            }
        }));
    }

    fn listen_with_args<F>(&mut self, event_type: &str, event_name: &str, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let event_full_name = format!("{}{}", event_type, event_name);
        self.listeners.push(Box::new(move |e_type, e_name| {
            let full_name = format!("{}{}", e_type, e_name);
            if full_name == event_full_name {
                callback("50"); // Simulamos el porcentaje para filecacheProgress
            }
        }));
    }

    async fn upgrade(&self) -> Result<(), String> {
        // Simulación del proceso de actualización
        
        // Inicio de mantenimiento
        self.trigger_event("\\OC\\Updater", "maintenanceStart").await;
        
        // Actualización de base de datos
        self.trigger_event("\\OC\\Updater", "dbUpgrade").await;
        
        // Actualización de caché de archivos
        self.trigger_event("\\OC\\Updater", "filecacheStart").await;
        
        // Progreso de caché de archivos
        self.trigger_event("\\OC\\Updater", "filecacheProgress").await;
        
        // Finalización de caché de archivos
        self.trigger_event("\\OC\\Updater", "filecacheDone").await;
        
        // Fin de mantenimiento
        self.trigger_event("\\OC\\Updater", "maintenanceEnd").await;
        
        Ok(())
    }

    async fn trigger_event(&self, event_type: &str, event_name: &str) {
        // En una implementación real, esto activaría callbacks registrados
        for listener in &self.listeners {
            listener(event_type, event_name);
        }
        // Simulamos algún trabajo
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

async fn handle_update() -> impl Responder {
    // Simular la verificación de actualización
    if OC::check_upgrade(false).await {
        // Crear los canales para la comunicación de eventos
        let (tx, rx) = mpsc::channel::<EventMessage>(100);
        let event_source = EventSource::new(tx);
        
        // Inicializar el traductor
        let l10n = Arc::new(L10N::new("core"));
        
        // Crear un logger mock
        struct MockLogger;
        impl Logger for MockLogger {
            fn log(&self, message: &str) {
                info!("{}", message);
            }
        }
        
        let logger = Arc::new(MockLogger);
        let mut updater = Updater::new(logger);
        
        // Registrar los listeners para los eventos
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen("\\OC\\Updater", "maintenanceStart", move || {
            let es = es_clone.clone();
            let l = l_clone.clone();
            tokio::spawn(async move {
                es.send("success", &l.t("Turned on maintenance mode")).await;
            });
        });
        
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen("\\OC\\Updater", "maintenanceEnd", move || {
            let es = es_clone.clone();
            let l = l_clone.clone();
            tokio::spawn(async move {
                es.send("success", &l.t("Turned off maintenance mode")).await;
            });
        });
        
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen("\\OC\\Updater", "dbUpgrade", move || {
            let es = es_clone.clone();
            let l = l_clone.clone();
            tokio::spawn(async move {
                es.send("success", &l.t("Updated database")).await;
            });
        });
        
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen("\\OC\\Updater", "filecacheStart", move || {
            let es = es_clone.clone();
            let l = l_clone.clone();
            tokio::spawn(async move {
                es.send("success", &l.t("Updating filecache, this may take really long...")).await;
            });
        });
        
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen("\\OC\\Updater", "filecacheDone", move || {
            let es = es_clone.clone();
            let l = l_clone.clone();
            tokio::spawn(async move {
                es.send("success", &l.t("Updated filecache")).await;
            });
        });
        
        let es_clone = event_source.clone();
        let l_clone = l10n.clone();
        updater.listen_with_args("\\OC\\Updater", "filecacheProgress", move |percent| {
            let es = es_clone.clone();
            let l = l_clone.clone();
            let percent_str = percent.to_string();
            tokio::spawn(async move {
                es.send(
                    "success", 
                    &l.t_with_args("... %d%% done ...", &[("d", &percent_str)])
                ).await;
            });
        });
        
        let es_clone = event_source.clone();
        updater.listen_with_args("\\OC\\Updater", "failure", move |message| {
            let es = es_clone.clone();
            let message_str = message.to_string();
            tokio::spawn(async move {
                es.send("failure", &message_str).await;
                // En una implementación real, aquí se desactivaría el modo de mantenimiento
                // OC_Config::set_value("maintenance", "false");
            });
        });
        
        // Iniciar la actualización en segundo plano
        tokio::spawn(async move {
            if let Err(e) = updater.upgrade().await {
                error!("Update failed: {}", e);
            }
            
            // Enviar un evento "done" al finalizar
            event_source.send("done", "").await;
            // El evento se cerrará automáticamente cuando se descarte el sender
        });
        
        // Convertir el receptor en un stream para SSE
        let stream = ReceiverStream::new(rx).map(|msg| {
            Ok::<_, actix_web::Error>(
                web::Bytes::from(format!("event: {}\ndata: {}\n\n", msg.event, msg.message))
            )
        });
        
        // Devolver la respuesta SSE
        HttpResponse::Ok()
            .insert_header(("Content-Type", "text/event-stream"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Connection", "keep-alive"))
            .streaming(stream)
    } else {
        // Si no hay actualización disponible
        HttpResponse::NoContent().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // No configuramos un límite de tiempo como en PHP (set_time_limit(0))
    // ya que Rust maneja esto de manera diferente
    
    HttpServer::new(|| {
        App::new()
            .route("/core/ajax/update.php", web::get().to(handle_update))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}