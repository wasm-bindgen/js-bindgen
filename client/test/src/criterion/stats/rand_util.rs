use core::cell::RefCell;

use oorandom::Rand64;

use crate::utils::LazyCell;
use crate::{SystemTime, UNIX_EPOCH};

pub type Rng = Rand64;

#[cfg_attr(target_feature = "atomics", thread_local)]
static SEED_RAND: LazyCell<RefCell<Rand64>> = LazyCell::new(|| {
	RefCell::new(Rand64::new(
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards")
			.as_millis(),
	))
});

pub fn new_rng() -> Rng {
	let mut r = SEED_RAND.borrow_mut();
	let seed = (u128::from(r.rand_u64()) << 64) | u128::from(r.rand_u64());
	Rand64::new(seed)
}
