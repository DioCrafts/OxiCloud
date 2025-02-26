use std::collections::HashMap;
use std::sync::Arc;
use std::str::FromStr;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Error as SqlxError, query, query_as};
use log::{debug, error};

/// Constante para etiqueta de favoritos
pub const TAG_FAVORITE: &str = "_$!<Favorite>!$_";

/// Tabla para las etiquetas
const TAG_TABLE: &str = "*PREFIX*vcategory";
/// Tabla para las relaciones entre etiquetas y objetos
const RELATION_TABLE: &str = "*PREFIX*vcategory_to_object";

/// Estructura que representa una etiqueta
#[derive(Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

/// Interfaz para el manejo de etiquetas
#[async_trait]
pub trait ITags: Send + Sync {
    /// Comprueba si no hay etiquetas
    async fn is_empty(&self) -> Result<bool, Error>;
    
    /// Obtiene todas las etiquetas
    async fn get_tags(&self) -> Vec<Tag>;
    
    /// Obtiene los IDs de objetos con una etiqueta específica
    async fn get_ids_for_tag(&self, tag: TagId) -> Result<Vec<i64>, Error>;
    
    /// Comprueba si existe una etiqueta
    fn has_tag(&self, name: &str) -> bool;
    
    /// Añade una nueva etiqueta
    async fn add(&mut self, name: &str) -> Result<i64, Error>;
    
    /// Renombra una etiqueta
    async fn rename(&mut self, from: &str, to: &str) -> Result<bool, Error>;
    
    /// Añade múltiples etiquetas
    async fn add_multiple(&mut self, names: &[String], sync: bool, id: Option<i64>) -> Result<bool, Error>;
    
    /// Elimina las relaciones de etiquetas para objetos específicos
    async fn purge_objects(&self, ids: &[i64]) -> Result<bool, Error>;
    
    /// Obtiene los objetos favoritos
    async fn get_favorites(&self) -> Vec<i64>;
    
    /// Marca un objeto como favorito
    async fn add_to_favorites(&mut self, obj_id: i64) -> Result<bool, Error>;
    
    /// Elimina un objeto de favoritos
    async fn remove_from_favorites(&mut self, obj_id: i64) -> Result<bool, Error>;
    
    /// Etiqueta un objeto
    async fn tag_as(&mut self, obj_id: i64, tag: TagId) -> Result<bool, Error>;
    
    /// Elimina la etiqueta de un objeto
    async fn un_tag(&self, obj_id: i64, tag: TagId) -> Result<bool, Error>;
    
    /// Elimina etiquetas
    async fn delete(&mut self, names: &[String]) -> Result<bool, Error>;
}

/// Error específico para operaciones de etiquetas
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    
    #[error("Tag not found: {0}")]
    TagNotFound(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Tipo para identificar una etiqueta (por ID o nombre)
#[derive(Debug, Clone)]
pub enum TagId {
    Id(i64),
    Name(String),
}

impl From<i64> for TagId {
    fn from(id: i64) -> Self {
        TagId::Id(id)
    }
}

impl From<&str> for TagId {
    fn from(name: &str) -> Self {
        TagId::Name(name.to_string())
    }
}

impl From<String> for TagId {
    fn from(name: String) -> Self {
        TagId::Name(name)
    }
}

/// Estructura para la relación temporal entre objetos y etiquetas
#[derive(Debug, Clone)]
struct TagRelation {
    obj_id: i64,
    tag: String,
}

/// Implementación de las etiquetas
pub struct Tags {
    /// Mapa de etiquetas (id -> nombre)
    tags: HashMap<i64, String>,
    
    /// Relaciones temporales entre objetos y etiquetas
    relations: Vec<TagRelation>,
    
    /// Tipo de etiqueta
    type_: String,
    
    /// Usuario propietario de las etiquetas
    user: String,
    
    /// Conexión a la base de datos
    db: Arc<Pool<Postgres>>,
}

impl Tags {
    /// Crea una nueva instancia de Tags
    pub async fn new(
        db: Arc<Pool<Postgres>>, 
        user: String, 
        type_: String, 
        default_tags: Vec<String>
    ) -> Result<Self, Error> {
        let mut tags = Tags {
            tags: HashMap::new(),
            relations: Vec::new(),
            type_,
            user,
            db,
        };
        
        tags.load_tags(&default_tags).await?;
        Ok(tags)
    }
    
    /// Carga las etiquetas desde la base de datos
    async fn load_tags(&mut self, default_tags: &[String]) -> Result<(), Error> {
        self.tags.clear();
        
        let sql = format!(
            "SELECT id, category FROM {} WHERE uid = $1 AND type = $2 ORDER BY category",
            TAG_TABLE
        );
        
        let rows = query(&sql)
            .bind(&self.user)
            .bind(&self.type_)
            .fetch_all(&*self.db)
            .await?;
            
        for row in rows {
            let id: i64 = row.get("id");
            let category: String = row.get("category");
            self.tags.insert(id, category);
        }
        
        if !default_tags.is_empty() && self.tags.is_empty() {
            self.add_multiple(default_tags, true, None).await?;
        }
        
        debug!("Tags loaded: {:?}", self.tags);
        Ok(())
    }
    
    /// Guarda las etiquetas y sus relaciones con objetos
    async fn save(&mut self) -> Result<(), Error> {
        // Guarda las etiquetas
        for tag in self.tags.values() {
            let sql = format!(
                "INSERT INTO {} (uid, type, category) 
                 VALUES ($1, $2, $3) 
                 ON CONFLICT (uid, type, category) DO NOTHING",
                TAG_TABLE
            );
            
            query(&sql)
                .bind(&self.user)
                .bind(&self.type_)
                .bind(tag)
                .execute(&*self.db)
                .await?;
        }
        
        // Recarga las etiquetas para obtener los IDs correctos
        self.load_tags(&[]).await?;
        
        // Procesa las relaciones temporales
        for relation in self.relations.drain(..) {
            let tag_id = self.array_searchi(&relation.tag)
                .ok_or_else(|| Error::TagNotFound(relation.tag.clone()))?;
                
            debug!("Saving relation: obj={}, tag={}, id={}", relation.obj_id, relation.tag, tag_id);
            
            let sql = format!(
                "INSERT INTO {} (objid, categoryid, type) 
                 VALUES ($1, $2, $3) 
                 ON CONFLICT (objid, categoryid, type) DO NOTHING",
                RELATION_TABLE
            );
            
            query(&sql)
                .bind(relation.obj_id)
                .bind(tag_id)
                .bind(&self.type_)
                .execute(&*self.db)
                .await?;
        }
        
        Ok(())
    }
    
    /// Busca una etiqueta por nombre, ignorando mayúsculas/minúsculas
    fn array_searchi(&self, needle: &str) -> Option<i64> {
        let needle_lower = needle.to_lowercase();
        for (id, name) in &self.tags {
            if name.to_lowercase() == needle_lower {
                return Some(*id);
            }
        }
        None
    }
}

/// Métodos para gestión de usuarios (hooks)
pub struct TagsUtil;

impl TagsUtil {
    /// Hook para eliminar etiquetas cuando se elimina un usuario
    pub async fn post_delete_user(db: &Pool<Postgres>, uid: &str) -> Result<(), Error> {
        // Primero obtenemos los IDs de las etiquetas del usuario
        let tag_ids: Vec<i64> = query_as::<_, (i64,)>(&format!("SELECT id FROM {} WHERE uid = $1", TAG_TABLE))
            .bind(uid)
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|(id,)| id)
            .collect();
        
        // Eliminamos las relaciones de las etiquetas
        for id in &tag_ids {
            query(&format!("DELETE FROM {} WHERE categoryid = $1", RELATION_TABLE))
                .bind(id)
                .execute(db)
                .await?;
        }
        
        // Eliminamos las etiquetas
        query(&format!("DELETE FROM {} WHERE uid = $1", TAG_TABLE))
            .bind(uid)
            .execute(db)
            .await?;
            
        Ok(())
    }
}

#[async_trait]
impl ITags for Tags {
    async fn is_empty(&self) -> Result<bool, Error> {
        let sql = format!(
            "SELECT COUNT(*) FROM {} WHERE uid = $1 AND type = $2",
            TAG_TABLE
        );
        
        let count: i64 = query_as::<_, (i64,)>(&sql)
            .bind(&self.user)
            .bind(&self.type_)
            .fetch_one(&*self.db)
            .await?
            .0;
            
        Ok(count == 0)
    }
    
    async fn get_tags(&self) -> Vec<Tag> {
        if self.tags.is_empty() {
            return Vec::new();
        }
        
        let mut tag_map = Vec::new();
        let mut tags: Vec<_> = self.tags.iter().collect();
        tags.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
        
        for (&id, name) in tags {
            if name != TAG_FAVORITE {
                tag_map.push(Tag {
                    id,
                    name: name.clone(),
                });
            }
        }
        
        tag_map
    }
    
    async fn get_ids_for_tag(&self, tag: TagId) -> Result<Vec<i64>, Error> {
        let tag_id = match tag {
            TagId::Id(id) => id,
            TagId::Name(name) => {
                self.array_searchi(&name)
                    .ok_or_else(|| Error::TagNotFound(name))?
            }
        };
        
        let sql = format!(
            "SELECT objid FROM {} WHERE categoryid = $1",
            RELATION_TABLE
        );
        
        let results: Vec<(i64,)> = query_as(&sql)
            .bind(tag_id)
            .fetch_all(&*self.db)
            .await?;
            
        Ok(results.into_iter().map(|(id,)| id).collect())
    }
    
    fn has_tag(&self, name: &str) -> bool {
        self.array_searchi(name).is_some()
    }
    
    async fn add(&mut self, name: &str) -> Result<i64, Error> {
        let name = name.trim();
        
        if self.has_tag(name) {
            debug!("Tag {} exists already", name);
            return Ok(self.array_searchi(name).unwrap());
        }
        
        let sql = format!(
            "INSERT INTO {} (uid, type, category) VALUES ($1, $2, $3) RETURNING id",
            TAG_TABLE
        );
        
        let id: i64 = query_as::<_, (i64,)>(&sql)
            .bind(&self.user)
            .bind(&self.type_)
            .bind(name)
            .fetch_one(&*self.db)
            .await?
            .0;
            
        debug!("Added tag with id: {}", id);
        self.tags.insert(id, name.to_string());
        
        Ok(id)
    }
    
    async fn rename(&mut self, from: &str, to: &str) -> Result<bool, Error> {
        let from = from.trim();
        let to = to.trim();
        
        let id = self.array_searchi(from)
            .ok_or_else(|| Error::TagNotFound(from.to_string()))?;
            
        let sql = format!(
            "UPDATE {} SET category = $1 WHERE uid = $2 AND type = $3 AND id = $4",
            TAG_TABLE
        );
        
        query(&sql)
            .bind(to)
            .bind(&self.user)
            .bind(&self.type_)
            .bind(id)
            .execute(&*self.db)
            .await?;
            
        self.tags.insert(id, to.to_string());
        
        Ok(true)
    }
    
    async fn add_multiple(&mut self, names: &[String], sync: bool, id: Option<i64>) -> Result<bool, Error> {
        let names: Vec<String> = names.iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
            
        let mut new_ones = Vec::new();
        
        for name in &names {
            if !self.has_tag(name) {
                new_ones.push(name.clone());
            }
            
            if let Some(obj_id) = id {
                self.relations.push(TagRelation {
                    obj_id,
                    tag: name.clone(),
                });
            }
        }
        
        for name in new_ones {
            self.tags.insert(0, name); // ID temporal hasta sincronización
        }
        
        if sync {
            self.save().await?;
        }
        
        Ok(true)
    }
    
    async fn purge_objects(&self, ids: &[i64]) -> Result<bool, Error> {
        if ids.is_empty() {
            return Ok(true);
        }
        
        // Construye parámetros para la consulta SQL
        let params: Vec<String> = (1..=ids.len()).map(|i| format!("${}", i)).collect();
        let placeholders = params.join(",");
        
        let sql = format!(
            "DELETE FROM {} WHERE objid IN ({}) AND type = ${}",
            RELATION_TABLE, placeholders, ids.len() + 1
        );
        
        let mut query = query(&sql);
        for id in ids {
            query = query.bind(id);
        }
        query = query.bind(&self.type_);
        
        query.execute(&*self.db).await?;
        
        Ok(true)
    }
    
    async fn get_favorites(&self) -> Vec<i64> {
        match self.get_ids_for_tag(TAG_FAVORITE.into()).await {
            Ok(ids) => ids,
            Err(e) => {
                error!("Error getting favorites: {}", e);
                Vec::new()
            }
        }
    }
    
    async fn add_to_favorites(&mut self, obj_id: i64) -> Result<bool, Error> {
        if !self.has_tag(TAG_FAVORITE) {
            self.add(TAG_FAVORITE).await?;
        }
        
        self.tag_as(obj_id, TAG_FAVORITE.into()).await
    }
    
    async fn remove_from_favorites(&mut self, obj_id: i64) -> Result<bool, Error> {
        self.un_tag(obj_id, TAG_FAVORITE.into()).await
    }
    
    async fn tag_as(&mut self, obj_id: i64, tag: TagId) -> Result<bool, Error> {
        let tag_id = match tag {
            TagId::Id(id) => id,
            TagId::Name(name) => {
                let name = name.trim().to_string();
                if !self.has_tag(&name) {
                    self.add(&name).await?
                } else {
                    self.array_searchi(&name).unwrap()
                }
            }
        };
        
        let sql = format!(
            "INSERT INTO {} (objid, categoryid, type) 
             VALUES ($1, $2, $3) 
             ON CONFLICT (objid, categoryid, type) DO NOTHING",
            RELATION_TABLE
        );
        
        query(&sql)
            .bind(obj_id)
            .bind(tag_id)
            .bind(&self.type_)
            .execute(&*self.db)
            .await?;
            
        Ok(true)
    }
    
    async fn un_tag(&self, obj_id: i64, tag: TagId) -> Result<bool, Error> {
        let tag_id = match tag {
            TagId::Id(id) => id,
            TagId::Name(name) => {
                self.array_searchi(&name)
                    .ok_or_else(|| Error::TagNotFound(name))?
            }
        };
        
        let sql = format!(
            "DELETE FROM {} WHERE objid = $1 AND categoryid = $2 AND type = $3",
            RELATION_TABLE
        );
        
        query(&sql)
            .bind(obj_id)
            .bind(tag_id)
            .bind(&self.type_)
            .execute(&*self.db)
            .await?;
            
        Ok(true)
    }
    
    async fn delete(&mut self, names: &[String]) -> Result<bool, Error> {
        if names.is_empty() {
            return Ok(true);
        }
        
        for name in names {
            let name = name.trim();
            let id = self.array_searchi(name);
            
            if let Some(tag_id) = id {
                self.tags.remove(&tag_id);
                
                // Elimina la etiqueta
                let sql = format!(
                    "DELETE FROM {} WHERE uid = $1 AND type = $2 AND category = $3",
                    TAG_TABLE
                );
                
                query(&sql)
                    .bind(&self.user)
                    .bind(&self.type_)
                    .bind(name)
                    .execute(&*self.db)
                    .await?;
                    
                // Elimina las relaciones
                let sql = format!(
                    "DELETE FROM {} WHERE categoryid = $1",
                    RELATION_TABLE
                );
                
                query(&sql)
                    .bind(tag_id)
                    .execute(&*self.db)
                    .await?;
            }
        }
        
        Ok(true)
    }
}