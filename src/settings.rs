use super::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub default_host: SocketAddr,
    pub default_mac: String,
    pub default_oui: String,
    pub device: HashMap<String, Device>,
    pub packet_forwarder: HashMap<String, PacketForwarder>,
}

impl Settings {
    /// Load Settings from a given path. Settings are loaded from a default.toml
    /// file in the given path, followed by merging in an optional settings.toml
    /// in the same folder.
    pub fn new(path: &Path) -> Result<Settings> {
        let mut c = Config::new();
        let default_file = path.join("default.toml");
        // Load default config and merge in overrides
        c.merge(File::with_name(default_file.to_str().expect("file name")))?;
        let settings_file = path.join("settings.toml");
        if settings_file.exists() {
            c.merge(File::with_name(settings_file.to_str().expect("file name")))?;
        }
        c.try_into().map_err(|e| e.into())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Device {
    pub credentials: Credentials,
    #[serde(default = "default_rejoin_frames")]
    pub rejoin_frames: u32,
    pub oui: Option<String>,
    pub packet_forwarder: Option<String>,
}

fn default_rejoin_frames() -> u32 {
    0xFFFF
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Credentials {
    pub app_eui: String,
    pub app_key: String,
    pub dev_eui: String,
}

impl Credentials {
    pub fn appeui_cloned_into_buf(&self) -> Result<[u8; 8]> {
        let vec = hex::decode(&self.app_eui)?;
        Ok([
            vec[7], vec[6], vec[5], vec[4], vec[3], vec[2], vec[1], vec[0],
        ])
    }

    pub fn deveui_cloned_into_buf(&self) -> Result<[u8; 8]> {
        let vec = hex::decode(&self.dev_eui)?;
        Ok([
            vec[7], vec[6], vec[5], vec[4], vec[3], vec[2], vec[1], vec[0],
        ])
    }
    pub fn appkey_cloned_into_buf(&self) -> Result<[u8; 16]> {
        let vec = hex::decode(&self.app_key)?;
        Ok([
            vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], vec[6], vec[7], vec[8], vec[9],
            vec[10], vec[11], vec[12], vec[13], vec[14], vec[15],
        ])
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PacketForwarder {
    mac: String,
    pub host: std::net::SocketAddr,
}

impl PacketForwarder {
    pub fn mac_cloned_into_buf(&self) -> Result<[u8; 8]> {
        mac_string_into_buf(&self.mac)
    }
}

pub fn mac_string_into_buf(s: &str) -> Result<[u8; 8]> {
    let vec = hex::decode(s)?;
    Ok([
        vec[7], vec[6], vec[5], vec[4], vec[3], vec[2], vec[1], vec[0],
    ])
}
