use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "帮助");
        m.insert("Personal", "个人");
        m.insert("Settings", "设置");
        m.insert("Users", "用户");
        m.insert("Admin", "管理");
        m.insert("Unknown filetype", "未知的文件类型");
        m.insert("Invalid image", "无效的图像");
        m.insert("web services under your control", "您控制的web服务");
        m.insert("ZIP download is turned off.", "ZIP 下载已经关闭");
        m.insert("Files need to be downloaded one by one.", "需要逐一下载文件");
        m.insert("Back to Files", "回到文件");
        m.insert("Selected files too large to generate zip file.", "选择的文件太大，无法生成 zip 文件。");
        m.insert("App does not provide an info.xml file", "应用未提供 info.xml 文件");
        m.insert("Application is not enabled", "应用程序未启用");
        m.insert("Authentication error", "认证出错");
        m.insert("Token expired. Please reload page.", "Token 过期，请刷新页面。");
        m.insert("Files", "文件");
        m.insert("Text", "文本");
        m.insert("Images", "图片");
        m.insert("%s enter the database username.", "%s 输入数据库用户名。");
        m.insert("%s enter the database name.", "%s 输入数据库名称。");
        m.insert("%s you may not use dots in the database name", "%s 您不能在数据库名称中使用英文句号。");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL 用户名和/或密码无效：%s");
        m.insert("You need to enter either an existing account or the administrator.", "你需要输入一个数据库中已有的账户或管理员账户。");
        m.insert("MySQL username and/or password not valid", "MySQL 数据库用户名和/或密码无效");
        m.insert("DB Error: \"%s\"", "数据库错误：\"%s\"");
        m.insert("Offending command was: \"%s\"", "冲突命令为：\"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL 用户 '%s'@'localhost' 已存在。");
        m.insert("Drop this user from MySQL", "建议从 MySQL 数据库中丢弃 Drop 此用户");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL 用户 '%s'@'%%' 已存在");
        m.insert("Drop this user from MySQL.", "建议从 MySQL 数据库中丢弃 Drop 此用户。");
        m.insert("Oracle connection could not be established", "不能建立甲骨文连接");
        m.insert("Oracle username and/or password not valid", "Oracle 数据库用户名和/或密码无效");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "冲突命令为：\"%s\"，名称：%s，密码：%s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL 数据库用户名和/或密码无效");
        m.insert("Set an admin username.", "请设置一个管理员用户名。");
        m.insert("Set an admin password.", "请设置一个管理员密码。");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "您的Web服务器尚未正确设置以允许文件同步, 因为WebDAV的接口似乎已损坏.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "请认真检查<a href='%s'>安装指南</a>.");
        m.insert("Could not find category \"%s\"", "无法找到分类 \"%s\"");
        m.insert("seconds ago", "秒前");
        m.insert("_%n minute ago_::_%n minutes ago_", "%n 分钟前");
        m.insert("_%n hour ago_::_%n hours ago_", "%n 小时前");
        m.insert("today", "今天");
        m.insert("yesterday", "昨天");
        m.insert("_%n day go_::_%n days ago_", "%n 天前");
        m.insert("last month", "上月");
        m.insert("_%n month ago_::_%n months ago_", "%n 月前");
        m.insert("last year", "去年");
        m.insert("years ago", "年前");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub struct ZhCn;

impl ZhCn {
    pub fn get_translation(key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }

    pub fn get_plural_form() -> &'static str {
        &PLURAL_FORMS
    }
}