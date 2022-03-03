#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use codec::FullCodec;
	use frame_support::dispatch::fmt::Debug;
	use frame_support::sp_std::result;
	use frame_support::inherent::Vec;

	use karaoke::{InherentError, INHERENT_IDENTIFIER, InherentType};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Line: Clone + Debug + Default + From<Vec<u8>> + FullCodec + PartialEq + TypeInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn song_line)]
	pub type SongLine<T: Config> = StorageValue<_, T::Line, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SongLineRead(T::Line, T::AccountId)
	}

	#[pallet::error]
	pub enum Error<T> {
		LineAlreadyUpdated
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn set(origin: OriginFor<T>, song_line: T::Line) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			match SongLine::<T>::get() {
				None => <SongLine<T>>::put(song_line),
				Some(_) => Err(Error::<T>::LineAlreadyUpdated)?
			}

			Ok(())
		}	
	}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = InherentError;
		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let inherent_data = data
				.get_data::<InherentType>(&INHERENT_IDENTIFIER)
				.expect("Song line inherent data not correctly encoded")
				.expect("Song line inherent data must be provided");
			let data = T::Line::from(inherent_data);

			Some(Call::set { song_line: data })
		}

		fn check_inherent(
			call: &Self::Call,
			data: &InherentData,
		) -> result::Result<(), Self::Error> {

			let data = data
				.get_data::<InherentType>(&INHERENT_IDENTIFIER)
				.expect("Timestamp inherent data not correctly encoded")
				.expect("Timestamp inherent data must be provided");

			if data.len() == 0 {
				Err(InherentError::TooShort(data))
			} else {
				Ok(())
			}
		}

		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::set { .. })
		}
	}
}
