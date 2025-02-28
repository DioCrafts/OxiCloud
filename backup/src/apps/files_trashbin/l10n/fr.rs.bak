use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Impossible d'effacer %s de façon permanente");
        m.insert("Couldn't restore %s", "Impossible de restaurer %s");
        m.insert("Error", "Erreur");
        m.insert("restored", "restauré");
        m.insert("Nothing in here. Your trash bin is empty!", "Il n'y a rien ici. Votre corbeille est vide !");
        m.insert("Name", "Nom");
        m.insert("Restore", "Restaurer");
        m.insert("Deleted", "Effacé");
        m.insert("Delete", "Supprimer");
        m.insert("Deleted Files", "Fichiers effacés");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}