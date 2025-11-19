// SPDX-License-Identifier: GPL-3.0-only

use libdisplay_info::info::{ColorPrimaries, HdrStaticMetadata, Info, SupportedSignalColorimetry};

/// Parsed HDR and colorimetry characteristics derived from EDID metadata.
#[derive(Debug, Clone, Copy)]
pub struct OutputColorCharacteristics {
    pub hdr_static_metadata: HdrStaticMetadata,
    pub primaries: ColorPrimaries,
    pub supported_colorimetry: SupportedSignalColorimetry,
    pub default_gamma: Option<f32>,
}

impl OutputColorCharacteristics {
    pub fn from_info(info: &Info) -> Self {
        Self {
            hdr_static_metadata: info.hdr_static_metadata(),
            primaries: info.default_color_primaries(),
            supported_colorimetry: info.supported_signal_colorimetry(),
            default_gamma: info.default_gamma(),
        }
    }

    pub fn supports_hdr(&self) -> bool {
        let eotf_support = &self.hdr_static_metadata;
        eotf_support.pq || eotf_support.hlg || eotf_support.traditional_hdr
    }
}
