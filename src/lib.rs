use std::sync::LazyLock;

pub mod cflookup;
mod net;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

static CRATE_VERSION: LazyLock<String, fn() -> String> = LazyLock::new(|| {
    if let Some(version) = option_env!("VERSION") {
        return version.to_string();
    }

    // otherwise: use git commit
    if let Some(hash) = built_info::GIT_COMMIT_HASH_SHORT {
        return format!("git+{hash}");
    }

    // finally, use build time
    let timestamp = built::util::strptime(built_info::BUILT_TIME_UTC);
    format!(
        "CUSTOM+{build_time}",
        build_time = timestamp.format("%y.%m%d.%H%M")
    )
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
