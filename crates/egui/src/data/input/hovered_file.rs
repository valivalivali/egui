/// A file about to be dropped into egui.
use crate::prelude::*;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HoveredFile {
    /// Set by the `egui-winit` backend.
    pub path: Option<String>,

    /// With the `eframe` web backend, this is set to the mime-type of the file (if available).
    pub mime: String,
}
