//! Card aspect ratios: exact integer-divisible native canvases, integer 3x upscale.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShareRatio {
    #[default]
    Landscape16x9,
    Story9x16,
    Classic4x3,
    Portrait3x4,
}

impl ShareRatio {
    pub fn all() -> &'static [ShareRatio] {
        &[
            Self::Landscape16x9,
            Self::Story9x16,
            Self::Classic4x3,
            Self::Portrait3x4,
        ]
    }

    pub fn parse(s: &str) -> Option<ShareRatio> {
        match s.trim().to_ascii_lowercase().as_str() {
            "16:9" | "landscape" => Some(Self::Landscape16x9),
            "9:16" | "story" => Some(Self::Story9x16),
            "4:3" | "classic" => Some(Self::Classic4x3),
            "3:4" | "portrait" => Some(Self::Portrait3x4),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Landscape16x9 => "16:9",
            Self::Story9x16 => "9:16",
            Self::Classic4x3 => "4:3",
            Self::Portrait3x4 => "3:4",
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Self::Landscape16x9 => "16x9",
            Self::Story9x16 => "9x16",
            Self::Classic4x3 => "4x3",
            Self::Portrait3x4 => "3x4",
        }
    }

    /// Native low-res authoring canvas; the final PNG is exactly `native * scale`.
    pub fn native(&self) -> (usize, usize) {
        match self {
            Self::Landscape16x9 => (384, 216),
            Self::Story9x16 => (216, 384),
            Self::Classic4x3 => (288, 216),
            Self::Portrait3x4 => (216, 288),
        }
    }

    pub fn scale(&self) -> usize {
        3
    }

    pub fn next(&self) -> Self {
        let list = Self::all();
        let i = list.iter().position(|r| r == self).unwrap_or(0);
        list[(i + 1) % list.len()]
    }

    pub fn prev(&self) -> Self {
        let list = Self::all();
        let i = list.iter().position(|r| r == self).unwrap_or(0);
        list[(i + list.len() - 1) % list.len()]
    }
}
