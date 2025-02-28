/**
 * ownCloud - App Framework
 *
 * @author Bernhard Posselt
 * @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use mockall::automock;
use mockall::predicate::*;
use std::rc::Rc;

// Interfaces definitions
#[automock]
pub trait IApi {
    fn get_app_name(&self) -> String;
}

pub struct TemplateResponse {
    api: Rc<dyn IApi>,
    template_name: String,
    params: HashMap<String, String>,
    render_as: Option<String>,
}

impl TemplateResponse {
    pub fn new(api: Rc<dyn IApi>, template_name: &str) -> Self {
        Self {
            api,
            template_name: template_name.to_string(),
            params: HashMap::new(),
            render_as: None,
        }
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
    }

    pub fn get_template_name(&self) -> &str {
        &self.template_name
    }

    pub fn render_as(&mut self, render: &str) {
        self.render_as = Some(render.to_string());
    }

    pub fn get_render_as(&self) -> Option<&str> {
        self.render_as.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct TemplateResponseTest {
        tpl: TemplateResponse,
        api: Rc<MockIApi>,
    }

    impl TemplateResponseTest {
        fn setup() -> Self {
            let mut api = MockIApi::new();
            api.expect_get_app_name()
                .returning(|| "app".to_string());
            
            let api_rc = Rc::new(api);
            let tpl = TemplateResponse::new(api_rc.clone(), "home");
            
            Self {
                tpl,
                api: api_rc,
            }
        }
    }

    #[test]
    fn test_set_params() {
        let mut test = TemplateResponseTest::setup();
        let mut params = HashMap::new();
        params.insert("hi".to_string(), "yo".to_string());
        
        test.tpl.set_params(params);
        
        let expected_params = {
            let mut map = HashMap::new();
            map.insert("hi".to_string(), "yo".to_string());
            map
        };
        
        assert_eq!(&expected_params, test.tpl.get_params());
    }

    #[test]
    fn test_get_template_name() {
        let test = TemplateResponseTest::setup();
        assert_eq!("home", test.tpl.get_template_name());
    }

    #[test]
    fn test_get_render_as() {
        let mut test = TemplateResponseTest::setup();
        let render = "myrender";
        test.tpl.render_as(render);
        assert_eq!(Some(render), test.tpl.get_render_as());
    }
}