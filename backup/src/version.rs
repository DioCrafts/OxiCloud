// version.rs

// We only can count up. The 4. digit is only for the internal patchlevel to trigger DB upgrades 
// between betas, final and RCs. This is _not_ the public version number. 
// Reset minor/patchlevel when updating major/minor version number.
pub const OC_VERSION: [u32; 4] = [6, 0, 0, 6];

// The human readable string
pub const OC_VERSION_STRING: &str = "6.0 beta 4";

// The ownCloud edition
pub const OC_EDITION: &str = "";

// The ownCloud channel
pub const OC_CHANNEL: &str = "git";

// The build number
pub const OC_BUILD: &str = "";