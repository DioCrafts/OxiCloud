use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Couldn't delete %s permanently", "Não foi possível excluir %s permanentemente");
    m.insert("Couldn't restore %s", "Não foi possível restaurar %s");
    m.insert("Error", "Erro");
    m.insert("restored", "restaurado");
    m.insert("Nothing in here. Your trash bin is empty!", "Nada aqui. Sua lixeira está vazia!");
    m.insert("Name", "Nome");
    m.insert("Restore", "Restaurar");
    m.insert("Deleted", "Excluído");
    m.insert("Delete", "Excluir");
    m.insert("Deleted Files", "Arquivos Apagados");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n > 1);";