//! This crate provides access and control for Lovense devices

extern crate serial;

// Only provide outside access via get_devices. Everything should be generated
// from the output there.
pub use ::lovesense::{LovesenseDevice, LovesenseProtocol};

mod lovesense;
