pub use lastfm::LastFM;
pub(crate) use storage::Storage;

pub mod cli;
pub mod storage;

mod domain;
mod lastfm;
mod utils;
