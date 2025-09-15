pub mod cflookup;
mod net;

pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")"
);
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
