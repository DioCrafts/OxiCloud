/*
 * Copyright (c) 2013 Christopher Schäpers <christopher@schaepers.it>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::avatar::Avatar;
use crate::image::Image;
use crate::user::get_user;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    async fn test_avatar() {
        let avatar = Avatar::new(get_user()).await;

        assert_eq!(None, avatar.get().await);

        let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT environment variable not set");
        let path = Path::new(&server_root).join("tests/data/testavatar.png");
        
        let mut expected = Image::from_path(&path).await.expect("Failed to load test avatar image");
        expected.resize(64);
        
        avatar.set(expected.data()).await.expect("Failed to set avatar");
        
        let avatar_data = avatar.get().await.expect("Failed to get avatar");
        assert_eq!(expected.data(), avatar_data.data());

        avatar.remove().await.expect("Failed to remove avatar");
        assert_eq!(None, avatar.get().await);
    }
}