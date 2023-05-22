use bincode::config::{BigEndian, Configuration, Fixint};

pub type BincodeConfig = Configuration<BigEndian, Fixint>;
pub type Lazy<T> = once_cell::sync::Lazy<T>;
