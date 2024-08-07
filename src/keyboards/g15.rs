#[derive(Debug)]
pub enum KeyMapG15 {
    M1,
    M2,
    M3,
    MR,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    Unknown,
}
impl KeyMapG15 {
    pub fn from_str(s: &str) -> Self {
        match s {
            "M1" => Self::M1,
            "M2" => Self::M2,
            "M3" => Self::M3,
            "MR" => Self::MR,
            "G1" => Self::G1,
            "G2" => Self::G2,
            "G3" => Self::G3,
            "G4" => Self::G4,
            "G5" => Self::G5,
            "G6" => Self::G6,
            _ => Self::Unknown,
        }
    }
    pub fn from_buffer(b: [u8; 8]) -> KeyMapG15 {
        if (b[1] & 0x01) == 1 {
            return Self::G1;
        }
        if (b[1] & 0x02) >> 1 == 1 {
            return Self::G2;
        }
        if (b[1] & 0x04) >> 2 == 1 {
            return Self::G3;
        }
        if (b[1] & 0x08) >> 3 == 1 {
            return Self::G4;
        }
        if (b[1] & 0x10) >> 4 == 1 {
            return Self::G5;
        }
        if (b[1] & 0x20) >> 5 == 1 {
            return Self::G6;
        }
        if (b[1] & 0x40) >> 6 == 1 {
            return Self::M1;
        }
        if (b[1] & 0x80) >> 7 == 1 {
            return Self::M2;
        }
        if (b[2] & 0x20) >> 5 == 1 {
            return Self::M3;
        }
        if (b[2] & 0x40) >> 6 == 1 {
            return Self::MR;
        }
        Self::Unknown
    }
}
