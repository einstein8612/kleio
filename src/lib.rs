use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::prelude::*;

// Sunday, June 23, 2024 Midnight UTC
const KLEIO_EPOCH_SINCE_UNIX_EPOCH: Duration = Duration::from_secs(1719100800);

#[cfg(not(test))]
pub fn now() -> SystemTime {
    SystemTime::now()
}

///
/// Generate a new key
/// 
/// This function generates a new key using the algorithm outlined in [`generate_id`] and then encodes it
/// using base62 encoding, to display it as a string. This function is meant to be used when generating
/// keys for external use like in urls.
///
pub fn generate_key() -> String {
    base62::encode(generate_id())
}

///
/// Generate a new id
///
/// This function generates a new key using the following algorithm:
/// 1. Generate a random 16-bit number
/// 2. Generate a 48-bit number representing the number of milliseconds since the Kleio Epoch
/// 3. Combine the two numbers into a 64-bit number
///
/// This guarantees that the key is unique, granted there are no collisions in the random number generation
/// at time x. There is a 50% possiblity of a collision if 110000 keys are generated at the same millisecond, assuming
/// a uniform distribution of random numbers.
///
/// For any reasonable number of keys generated, the probability of a collision is negligible.
///
pub fn generate_id() -> u64 {
    generate_id_from_rng(&mut thread_rng())
}

///
/// Generate a new id from rng
///
/// This function generates a new key using the following algorithm:
/// 1. Generate a random 16-bit number
/// 2. Generate a 48-bit number representing the number of milliseconds since the Kleio Epoch
/// 3. Combine the two numbers into a 64-bit number
///
/// This guarantees that the key is unique, granted there are no collisions in the random number generation
/// at time x. There is a 50% possiblity of a collision if 110000 keys are generated at the same millisecond, assuming
/// a uniform distribution of random numbers.
///
/// For any reasonable number of keys generated, the probability of a collision is negligible.
///
pub fn generate_id_from_rng(rng: &mut impl Rng) -> u64 {
    let entropy = (rng.next_u32() as u64) << 48;
    let ms_since_epoch = ((now().duration_since(UNIX_EPOCH + KLEIO_EPOCH_SINCE_UNIX_EPOCH).unwrap().as_millis() as u64) << 16) >> 16;

    entropy | ms_since_epoch
}

#[cfg(test)]
pub mod mock_time {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<Option<SystemTime>> = RefCell::new(None);
    }

    pub fn now() -> SystemTime {
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(SystemTime::now)
        })
    }

    pub fn set_mock_time(time: SystemTime) {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = Some(time));
    }

    pub fn clear_mock_time() {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = None);
    }
}

#[cfg(test)]
pub use mock_time::now;

#[cfg(test)]
mod tests {
    use mock_time::set_mock_time;
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn generate_id_from_rng_correctly_uses_rng_and_time() {
        // Set mock random to 1
        let mut step_rng = StepRng::new(0b1011001110001111, 1);
        // Set mock time to 0b1010101 milliseconds after the epoch
        set_mock_time(UNIX_EPOCH + KLEIO_EPOCH_SINCE_UNIX_EPOCH + Duration::from_millis(0b010011000111000111100001111100000111111000000111));

        let id = generate_id_from_rng(&mut step_rng);
        assert_eq!(id, 0b1011001110001111010011000111000111100001111100000111111000000111);
    }

    #[test]
    fn generate_id_produces_unique_ids() {
        let (id1, id2) = (generate_id(), generate_id());

        assert_ne!(id1, id2);
    }

    #[test]
    fn generate_key_produces_unique_ids() {
        let (id1, id2) = (generate_key(), generate_key());

        assert_ne!(id1, id2);
    }
}