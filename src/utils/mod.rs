/// Functions for quick validation
pub mod validation;

pub mod base58;

#[cfg(feature = "w3c")]
pub mod base64;

#[cfg(feature = "w3c")]
pub mod encoded_object;

pub mod hash;

pub mod query;

pub mod msg_pack;

#[macro_use]
pub mod macros;
