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
 *
 */

use std::collections::HashMap;
use crate::appframework::dependency_injection::di_container::DIContainer;
use crate::appframework::interfaces::app_container::IAppContainer;

/**
 * Entry point for every request in your app. You can consider this as your
 * public static void main() method
 *
 * Handles all the dependency injection, controllers and output flow
 */
pub struct App;

impl App {
    /**
     * Shortcut for calling a controller method and printing the result
     * @param controller_name the name of the controller under which it is
     *                        stored in the DI container
     * @param method_name the method that you want to call
     * @param container an instance of a pimple container.
     */
    pub fn main(controller_name: &str, method_name: &str, container: &dyn IAppContainer) -> Result<(), Box<dyn std::error::Error>> {
        let controller = container.get(controller_name)?;
        
        // initialize the dispatcher and run all the middleware before the controller
        let dispatcher = container.get("Dispatcher")?;
        
        let (http_headers, response_headers, output) = 
            dispatcher.dispatch(controller, method_name)?;
        
        if let Some(http_header) = http_headers {
            // In real implementation, this would interact with a web framework
            // to set HTTP headers
            println!("{}", http_header);
        }
        
        for (name, value) in response_headers {
            // Same here, setting headers in a web framework
            println!("{}: {}", name, value);
        }
        
        if let Some(out) = output {
            // Set content length and output in a real web framework
            println!("Content-Length: {}", out.len());
            println!("{}", out);
        }
        
        Ok(())
    }

    /**
     * Shortcut for calling a controller method and printing the result.
     * Similar to App:main except that no headers will be sent.
     * This should be used for example when registering sections via
     * \OC\AppFramework\Core\API::registerAdmin()
     *
     * @param controller_name the name of the controller under which it is
     *                        stored in the DI container
     * @param method_name the method that you want to call
     * @param url_params an array with variables extracted from the routes
     * @param container an instance of a pimple container.
     */
    pub fn part(
        controller_name: &str, 
        method_name: &str, 
        url_params: HashMap<String, String>, 
        container: &mut DIContainer
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        container.set("url_params", url_params);
        let controller = container.get(controller_name)?;
        
        let dispatcher = container.get("Dispatcher")?;
        
        let (_, _, output) = dispatcher.dispatch(controller, method_name)?;
        Ok(output)
    }
}