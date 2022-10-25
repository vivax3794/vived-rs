//! Who doesn't like colors?

use serde::{Deserialize, Serialize};

// the guilded api uses colors using u32

/// A color is a simple rgb tuple
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub struct Color(pub u8, pub u8, pub u8);

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self(r, g, b)
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(Color(r, g, b): Color) -> Self {
        (r, g, b)
    }
}

impl From<u32> for Color {
    // This is safe because of the bit shifting and bitwise and operations
    // We would in a perfect world have used a u24 (3 * 8), but we don't have that
    // so we use a u32 and just ignore the first 8 bits
    // You could argue we should error if those bits are not 0, but we don't
    #[allow(clippy::expect_used)]
    fn from(v: u32) -> Self {
        Self(
            ((v >> 16) & 0xFF).try_into().expect("u32 to u8 failed"),
            ((v >> 8) & 0xFF).try_into().expect("u32 to u8 failed"),
            (v & 0xFF).try_into().expect("u32 to u8 failed"),
        )
    }
}

impl From<Color> for u32 {
    // This is safe because of the bit shifting and bitwise and operations
    fn from(Color(r, g, b): Color) -> Self {
        u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b)
    }
}

impl Color {
    /// Convert a hex string to a color
    /// Might or might not contain a leading `#`
    /// 
    /// # Errors
    /// If the string is not a valid hex color
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        // Remove leading "#"
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        // check if length is valid
        if hex.len() != 6 {
            return Err("Invalid hex color length".to_owned());
        }

        // Split hex into rgb
        let r = &hex[0..2];
        let g = &hex[2..4];
        let b = &hex[4..6];

        // Convert hex to u8
        let r = r.parse().map_err(|_| "Invalid hex color")?;
        let b = b.parse().map_err(|_| "Invalid hex color")?;
        let g = g.parse().map_err(|_| "Invalid hex color")?;

        Ok(Self(r, g, b))
    }

    /// Convert this color to hex
    #[must_use]
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}