//! Copyright (c) 2013 Bjoern Schiessle <schiessle@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use crate::encryption::helper;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test for the striping of partial file extensions
    #[test]
    fn test_strip_partial_file_extension() {
        let part_filename = "testfile.txt.part";
        let filename = "testfile.txt";

        assert!(helper::is_partial_file_path(part_filename));
        assert_eq!("testfile.txt", helper::strip_partial_file_extension(part_filename));

        assert!(!helper::is_partial_file_path(filename));
        assert_eq!("testfile.txt", helper::strip_partial_file_extension(filename));
    }

    /// Test for the striping of partial file extensions with transfer ID
    #[test]
    fn test_strip_partial_file_extension_with_transfer_id_path() {
        let part_filename = "testfile.txt.ocTransferId643653835.part";
        let filename = "testfile.txt";

        assert!(helper::is_partial_file_path(part_filename));
        assert_eq!("testfile.txt", helper::strip_partial_file_extension(part_filename));

        assert!(!helper::is_partial_file_path(filename));
        assert_eq!("testfile.txt", helper::strip_partial_file_extension(filename));
    }
}