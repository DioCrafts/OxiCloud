// ownCloud
//
// @author Bart Visscher
// @copyright 2013 Bart Visscher <bartv@thisnet.nl>
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

//! Public interface of ownCloud for apps to use.
//! Image module

// Public namespace for all classes that are considered public.
// This means that they should be used by apps instead of the internal ownCloud classes
pub mod ocp {
    use crate::oc_image::OCImage;

    /// This struct provides functions to handle images
    ///
    /// A public wrapper around the internal OCImage implementation
    pub struct Image {
        inner: OCImage,
    }

    impl Image {
        /// Creates a new Image instance
        pub fn new() -> Self {
            Self {
                inner: OCImage::new(),
            }
        }
    }

    impl Default for Image {
        fn default() -> Self {
            Self::new()
        }
    }

    impl std::ops::Deref for Image {
        type Target = OCImage;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl std::ops::DerefMut for Image {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl From<OCImage> for Image {
        fn from(inner: OCImage) -> Self {
            Self { inner }
        }
    }
}