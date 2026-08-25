//! Art-net
//! *sigh*
//! *relieved sigh*
//! I shouldn't be complaining, compared to many protocols, its
//! very simple, its just the mash-up of byte ordering which is odd

use crate::dmx;

pub const ARTNET_PORT: u16 = 6454;
pub const ARTNET_MAGIC_NUMBER: [u8; 8] = *b"Art-Net\0";
pub const ARTNET_OPCODE: u16 = 0x5000;
pub const ARTNET_PROTOCOL_VERSION: u16 = 14;
pub const ARTNET_SEQUENCE_NULL: u8 = 0;
pub const ARTNET_PHYSICAL_VIRTUAL: u8 = 0;
pub const ARTNET_UNIVERSE_SIZE: u16 = 512;

pub struct ArtNetPacket {
    pub universe: ArtNetUniverseId,
    pub dmx: dmx::DmxUniverse
}

impl ArtNetPacket {
    /// this unfortunately cant be const (not that it matters)
    /// because indexmut isnt stable yet
    pub fn to_raw(&self) -> [u8; 530] {
        let mut packet = [0u8; 530];

        packet[0..8].copy_from_slice(&ARTNET_MAGIC_NUMBER);

        // Art-Net opcode is little-endian.
        packet[8..10].copy_from_slice(&ARTNET_OPCODE.to_le_bytes());

        // Protocol version is big-endian.
        packet[10..12].copy_from_slice(&ARTNET_PROTOCOL_VERSION.to_be_bytes());

        // endiness doesnt matter here, there zeroed out
        packet[12] = ARTNET_SEQUENCE_NULL;
        packet[13] = ARTNET_PHYSICAL_VIRTUAL;

        // Universe is little-endian.
        packet[14..16].copy_from_slice(&self.universe.to_raw().to_le_bytes());

        // Length: 512 DMX channels.
        packet[16..18].copy_from_slice(&ARTNET_UNIVERSE_SIZE.to_be_bytes());

        // DMX data.
        packet[18..530].copy_from_slice(&self.dmx.to_raw());

        packet
    }
}

#[derive(Copy, Clone)]
pub struct ArtNetUniverseId(u16);

impl ArtNetUniverseId {
    pub const fn new(net: u8, subnet: u8, universe: u8) -> Option<Self> {
        if net > 127 || subnet > 15 || universe > 15 {
            return None;
        }

        Some(Self(
            ((net as u16) << 8)
                | ((subnet as u16) << 4)
                | universe as u16,
        ))
    }

    #[inline]
    pub const fn net(self) -> u8 {
        (self.0 >> 8) as u8
    }

    #[inline]
    pub const fn subnet(self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    #[inline]
    pub const fn universe(self) -> u8 {
        (self.0 & 0x0F) as u8
    }

    #[inline]
    pub const fn to_raw(self) -> u16 {
        self.0
    }
}