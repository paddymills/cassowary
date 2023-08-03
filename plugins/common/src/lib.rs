
//! Cassowary plugin interface

pub trait CassowaryPlugin {
    type Error;

    fn on_event() -> Result<(), Self::Error>;
}
