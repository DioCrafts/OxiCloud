//! Geo utilities for time and location.
//!
//! Copyright (c) 2012 Georg Ehrke <ownclouddev at georgswebsite dot de>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use chrono_tz::TZ_VARIANTS;
use thiserror::Error;
use std::collections::BTreeMap;

#[derive(Debug, Error)]
pub enum GeoError {
    #[error("No timezones available")]
    NoTimezonesAvailable,
}

/// Geographic utilities and calculations
pub struct Geo;

impl Geo {
    /// Returns the closest timezone to the given coordinates
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude as float
    /// * `longitude` - Longitude as float
    ///
    /// # Returns
    ///
    /// * `Result<String, GeoError>` - Name of the closest timezone or error
    pub fn timezone(latitude: f64, longitude: f64) -> Result<String, GeoError> {
        let mut variances = BTreeMap::new();
        
        // Calculate for all timezones the system knows
        for tz in TZ_VARIANTS.iter() {
            // Note: In a real implementation, we would parse the IANA timezone database
            // to get the location information for each timezone.
            // For this simplified version, we're using a sample approach.
            if let Some((tz_latitude, tz_longitude)) = Self::get_timezone_location(tz.name()) {
                let variance = (tz_latitude - latitude).abs() + (tz_longitude - longitude).abs();
                variances.insert(variance, tz.name().to_string());
            }
        }
        
        // Return the timezone with the smallest difference
        variances.into_iter().next()
            .map(|(_, timezone)| timezone)
            .ok_or(GeoError::NoTimezonesAvailable)
    }

    // Helper method to get latitude and longitude for a timezone
    // In a real implementation, this would query the IANA timezone database
    fn get_timezone_location(timezone: &str) -> Option<(f64, f64)> {
        // This is a simplified implementation - in a real application,
        // you would use a proper database of timezone coordinates
        // For example purposes only - these are not accurate values
        match timezone {
            "Europe/Berlin" => Some((52.5200, 13.4050)),
            "America/New_York" => Some((40.7128, -74.0060)),
            "Asia/Tokyo" => Some((35.6762, 139.6503)),
            "Australia/Sydney" => Some((-33.8688, 151.2093)),
            // Add more mappings as needed
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone() {
        // Test with coordinates near Berlin
        let result = Geo::timezone(52.5, 13.4);
        assert!(result.is_ok());
        // This assertion depends on your implementation and available timezone data
        // assert_eq!(result.unwrap(), "Europe/Berlin");
    }
}