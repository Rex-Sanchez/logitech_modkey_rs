use std::{collections::HashMap, default, fs::read_to_string, hash};

use serde::{Deserialize, Serialize};
use virtual_keyboard_rs::VirtKeyboard;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct GKeys {
    G1: Option<String>,
    G2: Option<String>,
    G3: Option<String>,
    G4: Option<String>,
    G5: Option<String>,
    G6: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct G15Mappings {
    Normal: Option<GKeys>,
    M1: Option<GKeys>,
    M2: Option<GKeys>,
    M3: Option<GKeys>,
}

pub struct G15Mod {
    pub mode: G15ModModes,
    pub key_lockup: HashMap<G15ModModes, HashMap<G15KeyMap, String>>,
    pub kb: VirtKeyboard,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum G15ModModes {
    M,
    M1,
    M2,
    M3,
}
impl G15Mod {
    pub fn new(mappings:&str) -> Self {

        let mappings = toml::from_str::<G15Mappings>(&mappings).unwrap();

        let mut normal_map: HashMap<G15KeyMap, String> = HashMap::new();
        let mut m1_map: HashMap<G15KeyMap, String> = HashMap::new();
        let mut m2_map: HashMap<G15KeyMap, String> = HashMap::new();
        let mut m3_map: HashMap<G15KeyMap, String> = HashMap::new();

        if let Some(normal) = mappings.Normal {
            if let Some(val) = normal.G1 {
                normal_map.insert(G15KeyMap::G1, val);
            }
            if let Some(val) = normal.G2 {
                normal_map.insert(G15KeyMap::G2, val);
            }
            if let Some(val) = normal.G3 {
                normal_map.insert(G15KeyMap::G3, val);
            }
            if let Some(val) = normal.G4 {
                normal_map.insert(G15KeyMap::G4, val);
            }
            if let Some(val) = normal.G5 {
                normal_map.insert(G15KeyMap::G5, val);
            }
            if let Some(val) = normal.G6 {
                normal_map.insert(G15KeyMap::G6, val);
            }
        }

        if let Some(m1) = mappings.M1 {
            if let Some(val) = m1.G1 {
                m1_map.insert(G15KeyMap::G1, val);
            }
            if let Some(val) = m1.G2 {
                m1_map.insert(G15KeyMap::G2, val);
            }
            if let Some(val) = m1.G3 {
                m1_map.insert(G15KeyMap::G3, val);
            }
            if let Some(val) = m1.G4 {
                m1_map.insert(G15KeyMap::G4, val);
            }
            if let Some(val) = m1.G5 {
                m1_map.insert(G15KeyMap::G5, val);
            }
            if let Some(val) = m1.G6 {
                m1_map.insert(G15KeyMap::G6, val);
            }
        }
        if let Some(m2) = mappings.M2 {
            if let Some(val) = m2.G1 {
                m2_map.insert(G15KeyMap::G1, val);
            }
            if let Some(val) = m2.G2 {
                m2_map.insert(G15KeyMap::G2, val);
            }
            if let Some(val) = m2.G3 {
                m2_map.insert(G15KeyMap::G3, val);
            }
            if let Some(val) = m2.G4 {
                m2_map.insert(G15KeyMap::G4, val);
            }
            if let Some(val) = m2.G5 {
                m2_map.insert(G15KeyMap::G5, val);
            }
            if let Some(val) = m2.G6 {
                m2_map.insert(G15KeyMap::G6, val);
            }
        }
        if let Some(m3) = mappings.M3 {
            if let Some(val) = m3.G1 {
                m3_map.insert(G15KeyMap::G1, val);
            }
            if let Some(val) = m3.G2 {
                m3_map.insert(G15KeyMap::G2, val);
            }
            if let Some(val) = m3.G3 {
                m3_map.insert(G15KeyMap::G3, val);
            }
            if let Some(val) = m3.G4 {
                m3_map.insert(G15KeyMap::G4, val);
            }
            if let Some(val) = m3.G5 {
                m3_map.insert(G15KeyMap::G5, val);
            }
            if let Some(val) = m3.G6 {
                m3_map.insert(G15KeyMap::G6, val);
            }
        }

        let mut map = HashMap::new();
        map.insert(G15ModModes::M, normal_map);
        map.insert(G15ModModes::M1, m1_map);
        map.insert(G15ModModes::M2, m2_map);
        map.insert(G15ModModes::M3, m3_map);

        Self {
            mode: G15ModModes::M,
            key_lockup: map,
            kb: VirtKeyboard::new("G15_ModKeyboard").unwrap(),
        }
    }
    pub fn set_mode(&mut self, mode: G15ModModes) {
        self.mode = mode;
    }
    pub fn send_keyevent(&mut self, key: &G15KeyMap) {
        if let Some(mode) = self.key_lockup.get(&self.mode) {
            if let Some(map) = mode.get(&key) {
                self.kb.send_keystrokes(map);
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum G15KeyMap {
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
impl G15KeyMap {
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
    pub fn from_buffer(b: [u8; 8]) -> G15KeyMap {
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
