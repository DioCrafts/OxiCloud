use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "清除映射失败。");
        m.insert("Failed to delete the server configuration", "未能删除服务器配置");
        m.insert("The configuration is valid and the connection could be established!", "配置有效，能够建立连接！");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "配置有效但绑定失败。请检查服务器设置和认证信息。");
        m.insert("Deletion failed", "删除失败");
        m.insert("Take over settings from recent server configuration?", "从近期的服务器配置中导入设置？");
        m.insert("Keep settings?", "保留设置吗？");
        m.insert("Cannot add server configuration", "无法添加服务器配置");
        m.insert("mappings cleared", "清除映射");
        m.insert("Success", "成功");
        m.insert("Error", "错误");
        m.insert("Select groups", "选择分组");
        m.insert("Connection test succeeded", "连接测试成功");
        m.insert("Connection test failed", "连接测试失败");
        m.insert("Do you really want to delete the current Server Configuration?", "您真的想要删除当前服务器配置吗？");
        m.insert("Confirm Deletion", "确认删除");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "保存");
        m.insert("Test Configuration", "测试配置");
        m.insert("Help", "帮助");
        m.insert("Add Server Configuration", "添加服务器配置");
        m.insert("Host", "主机");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "可以忽略协议，但如要使用SSL，则需以ldaps://开头");
        m.insert("Port", "端口");
        m.insert("User DN", "User DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "客户端使用的DN必须与绑定的相同，比如uid=agent,dc=example,dc=com\n如需匿名访问，将DN和密码保留为空");
        m.insert("Password", "密码");
        m.insert("For anonymous access, leave DN and Password empty.", "启用匿名访问，将DN和密码保留为空");
        m.insert("One Base DN per line", "每行一个基本判别名");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "您可以在高级选项卡里为用户和组指定Base DN");
        m.insert("Back", "返回");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>警告：</b> PHP LDAP 模块未安装，后端将无法工作。请请求您的系统管理员安装该模块。");
        m.insert("Connection Settings", "连接设置");
        m.insert("Configuration Active", "现行配置");
        m.insert("When unchecked, this configuration will be skipped.", "当反选后，此配置将被忽略。");
        m.insert("User Login Filter", "用户登录过滤");
        m.insert("Backup (Replica) Host", "备份 (镜像) 主机");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "给出一个可选的备份主机。它必须为主 LDAP/AD 服务器的一个镜像。");
        m.insert("Backup (Replica) Port", "备份 (镜像) 端口");
        m.insert("Disable Main Server", "禁用主服务器");
        m.insert("Case insensitve LDAP server (Windows)", "大小写敏感LDAP服务器(Windows)");
        m.insert("Turn off SSL certificate validation.", "关闭SSL证书验证");
        m.insert("Cache Time-To-Live", "缓存存活时间");
        m.insert("in seconds. A change empties the cache.", "以秒计。修改将清空缓存。");
        m.insert("Directory Settings", "目录设置");
        m.insert("User Display Name Field", "用户显示名称字段");
        m.insert("Base User Tree", "基础用户树");
        m.insert("One User Base DN per line", "每行一个用户基准判别名");
        m.insert("User Search Attributes", "用户搜索属性");
        m.insert("Optional; one attribute per line", "可选;每行一个属性");
        m.insert("Group Display Name Field", "组显示名称字段");
        m.insert("Base Group Tree", "基础组树");
        m.insert("One Group Base DN per line", "每行一个群组基准判别名");
        m.insert("Group Search Attributes", "群组搜索属性");
        m.insert("Group-Member association", "组成员关联");
        m.insert("Special Attributes", "特殊属性");
        m.insert("Quota Field", "配额字段");
        m.insert("Quota Default", "默认配额");
        m.insert("in bytes", "字节数");
        m.insert("Email Field", "电邮字段");
        m.insert("User Home Folder Naming Rule", "用户主目录命名规则");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "将用户名称留空(默认)。否则指定一个LDAP/AD属性");
        m.insert("Internal Username", "内部用户名");
        m.insert("Internal Username Attribute:", "内部用户名属性：");
        m.insert("Override UUID detection", "超越UUID检测");
        m.insert("Username-LDAP User Mapping", "用户名-LDAP用户映射");
        m.insert("Clear Username-LDAP User Mapping", "清除用户-LDAP用户映射");
        m.insert("Clear Groupname-LDAP Group Mapping", "清除组用户-LDAP级映射");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}