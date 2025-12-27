use gst::glib::subclass::types::ObjectSubclassExt;
use gst::prelude::{ElementExt, GstBinExt, PadExt};
use gst::subclass::prelude::{BinImpl, ElementImpl, ObjectImpl, URIHandlerImpl};
use gst::subclass::prelude::{GstObjectImpl, ObjectSubclass};
use gst::{glib, GhostPad};
use gstreamer::glib::object::ObjectExt;
use gstreamer::glib::subclass::object::ObjectImplExt;
use gstreamer_base::gst;

const ASSET_URI_SCHEME: &str = "asset";

#[derive(Default)]
pub struct TauriAsset {
    pub filesrc: std::sync::OnceLock<gst::Element>,
}

impl TauriAsset {
    pub fn new() -> Self {
        Self::default()
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TauriAsset {
    const NAME: &'static str = "GstTauriAsset";
    type Type = super::TauriAsset;
    type ParentType = gst::Bin;
    type Interfaces = (gst::URIHandler,);
}

impl GstObjectImpl for TauriAsset {}
impl ObjectImpl for TauriAsset {
    fn constructed(&self) {
        self.parent_constructed();

        let element = self.obj();
        let filesrc = gst::ElementFactory::make("filesrc")
            .build()
            .expect("Failed to create filesrc");

        element
            .add(&filesrc)
            .expect("Failed to add filesrc to element");

        let srcpad = filesrc
            .static_pad("src")
            .expect("Failed to get src pad from filesrc");
        let ghostpad = GhostPad::with_target(&srcpad).expect("Failed to create ghost pad");
        element
            .add_pad(&ghostpad)
            .expect("Failed to add ghost pad to element");
        ghostpad
            .set_active(true)
            .expect("Failed to activate ghost pad");

        self.filesrc.set(filesrc).expect("Failed to set filesrc");
    }
}
impl BinImpl for TauriAsset {}
impl ElementImpl for TauriAsset {}

impl URIHandlerImpl for TauriAsset {
    const URI_TYPE: gst::URIType = gst::URIType::Src;

    fn protocols() -> &'static [&'static str] {
        &[ASSET_URI_SCHEME]
    }

    fn uri(&self) -> Option<String> {
        None
    }

    fn set_uri(&self, uri: &str) -> Result<(), glib::Error> {
        // uri is like: asset://path/to/asset or asset://localhost/path/to/asset
        let sep = format!("{}://", ASSET_URI_SCHEME);
        let mut split = uri.split(sep.as_str());
        let location = split.nth(1).ok_or_else(|| {
            glib::Error::new(gst::URIError::BadUri, "Could not get location from URI")
        })?;

        // directly having full path after asset:// or having localhost
        let location = location.strip_prefix("localhost").unwrap_or(location);

        // Uri could be percent-encoded
        let location = percent_encoding::percent_decode_str(location)
            .decode_utf8()
            .map_err(|_| {
                glib::Error::new(
                    gst::URIError::BadUri,
                    "Could not decode percent-encoded URI",
                )
            })?
            .to_string();

        self.filesrc
            .get()
            .expect("filesrc not initialized")
            .set_property("location", &location);

        Ok(())
    }
}
