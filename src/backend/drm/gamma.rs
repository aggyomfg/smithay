//! The hardware gamma ramp of a CRTC.
//!
//! A ramp is the lookup table the scanout hardware applies to every pixel after
//! composition, which is what a night-light client such as `wlsunset` adjusts. It is set
//! with [`DrmSurface::use_gamma`](super::DrmSurface::use_gamma), which stages it so that it
//! is applied as part of the *same* atomic commit as the mode and plane state, and its
//! required length is [`DrmSurface::gamma_size`](super::DrmSurface::gamma_size).

/// One entry of a `GAMMA_LUT` blob, laid out as the kernel's `struct drm_color_lut`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GammaLutEntry {
    /// The red channel's output value for this input.
    pub red: u16,
    /// The green channel's output value for this input.
    pub green: u16,
    /// The blue channel's output value for this input.
    pub blue: u16,
    reserved: u16,
}

impl GammaLutEntry {
    /// One entry with the reserved word zeroed, as the kernel requires.
    pub fn new(red: u16, green: u16, blue: u16) -> Self {
        GammaLutEntry {
            red,
            green,
            blue,
            reserved: 0,
        }
    }

    /// The entry as the kernel reads it: four native-endian `u16`, the last reserved.
    pub(crate) fn to_kernel_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        for (slot, value) in bytes
            .chunks_exact_mut(2)
            .zip([self.red, self.green, self.blue, self.reserved])
        {
            slot.copy_from_slice(&value.to_ne_bytes());
        }
        bytes
    }
}
