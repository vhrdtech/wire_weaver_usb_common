#![no_std]
#![allow(async_fn_in_trait)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod common;
mod receiver;
mod sender;
mod tests;

/// CRC used on packets that span multiple frames.
const CRC_KIND: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

/// Link versions itself, checked to guard against interpreting incompatible data.
const LINK_PROTOCOL_VERSION: u8 = 1;

/// Minimum message sized assumed to be supported before link setup is done and higher number is
/// potentially received.
const MIN_MESSAGE_SIZE: usize = 64;

// Some features are host and device specific to reduce confusion.
#[cfg(all(feature = "device", feature = "host"))]
compile_error!("Exactly one of 'device' or 'host' features must be enabled");

pub use common::{Error, PacketSink, PacketSource, ProtocolInfo, WireWeaverUsbLink};
pub use receiver::{MessageKind, ReceiverStats};
pub use sender::SenderStats;
