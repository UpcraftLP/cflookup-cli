use std::sync::LazyLock;

pub mod cflookup;
mod net;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

static CRATE_VERSION: LazyLock<String, fn() -> String> =
    LazyLock::new(|| match env!("CARGO_PKG_VERSION") {
        "0.0.0" => built_info::GIT_COMMIT_HASH_SHORT
            .map(|hash| format!("git+{hash}"))
            .unwrap_or_else(|| "UNKNOWN".to_string()),
        ver => ver.to_string(),
    });

pub fn crate_version() -> &'static str {
    &CRATE_VERSION
}

pub fn user_agent() -> String {
    let mut user_agent = format!(
        "{name}/{version} ({repository})",
        name = built_info::PKG_NAME,
        version = crate_version(),
        repository = built_info::PKG_REPOSITORY
    );

    if let Some(commit_sha) = built_info::GIT_COMMIT_HASH_SHORT {
        user_agent = format!("{user_agent}(#{commit_sha})");
    }

    user_agent
}
