// core/templates/layout_guest.rs

use askama::Template;
use std::collections::HashMap;

/// Header structure represents metadata in the HTML head
#[derive(Debug, Clone)]
pub struct Header {
    pub tag: String,
    pub attributes: HashMap<String, String>,
}

/// Theme structure to handle theme-specific details
pub trait Theme {
    fn get_title(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_logo_claim(&self) -> &str;
    fn get_long_footer(&self) -> &str;
}

/// GuestLayout template for rendering the guest layout
#[derive(Template)]
#[template(source = r#"
<!DOCTYPE html>
<!--[if lt IE 7]><html class="ng-csp ie ie6 lte9 lte8 lte7"><![endif]-->
<!--[if IE 7]><html class="ng-csp ie ie7 lte9 lte8 lte7"><![endif]-->
<!--[if IE 8]><html class="ng-csp ie ie8 lte9 lte8"><![endif]-->
<!--[if IE 9]><html class="ng-csp ie ie9 lte9"><![endif]-->
<!--[if gt IE 9]><html class="ng-csp ie"><![endif]-->
<!--[if !IE]><!--><html class="ng-csp"><!--<![endif]-->

    <head data-requesttoken="{{ requesttoken }}">
        <title>
        {{ theme_title }}
        </title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta name="apple-itunes-app" content="app-id=543672169">
        <link rel="shortcut icon" href="{{ favicon_path }}" />
        <link rel="apple-touch-icon-precomposed" href="{{ favicon_touch_path }}" />
        {% for cssfile in cssfiles %}
            <link rel="stylesheet" href="{{ cssfile|safe }}" type="text/css" media="screen" />
        {% endfor %}
        {% for jsfile in jsfiles %}
            <script type="text/javascript" src="{{ jsfile|safe }}"></script>
        {% endfor %}

        {% for header in headers %}
            <{{ header.tag }} 
            {% for name, value in header.attributes %}
                {{ name }}='{{ value }}' 
            {% endfor %}
            />
        {% endfor %}
    </head>

    <body id="body-login">
        <div class="wrapper"><!-- for sticky footer -->
            <header><div id="header">
                <img src="{{ logo_path|safe }}" class="svg" alt="{{ theme_name }}" />
                <div id="logo-claim" style="display:none;">{{ logo_claim }}</div>
            </div></header>

            {{ content|safe }}

            <div class="push"></div><!-- for sticky footer -->
        </div>

        <footer>
            <p class="info">
                {{ long_footer|safe }}
            </p>
        </footer>
    </body>
</html>
"#, escape = "none")]
pub struct GuestLayout<'a, T: Theme> {
    pub requesttoken: &'a str,
    pub theme: &'a T,
    pub theme_title: &'a str,
    pub theme_name: &'a str,
    pub logo_claim: &'a str,
    pub long_footer: &'a str,
    pub favicon_path: String,
    pub favicon_touch_path: String,
    pub logo_path: String,
    pub cssfiles: &'a [String],
    pub jsfiles: &'a [String],
    pub headers: &'a [Header],
    pub content: &'a str,
}

impl<'a, T: Theme> GuestLayout<'a, T> {
    pub fn new(
        requesttoken: &'a str,
        theme: &'a T,
        cssfiles: &'a [String],
        jsfiles: &'a [String],
        headers: &'a [Header],
        content: &'a str,
        image_path_fn: impl Fn(&str, &str) -> String,
    ) -> Self {
        GuestLayout {
            requesttoken,
            theme,
            theme_title: theme.get_title(),
            theme_name: theme.get_name(),
            logo_claim: theme.get_logo_claim(),
            long_footer: theme.get_long_footer(),
            favicon_path: image_path_fn("", "favicon.png"),
            favicon_touch_path: image_path_fn("", "favicon-touch.png"),
            logo_path: image_path_fn("", "logo.svg"),
            cssfiles,
            jsfiles,
            headers,
            content,
        }
    }
}

/// Utility function to render the guest layout template
pub async fn render_guest_layout<T: Theme>(
    requesttoken: &str,
    theme: &T,
    cssfiles: &[String],
    jsfiles: &[String],
    headers: &[Header],
    content: &str,
    image_path_fn: impl Fn(&str, &str) -> String,
) -> Result<String, askama::Error> {
    let template = GuestLayout::new(
        requesttoken,
        theme,
        cssfiles,
        jsfiles,
        headers,
        content,
        image_path_fn,
    );
    template.render()
}