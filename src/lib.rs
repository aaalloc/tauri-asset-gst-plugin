use gst::glib;

mod tauri_asset;
// https://github.com/sdroege/gst-plugin-rs/blob/main/net/aws/src/lib.rs

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    tauri_asset::register(plugin)
}

gst::plugin_define!(
    assettauri,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "The Plugin's License",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
