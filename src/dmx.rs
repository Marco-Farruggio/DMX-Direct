use std::num::NonZeroU16;

pub struct DmxUniverse {
    data: [u8; 512]
}

impl DmxUniverse {
    pub const fn default() -> Self {
        Self {
            data: [0; 512]
        }
    }

    #[inline]
    pub const fn set_channel(&mut self, channel: DmxChannel, value: u8) {
        self.data[channel.as_idx()] = value;
    }

    #[inline]
    pub const fn get_channel(&self, channel: DmxChannel) -> u8 {
        self.data[channel.as_idx()]
    }

    #[inline]
    pub const fn to_raw(&self) -> [u8; 512] {
        self.data
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct DmxChannel(NonZeroU16);

/// A human-indexed (1..=512) DMX channel number
impl DmxChannel {
    pub const fn new(channel: u16) -> Option<Self> {
        if channel > 512 {
            return None;
        }

        // I could simply guard for 0 or greater than 512 above, but
        // id rather avoid an unwrap, even if it is completely safe
        match NonZeroU16::new(channel) {
            Some(channel) => Some(Self(channel)),
            None => None,
        }
    }

    // An index into a DMX512 universe, guarenteed to be safe
    #[inline]
    pub const fn as_idx(&self) -> usize {
        self.0.get() as usize - 1
    }
}

impl std::fmt::Display for DmxChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}