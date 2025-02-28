use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "无法彻底删除文件%s");
        m.insert("Couldn't restore %s", "无法恢复%s");
        m.insert("Error", "错误");
        m.insert("restored", "已恢复");
        m.insert("Nothing in here. Your trash bin is empty!", "这里没有东西. 你的回收站是空的!");
        m.insert("Name", "名称");
        m.insert("Restore", "恢复");
        m.insert("Deleted", "已删除");
        m.insert("Delete", "删除");
        m.insert("Deleted Files", "已删除文件");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}