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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

    #[inline]
    pub const fn raw(&self) -> u16 {
        self.0.get()
    }
}

impl std::fmt::Display for DmxChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A constant time O(1), memory efficent, fast, representation of channel selection,
/// superior to a vec or hashset for this usecase
pub struct DmxChannelSelectionMask {
    bits: [u64; 8] // 64 bytes, 512bits
}

impl DmxChannelSelectionMask {
    pub const fn default() -> Self {
        Self {
            bits: [0; 8]
        }
    }

    #[inline]
    pub const fn get(&self, channel: DmxChannel) -> bool {
        let index = channel.as_idx();
        (self.bits[index / 64] & (1 << (index % 64))) != 0
    }

    pub const fn set_true(&mut self, channel: DmxChannel) {
        let index = channel.as_idx();
        self.bits[index / 64] |= 1 << (index % 64);
    }

    pub const fn set_false(&mut self, channel: DmxChannel) {
        let index = channel.as_idx();
        self.bits[index / 64] &= !(1 << (index % 64));
    }

    pub const fn toggle(&mut self, channel: DmxChannel) {
        let index = channel.as_idx();
        self.bits[index / 64] ^= 1 << (index % 64);
    }

    pub const fn set_true_excl(&mut self, channel: DmxChannel) {
        self.clear();
        self.set_true(channel);
    }

    pub const fn clear(&mut self) {
        self.bits = [0; 8]
    }

    /// a more idiomatic standalone iterator would probably look something like this:
    /// ```rust
    /// pub fn iter(&self) -> impl Iterator<Item = DmxChannel> + '_ {
    ///     (1..=512)
    ///         .filter_map(DmxChannel::new)
    ///         .filter(|&channel| self.get(channel))
    /// }
    /// ```
    /// However, this has the issue of constructing a DmxChannel for each of the 512 channels,
    /// which is very inneficient, thus the following mess is required
    pub fn iter(&self) -> impl Iterator<Item = DmxChannel> + '_ {
        (0..512)
            .filter(|&i| {
                (self.bits[i / 64] & (1 << (i % 64))) != 0 // essentially a manual is_bit_set()
            })
            .map(|i| DmxChannel::new(i as u16 + 1).unwrap())
    }
}