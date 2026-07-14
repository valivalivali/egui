/// A file dropped into egui.
use crate::prelude::*;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DroppedFile {
    /// Set by the `egui-winit` backend.
    pub path: Option<String>,

    /// Name of the file. Set by the `eframe` web backend.
    pub name: String,

    /// With the `eframe` web backend, this is set to the mime-type of the file (if available).
    pub mime: String,

    /// Set by the `eframe` web backend.
    pub last_modified: Option<u64>,

    /// Set by the `eframe` web backend.
    pub bytes: Option<alloc::sync::Arc<[u8]>>,
}
