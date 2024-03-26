//! A collection of runtime interfaces (Substrate's concept for outsourcing computation to the host) for Aleph Zero
//! chain. For development / debugging purposes only.

#![cfg_attr(not(feature = "std"), no_std)]
#[deny(missing_docs)]

/// A runtime interface for getting the current time in nanoseconds.
#[cfg(feature = "now")]
#[sp_runtime_interface::runtime_interface]
pub trait Now {
    /// Get current UTC time in nanoseconds.
    fn now() -> Result<i64, ()> {
        Ok(chrono::prelude::Utc::now().timestamp_nanos_opt().unwrap())
    }
}
