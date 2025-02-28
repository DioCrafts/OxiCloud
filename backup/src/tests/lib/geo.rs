/*
 * Copyright (c) 2012 Lukas Reschke <lukas@statuscode.ch>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::geo::OcGeo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "medium"]
    fn test_timezone() {
        let result = OcGeo::timezone(3.0, 3.0);
        let expected = "Africa/Porto-Novo";
        assert_eq!(expected, result);

        let result = OcGeo::timezone(-3.0, -3333.0);
        let expected = "Pacific/Enderbury";
        assert_eq!(expected, result);
    }
}