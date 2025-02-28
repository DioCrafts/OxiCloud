// Copyright 2012 Bernhard Posselt nukeawhale@gmail.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use crate::ocp::Template;
use crate::appframework::http::Response;

/// Response for a normal template
pub struct TemplateResponse {
    template_name: String,
    params: HashMap<String, String>,
    render_as: String,
    app_name: String,
    response: Response,
}

impl TemplateResponse {
    /// Creates a new template response
    ///
    /// # Arguments
    ///
    /// * `app_name` - the name of the app to load the template from
    /// * `template_name` - the name of the template
    pub fn new(app_name: impl Into<String>, template_name: impl Into<String>) -> Self {
        TemplateResponse {
            template_name: template_name.into(),
            app_name: app_name.into(),
            params: HashMap::new(),
            render_as: "user".to_string(),
            response: Response::default(),
        }
    }

    /// Sets template parameters
    ///
    /// # Arguments
    ///
    /// * `params` - a HashMap with key => value structure which sets template variables
    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    /// Used for accessing the set parameters
    ///
    /// # Returns
    ///
    /// * the params
    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Used for accessing the name of the set template
    ///
    /// # Returns
    ///
    /// * the name of the used template
    pub fn get_template_name(&self) -> &str {
        &self.template_name
    }

    /// Sets the template page
    ///
    /// # Arguments
    ///
    /// * `render_as` - admin, user or blank. Admin also prints the admin
    ///                 settings header and footer, user renders the normal
    ///                 normal page including footer and header and blank
    ///                 just renders the plain template
    pub fn render_as(&mut self, render_as: impl Into<String>) {
        self.render_as = render_as.into();
    }

    /// Returns the set renderAs
    ///
    /// # Returns
    ///
    /// * the render_as value
    pub fn get_render_as(&self) -> &str {
        &self.render_as
    }

    /// Returns the rendered html
    ///
    /// # Returns
    ///
    /// * the rendered html
    pub fn render(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut template = Template::new(&self.app_name, &self.template_name, &self.render_as)?;

        for (key, value) in &self.params {
            template.assign(key, value);
        }

        template.fetch_page()
    }
}

impl From<TemplateResponse> for Response {
    fn from(template_response: TemplateResponse) -> Self {
        template_response.response
    }
}

impl AsRef<Response> for TemplateResponse {
    fn as_ref(&self) -> &Response {
        &self.response
    }
}

impl AsMut<Response> for TemplateResponse {
    fn as_mut(&mut self) -> &mut Response {
        &mut self.response
    }
}