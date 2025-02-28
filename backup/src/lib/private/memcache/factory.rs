// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

pub mod oc {
    pub mod memcache {
        use std::rc::Rc;

        /// Cache trait that all memory cache implementations must implement
        pub trait Cache {
            fn new(prefix: &str) -> Self where Self: Sized;
        }

        pub struct XCache {
            prefix: String,
        }

        impl XCache {
            pub fn is_available() -> bool {
                // Implementation would check if XCache is available
                false
            }
        }

        impl Cache for XCache {
            fn new(prefix: &str) -> Self {
                XCache {
                    prefix: prefix.to_string(),
                }
            }
        }

        pub struct APCu {
            prefix: String,
        }

        impl APCu {
            pub fn is_available() -> bool {
                // Implementation would check if APCu is available
                false
            }
        }

        impl Cache for APCu {
            fn new(prefix: &str) -> Self {
                APCu {
                    prefix: prefix.to_string(),
                }
            }
        }

        pub struct APC {
            prefix: String,
        }

        impl APC {
            pub fn is_available() -> bool {
                // Implementation would check if APC is available
                false
            }
        }

        impl Cache for APC {
            fn new(prefix: &str) -> Self {
                APC {
                    prefix: prefix.to_string(),
                }
            }
        }

        pub struct Memcached {
            prefix: String,
        }

        impl Memcached {
            pub fn is_available() -> bool {
                // Implementation would check if Memcached is available
                false
            }
        }

        impl Cache for Memcached {
            fn new(prefix: &str) -> Self {
                Memcached {
                    prefix: prefix.to_string(),
                }
            }
        }

        pub struct Factory;

        impl Factory {
            /// Get a cache instance, will return None if no backend is available
            ///
            /// # Arguments
            ///
            /// * `prefix` - Prefix for cache keys
            ///
            /// # Returns
            ///
            /// Option containing a boxed Cache implementation or None if unavailable
            pub fn create(&self, prefix: &str) -> Option<Rc<dyn Cache>> {
                if XCache::is_available() {
                    Some(Rc::new(XCache::new(prefix)))
                } else if APCu::is_available() {
                    Some(Rc::new(APCu::new(prefix)))
                } else if APC::is_available() {
                    Some(Rc::new(APC::new(prefix)))
                } else if Memcached::is_available() {
                    Some(Rc::new(Memcached::new(prefix)))
                } else {
                    None
                }
            }

            /// Check if there is a memcache backend available
            ///
            /// # Returns
            ///
            /// `true` if a memcache backend is available, `false` otherwise
            pub fn is_available(&self) -> bool {
                XCache::is_available() || APCu::is_available() || APC::is_available() || Memcached::is_available()
            }

            /// Get a in-server cache instance, will return None if no backend is available
            ///
            /// # Arguments
            ///
            /// * `prefix` - Prefix for cache keys
            ///
            /// # Returns
            ///
            /// Option containing a boxed Cache implementation or None if unavailable
            pub fn create_low_latency(prefix: &str) -> Option<Rc<dyn Cache>> {
                if XCache::is_available() {
                    Some(Rc::new(XCache::new(prefix)))
                } else if APCu::is_available() {
                    Some(Rc::new(APCu::new(prefix)))
                } else if APC::is_available() {
                    Some(Rc::new(APC::new(prefix)))
                } else {
                    None
                }
            }

            /// Check if there is a in-server backend available
            ///
            /// # Returns
            ///
            /// `true` if an in-server backend is available, `false` otherwise
            pub fn is_available_low_latency() -> bool {
                XCache::is_available() || APCu::is_available() || APC::is_available()
            }
        }
    }
}