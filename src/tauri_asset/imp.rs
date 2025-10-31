// typedef struct _GstAssets
// {
//     GstBin parent_instance;
//     GstElement *internal_src;
// } GstTauriAsset;

// typedef struct _GstAssetsClass
// {
//     GstBinClass parent_class;
// } GstTauriAssetClass;

use gst::ffi::gst_uri_get_location;
use gst::prelude::{ElementExt, GhostPadExt, PadExt};
use gst::subclass::prelude::{BinImpl, ElementImpl, ObjectImpl, URIHandlerImpl};
use gst::subclass::prelude::{GstObjectImpl, ObjectSubclass};
use gst::{GhostPad, PadDirection};

const ASSET_URI_SCHEME: &str = "asset";

#[derive(Default)]
pub struct TauriAsset {
    internal_src: Option<gst::Element>,
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
impl ObjectImpl for TauriAsset {}
impl BinImpl for TauriAsset {}
impl ElementImpl for TauriAsset {}
// impl ElementExt for TauriAsset {}

impl URIHandlerImpl for TauriAsset {
    const URI_TYPE: gst::URIType = gst::URIType::Src;

    fn protocols() -> &'static [&'static str] {
        &[ASSET_URI_SCHEME]
    }

    fn uri(&self) -> Option<String> {
        todo!()
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

        // now location is like: /path/to/asset
        let internal_src = gst::ElementFactory::make("filesrc")
            .name("tauri-asset-filesrc")
            .property("location", location)
            .build()
            .ok();

        let srcpad = internal_src.as_ref().and_then(|src| src.static_pad("src"));
        let ghostpad = GhostPad::new(PadDirection::Src);
        ghostpad.set_target(Some(&srcpad.unwrap()));
        ghostpad.set_active(true);
        self.add_pad(&ghostpad).expect("Failed to add ghost pad");

        // free srcpad
        Ok(())
    }
    // static gboolean gst_asset_tauriuri_set_uri(GstURIHandler *handler,
    //                                            const gchar *uri, GError **error)
    // {
    //     GstTauriAsset *self = GST_ASSETS(handler);

    //     gchar *path = gst_uri_get_location(uri);
    //     if (!path)
    //     {
    //         g_set_error(error, GST_URI_ERROR, GST_URI_ERROR_BAD_URI,
    //                     "Could not get location from URI: %s", uri);
    //         return FALSE;
    //     }
    //     else if (!g_file_test(path, G_FILE_TEST_EXISTS))
    //     {
    //         g_set_error(error, GST_URI_ERROR, G_FILE_ERROR_NOENT,
    //                     "File does not exist: %s", path);
    //         g_free(path);
    //         return FALSE;
    //     }

    //     // Build internal pipeline (filesrc)
    //     if (self->internal_src)
    //         gst_bin_remove(GST_BIN(self), self->internal_src);

    //     self->internal_src = gst_element_factory_make("filesrc", NULL);
    //     g_object_set(self->internal_src, "location", path, NULL);
    //     gst_bin_add(GST_BIN(self), self->internal_src);

    //     // Expose src pad
    //     GstPad *srcpad = gst_element_get_static_pad(self->internal_src, "src");
    //     GstPad *ghostpad = gst_ghost_pad_new("src", srcpad);
    //     gst_pad_set_active(ghostpad, TRUE);
    //     gst_element_add_pad(GST_ELEMENT(self), ghostpad);
    //     gst_object_unref(srcpad);

    //     g_free(path);
    //     return TRUE;
    // }
}
