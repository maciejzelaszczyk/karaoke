#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::inherent::Vec;
use scale_info::prelude::boxed::Box;
use scale_info::prelude::string::String;
use sp_inherents::{InherentData, InherentIdentifier, IsFatalError};
use sp_runtime::RuntimeString;
use sp_std::convert::From;

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"karaoke0";

pub type InherentType = Vec<u8>;

#[derive(Encode)]
pub enum InherentError {
    TooShort(InherentType),
    Other(RuntimeString)
}

impl IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		true
	}
}

pub struct InherentDataProvider {
    song_line: InherentType
}

impl InherentDataProvider {
    pub fn from_song_line(song_line: InherentType) -> Self {
        Self { song_line }
    }
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(
        &self,
        inherent_data: &mut InherentData,
    ) -> Result<(), sp_inherents::Error> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &InherentType::from(self.song_line.clone()))
    }

    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        mut error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        Some(Err(
            sp_inherents::Error::Application(Box::from(String::decode(&mut error).ok()?))
        ))
    }
}