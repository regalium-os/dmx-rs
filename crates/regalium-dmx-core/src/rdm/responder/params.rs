// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The parameters a responder answers, GET and SET.

use super::{Responder, nack, parts};
use crate::rdm::codes::{CommandClass, NackReason, Pid, ResponseType};
use crate::rdm::message::Message;

/// The longest device label: 32 octets.
const LABEL: usize = 32;

impl Responder {
    /// The answer to a GET or SET: its type, data, and the parameter it
    /// answers for.
    pub(super) fn parameter(&mut self, m: &Message<'_>) -> (ResponseType, Vec<u8>, Pid) {
        if m.pid == Pid::QUEUED_MESSAGE && m.command_class == CommandClass::GET {
            return match self.queued.take() {
                Some((pid, data)) => (ResponseType::ACK, data, pid),
                None => (ResponseType::ACK, Vec::new(), Pid::STATUS_MESSAGES),
            };
        }
        // Another request abandons an overflow in progress.
        if let Some((pid, rest)) = self.overflow.take()
            && pid == m.pid
            && m.command_class == CommandClass::GET
        {
            let (kind, data) = parts(&mut self.overflow, pid, rest);
            return (kind, data, pid);
        }
        let (kind, data) = match m.command_class {
            CommandClass::GET => self.get(m.pid),
            CommandClass::SET => self.set(m.pid, m.data),
            _ => nack(NackReason::UNSUPPORTED_COMMAND_CLASS),
        };
        if self.slow == Some(m.pid) && kind == ResponseType::ACK {
            self.slow = None;
            self.queued = Some((m.pid, data));
            return (ResponseType::ACK_TIMER, vec![0, 1], m.pid);
        }
        (kind, data, m.pid)
    }

    fn get(&mut self, pid: Pid) -> (ResponseType, Vec<u8>) {
        let ack = |data: Vec<u8>| (ResponseType::ACK, data);
        match pid {
            Pid::DEVICE_INFO => ack(self.info.encode().to_vec()),
            Pid::DMX_START_ADDRESS => ack(self.info.start_address.to_be_bytes().to_vec()),
            Pid::DEVICE_LABEL => ack(self.label.clone()),
            Pid::IDENTIFY_DEVICE => ack(vec![u8::from(self.identify)]),
            Pid::SUPPORTED_PARAMETERS => {
                let data = self
                    .supported
                    .iter()
                    .flat_map(|p| p.0.to_be_bytes())
                    .collect();
                parts(&mut self.overflow, pid, data)
            }
            _ => nack(NackReason::UNKNOWN_PID),
        }
    }

    fn set(&mut self, pid: Pid, data: &[u8]) -> (ResponseType, Vec<u8>) {
        let done = (ResponseType::ACK, Vec::new());
        match (pid, data) {
            (Pid::DMX_START_ADDRESS, &[hi, lo]) => match u16::from_be_bytes([hi, lo]) {
                address @ 1..=512 => {
                    self.info.start_address = address;
                    done
                }
                _ => nack(NackReason::DATA_OUT_OF_RANGE),
            },
            (Pid::DEVICE_LABEL, label) if label.len() <= LABEL => {
                self.label = label.to_vec();
                done
            }
            (Pid::IDENTIFY_DEVICE, &[on @ (0 | 1)]) => {
                self.identify = on == 1;
                done
            }
            (Pid::IDENTIFY_DEVICE, &[_]) => nack(NackReason::DATA_OUT_OF_RANGE),
            (Pid::DMX_START_ADDRESS | Pid::DEVICE_LABEL | Pid::IDENTIFY_DEVICE, _) => {
                nack(NackReason::FORMAT_ERROR)
            }
            (Pid::DEVICE_INFO | Pid::SUPPORTED_PARAMETERS, _) => {
                nack(NackReason::UNSUPPORTED_COMMAND_CLASS)
            }
            _ => nack(NackReason::UNKNOWN_PID),
        }
    }
}
