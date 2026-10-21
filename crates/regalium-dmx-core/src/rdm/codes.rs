// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The numbers an RDM message is made of: command classes, response types,
//! parameter ids and NACK reasons (ANSI E1.20). Each is a newtype over
//! every value, so a manufacturer's own decodes and re-encodes.

use core::fmt;

/// What a message asks or answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommandClass(pub u8);

impl CommandClass {
    /// Discovery: unique branch, mute, un-mute.
    pub const DISCOVERY: Self = Self(0x10);
    /// A discovery command's response.
    pub const DISCOVERY_RESPONSE: Self = Self(0x11);
    /// Read a parameter.
    pub const GET: Self = Self(0x20);
    /// A GET's response.
    pub const GET_RESPONSE: Self = Self(0x21);
    /// Write a parameter.
    pub const SET: Self = Self(0x30);
    /// A SET's response.
    pub const SET_RESPONSE: Self = Self(0x31);

    /// The class a response to this request carries: one more.
    #[must_use]
    pub const fn response(self) -> Self {
        Self(self.0.wrapping_add(1))
    }

    /// The class's name in E1.20, or `None`.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::DISCOVERY => "discovery",
            Self::DISCOVERY_RESPONSE => "discovery-response",
            Self::GET => "get",
            Self::GET_RESPONSE => "get-response",
            Self::SET => "set",
            Self::SET_RESPONSE => "set-response",
            _ => return None,
        })
    }
}

/// A response's type, carried where a request carries its port id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResponseType(pub u8);

impl ResponseType {
    /// Done; the data is the answer.
    pub const ACK: Self = Self(0x00);
    /// Not yet: ask the queue after the time the data gives, in 100 ms.
    pub const ACK_TIMER: Self = Self(0x01);
    /// Refused; the data is the reason.
    pub const NACK_REASON: Self = Self(0x02);
    /// More data than one message holds: ask again for the rest.
    pub const ACK_OVERFLOW: Self = Self(0x03);
}

/// A parameter id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pid(pub u16);

impl Pid {
    /// Discovery: does any device in this UID range exist?
    pub const DISC_UNIQUE_BRANCH: Self = Self(0x0001);
    /// Discovery: stop answering unique branch.
    pub const DISC_MUTE: Self = Self(0x0002);
    /// Discovery: answer unique branch again.
    pub const DISC_UN_MUTE: Self = Self(0x0003);
    /// A queued response, after `ACK_TIMER`.
    pub const QUEUED_MESSAGE: Self = Self(0x0020);
    /// Status messages.
    pub const STATUS_MESSAGES: Self = Self(0x0030);
    /// The parameters a device supports beyond the required ones.
    pub const SUPPORTED_PARAMETERS: Self = Self(0x0050);
    /// What a manufacturer's own parameter is.
    pub const PARAMETER_DESCRIPTION: Self = Self(0x0051);
    /// The device's model, footprint, address and personality.
    pub const DEVICE_INFO: Self = Self(0x0060);
    /// The model as text.
    pub const DEVICE_MODEL_DESCRIPTION: Self = Self(0x0080);
    /// The manufacturer as text.
    pub const MANUFACTURER_LABEL: Self = Self(0x0081);
    /// A label the user gives the device.
    pub const DEVICE_LABEL: Self = Self(0x0082);
    /// The firmware as text.
    pub const SOFTWARE_VERSION_LABEL: Self = Self(0x00C0);
    /// The current personality, and how many there are.
    pub const DMX_PERSONALITY: Self = Self(0x00E0);
    /// One personality's footprint and name.
    pub const DMX_PERSONALITY_DESCRIPTION: Self = Self(0x00E1);
    /// The DMX512 start address, 1 to 512.
    pub const DMX_START_ADDRESS: Self = Self(0x00F0);
    /// A sensor's type, range and name.
    pub const SENSOR_DEFINITION: Self = Self(0x0200);
    /// A sensor's reading.
    pub const SENSOR_VALUE: Self = Self(0x0201);
    /// Make the device show itself.
    pub const IDENTIFY_DEVICE: Self = Self(0x1000);
    /// Reset the device.
    pub const RESET_DEVICE: Self = Self(0x1001);
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}

/// Why a device refused, from a `NACK_REASON` response's data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NackReason(pub u16);

impl NackReason {
    /// The device does not know the parameter.
    pub const UNKNOWN_PID: Self = Self(0x0000);
    /// The request's data was malformed.
    pub const FORMAT_ERROR: Self = Self(0x0001);
    /// The device could not do it for a hardware reason.
    pub const HARDWARE_FAULT: Self = Self(0x0002);
    /// A proxy refused it.
    pub const PROXY_REJECT: Self = Self(0x0003);
    /// The parameter is read-only now.
    pub const WRITE_PROTECT: Self = Self(0x0004);
    /// GET or SET is not supported for this parameter.
    pub const UNSUPPORTED_COMMAND_CLASS: Self = Self(0x0005);
    /// A value out of range.
    pub const DATA_OUT_OF_RANGE: Self = Self(0x0006);
    /// The device's buffer is full.
    pub const BUFFER_FULL: Self = Self(0x0007);
    /// The request was too long for the device.
    pub const PACKET_SIZE_UNSUPPORTED: Self = Self(0x0008);
    /// No such sub-device.
    pub const SUB_DEVICE_OUT_OF_RANGE: Self = Self(0x0009);
    /// A proxy's buffer is full.
    pub const PROXY_BUFFER_FULL: Self = Self(0x000A);

    /// The reason's name in E1.20, or `None`.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::UNKNOWN_PID => "unknown-pid",
            Self::FORMAT_ERROR => "format-error",
            Self::HARDWARE_FAULT => "hardware-fault",
            Self::PROXY_REJECT => "proxy-reject",
            Self::WRITE_PROTECT => "write-protect",
            Self::UNSUPPORTED_COMMAND_CLASS => "unsupported-command-class",
            Self::DATA_OUT_OF_RANGE => "data-out-of-range",
            Self::BUFFER_FULL => "buffer-full",
            Self::PACKET_SIZE_UNSUPPORTED => "packet-size-unsupported",
            Self::SUB_DEVICE_OUT_OF_RANGE => "sub-device-out-of-range",
            Self::PROXY_BUFFER_FULL => "proxy-buffer-full",
            _ => return None,
        })
    }
}
