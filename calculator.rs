use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
extern crate integer_sqrt;
use crate::calculator::integer_sqrt::IntegerSquareRoot;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as calculator {
		FinalResult: u64;
	}
}

decl_module! {
	pub struct Module <T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default; //This is default deposit_event definition and it is used only if you are using events in your module i.e. decl_events

		fn add(origin, a: u64, b: u64) -> Result {
			// We use this to make sure that this is a signed message
			// and that a user will be charged a transaction fee.
			let sender = ensure_signed(origin)?;
			let result = a.checked_add(b).ok_or("Numbers are too large to store in a u64")?;
			<FinalResult<T>>::put(result);

			Self::deposit_event(RawEvent::Addition(sender, a, b, result));

			Ok(())
		}

        fn sub(origin, a: u64, b: u64) -> Result {
			// We use this to make sure that this is a signed message
			// and that a user will be charged a transaction fee.
			let sender = ensure_signed(origin)?;
			let result = a.checked_sub(b).ok_or("Numbers are too large to store in a u64")?;
			<FinalResult<T>>::put(result);

			Self::deposit_event(RawEvent::Addition(sender, a, b, result));

			Ok(())
		}

        fn mul(origin, a: u64, b: u64) -> Result {
			// We use this to make sure that this is a signed message
			// and that a user will be charged a transaction fee.
			let sender = ensure_signed(origin)?;
			let result = a.checked_mul(b).ok_or("Numbers are too large to store in a u64")?;
			<FinalResult<T>>::put(result);

			Self::deposit_event(RawEvent::Multiplication(sender, a, b, result));

			Ok(())
		}

        fn div(origin, a: u64, b: u64) -> Result {
			// We use this to make sure that this is a signed message
			// and that a user will be charged a transaction fee.
			let sender = ensure_signed(origin)?;
			let result = a.checked_div(b).ok_or("Numbers are too large to store in a u64")?;
			<FinalResult<T>>::put(result);

			Self::deposit_event(RawEvent::Division(sender, a, b, result));

			Ok(())
		}

		fn sqrt(origin, a: u64) -> Result {
			// We use this to make sure that this is a signed message
			// and that a user will be charged a transaction fee.
			let sender = ensure_signed(origin)?;
			let result = a.integer_sqrt().ok_or("Numbers are too large to store in a u64")?;
			<FinalResult<T>>::put(result);

			Self::deposit_event(RawEvent::Squareroot(sender, a, result));

			Ok(())
		}


	}
}

decl_event! (
	pub enum Event<T> where <T as system::Trait>::AccountId {
		Addition(AccountId, u64, u64, u64),
        Substraction(AccountId, u64, u64, u64),
        Multiplication(AccountId, u64, u64, u64),
        Division(AccountId, u64, u64, u64),
		Squareroot(AccountId, u64, u64),
	}
);
