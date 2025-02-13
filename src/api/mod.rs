// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

pub mod client;

/// The kind of block device
#[derive(Debug, Serialize, Deserialize)]
pub enum BlockDeviceKind {
    Disk,
    Loopback { backing_file: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDevice {
    pub path: String,
    pub size: u64,
    pub sectors: u64,
    pub kind: BlockDeviceKind,
    pub model: Option<String>,
    #[serde(default)]
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition {
    pub name: String,
    pub path: String,
    pub number: u32,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Superblock {
    pub uuid: Option<String>,
    pub label: Option<String>,
    pub filesystem: String,
}

impl From<superblock::Superblock> for Superblock {
    fn from(val: superblock::Superblock) -> Self {
        Superblock {
            uuid: val.uuid().ok(),
            label: val.label().ok(),
            filesystem: val.kind().to_string(),
        }
    }
}

impl From<&disks::BlockDevice> for BlockDevice {
    fn from(val: &disks::BlockDevice) -> Self {
        match val {
            disks::BlockDevice::Disk(disk) => BlockDevice {
                path: disk.device_path().to_string_lossy().to_string(),
                size: disk.size(),
                sectors: disk.sectors(),
                kind: BlockDeviceKind::Disk,
                model: disk.model().map(String::from),
                partitions: disk.partitions().iter().map(Into::into).collect(),
            },
            disks::BlockDevice::Loopback(loopback) => BlockDevice {
                path: loopback.device_path().to_string_lossy().to_string(),
                size: loopback.disk().map_or(0, |d| d.size()),
                sectors: loopback.disk().map_or(0, |d| d.sectors()),
                kind: BlockDeviceKind::Loopback {
                    backing_file: loopback
                        .file_path()
                        .map(|p| p.to_string_lossy().to_string()),
                },
                partitions: loopback.disk().map_or(Vec::new(), |d| {
                    d.partitions().iter().map(Into::into).collect()
                }),
                model: None,
            },
        }
    }
}

impl From<&disks::partition::Partition> for Partition {
    fn from(val: &disks::partition::Partition) -> Self {
        Partition {
            name: val.name.clone(),
            path: val.device.to_string_lossy().to_string(),
            number: val.number,
            start: val.start,
            end: val.end,
            size: val.size,
        }
    }
}

/// Encapsulation of client-initiated requests
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetBlockDevices,
    Shutdown,
    GetSuperblock(String),
}

/// Encapsulation of server-initiated responses
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    BlockDevices(Vec<BlockDevice>),
    Error(String),
    Superblock(Superblock),
}
