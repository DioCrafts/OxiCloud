// Copyright (c) 2012 Bernhard Posselt <nukeawhale@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use crate::appframework::utility::method_annotation_reader::MethodAnnotationReader;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    struct MethodAnnotationReaderTest;

    impl MethodAnnotationReaderTest {
        /// Annotation
        fn test_read_annotation(&self) {}

        /// Annotation
        /// param test
        fn test_read_annotation_no_lowercase(&self) {}
    }

    #[test]
    fn test_read_annotation() -> Result<(), Box<dyn Error>> {
        let reader = MethodAnnotationReader::new(
            "MethodAnnotationReaderTest",
            "test_read_annotation",
        )?;

        assert!(reader.has_annotation("Annotation"));
        Ok(())
    }

    #[test]
    fn test_read_annotation_no_lowercase() -> Result<(), Box<dyn Error>> {
        let reader = MethodAnnotationReader::new(
            "MethodAnnotationReaderTest",
            "test_read_annotation_no_lowercase",
        )?;

        assert!(reader.has_annotation("Annotation"));
        assert!(!reader.has_annotation("param"));
        Ok(())
    }
}