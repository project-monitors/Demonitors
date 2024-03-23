use anchor_lang::prelude::*;

use crate::error::ErrorCode;

const URI_MAX_LEN: usize = 150;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UriResource {
    pub len: u16,
    pub uri: [u8; URI_MAX_LEN],
}

impl Default for UriResource {
    fn default() -> Self {
        UriResource {
            len: 0,
            uri: [0u8; URI_MAX_LEN],
        }
    }
}

impl UriResource {

    pub fn validate(uri: &str) -> Result<UriResource> {
        let len = uri.len();
        if len > URI_MAX_LEN {
            return Err(error!(ErrorCode::StringTooLong));
        }

        let mut bytes = [0; URI_MAX_LEN];
        bytes[..len].copy_from_slice(uri.as_bytes());

        Ok(UriResource {
            len: len as u16,
            uri: bytes,
        })
    }

    pub const LEN: usize = 2 + URI_MAX_LEN;
}