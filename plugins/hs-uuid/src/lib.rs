
// linting directives (see https://doc.rust-lang.org/rustc/lints/index.html)
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

#[macro_use] extern crate cfg_if;


use std::fmt::{self, Display};

#[cfg(any(feature="v1_uuid", feature="v5_uuid"))]
use uuid::Uuid;

// Uuid spec if of 32 characters long, so this number must not exceed 32
const UUID_LEN: usize = 16;

/// UUID specific to High Steel's usage
#[derive(Debug)]
pub struct HsUuid {
    uuid: [u8; UUID_LEN]
}

impl HsUuid {
    /// Generate a new UUID
    /// 
    /// This generation is feature driven
    /// features:
    ///     - `v1_uuid': generate using [Uuid::now_v1](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.now_v1)
    ///     - `v5_uuid': generate using [Uuid::new_v5](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v5)
    ///     - `db_uuid': generate UUID from next available UUID in the database
    /// 
    /// if more than one of these features is enabled, precedence is
    ///     1) `v5_uuid'
    ///     2) `v1_uuid'
    ///     3) `db_uuid'
    /// 
    /// panics if no UUID feature is enabled
    pub fn new() -> Self {
        cfg_if! {
            if #[cfg(feature = "v5_uuid")] {
                Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"high.net").into()
            }

            else if #[cfg(feature = "v1_uuid")] {
                Uuid::now_v1(&[0; 6]).into()
            }

            else if #[cfg(feature = "db_uuid")] {
                todo!("HsUuid init from database not implemented")
                // something like
                // let mut uuid = db::get_last_uuid();
                // uuid.next();
                // uuid
            }

            else {
                panic!("No UUID generation feature enabled")
            }
        }
    }

    /// Get next valid alphanumeric UUID
    pub fn next(&mut self) {
        self.increment(UUID_LEN-1);
    }

    /// increments given index of UUID
    /// 
    /// - numeric values increment numerically
    /// - alphabetical value increment to the next letter
    /// - 9 increments to a
    /// - z wraps around to 0 and increments next order digit
    /// 
    /// panics if max UUID is reached
    fn increment(&mut self, index: usize) {
        match self.uuid[index] {
            // jump to lowercase alphabet
            b'9' => self.uuid[index] = b'a',

            // wrap around to start
            b'z' => {
                self.uuid[index] = b'0';

                // handle max UUID reached (cannot increment index = -1)
                // TODO: return error
                if index == 0 {
                    panic!(r#"
Max {UUID_LEN}-digit UUID reached. Cannot increment any further.
Either
    a) implement more digits
    b) implement or clean database and start from 0*

    *will require more development to always pick imported digit
        as it will now not be the highest UUID
                    "#)
                }

                // increment next order digit
                self.increment(index-1);
            },
            _    => self.uuid[index] += 1,
        }
    }
}

impl Display for HsUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe { std::str::from_utf8_unchecked(&self.uuid) })
    }
}

#[cfg(any(feature="v1_uuid", feature="v5_uuid"))]
impl From<uuid::Uuid> for HsUuid {
    fn from(full_uuid: Uuid) -> Self {
        let uuid = full_uuid
            .simple()
            .to_string()
            .bytes()
                // we only want the first {UUID_LEN} digits
                .take(UUID_LEN)
                .collect::<Vec<u8>>()
                .try_into()
                    // will error if array and Vec lengths differ, which it should not unless UUID_LEN <= 32
                    //  - take(UUID_LEN) ensures that we have UUID_LEN or less characters
                    //  - take(UUID_LEN) should not yield less than UUID_LE
                    //      due to Uuid spec stating that Uuid should be 32 characters long
                    .expect(
                        &format!("failed to coerce Vec<u8> into [u8; {}]", UUID_LEN)
                    );

        Self { uuid }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_preset_uuid_length() {
        assert!(crate::UUID_LEN <= 32);
    }
}
