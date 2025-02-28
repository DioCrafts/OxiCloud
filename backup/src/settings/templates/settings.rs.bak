/*
 * Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;

/// Render settings forms template
pub fn render_settings_template(forms: &[String]) -> String {
    forms.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_settings_template() {
        let forms = vec![
            "<form>Test1</form>".to_string(),
            "<form>Test2</form>".to_string(),
        ];
        let result = render_settings_template(&forms);
        assert_eq!(result, "<form>Test1</form><form>Test2</form>");
    }
}