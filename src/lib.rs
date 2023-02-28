//! # Elmar MPPT Driver
//!
//! This a platform agnostic driver for the Elmar MPPT solar array power
//! converter.

#![no_std]

use bitflags::bitflags;
use bxcan::{Frame, Id, StandardId};
use defmt::Format;

/// Default identifier with rotary switch in the 0 position.
pub static ID_BASE: u16 = 0x600;

/// Offset added by each incement of the rotary switch.
pub static ID_INC: u16 = 0x10;

// message identifiers (normalized for base identifier offset)
const ID_BROADCAST_INPUT: u16 = 0x00;
const ID_BROADCAST_OUTPUT: u16 = 0x01;
const ID_BROADCAST_TEMPERATURE: u16 = 0x02;
const ID_BROADCAST_AUX_POWER: u16 = 0x03;
const ID_BROADCAST_LIMITS: u16 = 0x04;
const ID_BROADCAST_STATUS: u16 = 0x05;
const ID_BROADCAST_POWER_CONNECTOR: u16 = 0x06;

// command message identifiers (normalized for base identifier offset)
const ID_COMMAND_MODE: u16 = 0x08;
const ID_COMMAND_MAX_OUTPUT_VOLTAGE: u16 = 0x0A;
const ID_COMMAND_MAX_INPUT_CURERNT: u16 = 0x0B;

bitflags! {
    /// Error status flags
    #[derive(Format)]
    pub struct ErrorFlags: u8 {
        const HW_OVERVOLTAGE    = 1 << 0;
        const HW_OVERCURRENT    = 1 << 1;
        // const RESERVED       = 1 << 2;
        const UNDERVOLTAGE_12V  = 1 << 3;
        const BATTERY_FULL      = 1 << 4;
        const BATTERY_LOW       = 1 << 5;
        const MOSFET_OVERHEAT   = 1 << 6;
        const LOW_ARRAY_POWER   = 1 << 7;
    }
}

bitflags! {
    /// Limit status flags
    #[derive(Format)]
    pub struct LimitFlags: u8 {
        const GLOBAL_MPPT           = 1 << 0;
        const LOCAL_MPPT            = 1 << 1;
        const DUTY_CYCLE_MAX        = 1 << 2;
        const DUTY_CYCLE_MIN        = 1 << 3;
        const MOSFET_TEMPERATURE    = 1 << 4;
        const OUTPUT_VOLTAGE_MAX    = 1 << 5;
        const INPUT_CURRENT_MAX     = 1 << 6;
        const INPUT_CURRENT_MIN     = 1 << 7;
    }
}

/// Operating mode.
#[derive(Format, PartialEq, Clone, Copy)]
pub enum Mode {
    Standby = 0,
    On = 1,
}

/// Status of the device.
///
/// Each field is `Option<T>` as we do not know the value until a valid message
/// is processed.
#[derive(Format, Default, Clone, Copy)]
pub struct Status {
    pub input_voltage: Option<f32>,
    pub input_current: Option<f32>,
    pub output_voltage: Option<f32>,
    pub output_current: Option<f32>,
    pub mostfet_temperature: Option<f32>,
    pub controller_temperature: Option<f32>,
    pub rail_12v: Option<f32>,
    pub rail_3v: Option<f32>,
    pub maximum_output_voltage: Option<f32>,
    pub maximum_input_current: Option<f32>,
    pub can_rx_error_count: Option<u8>,
    pub can_tx_error_count: Option<u8>,
    pub can_tx_overflow_count: Option<u8>,
    pub error_flags: Option<ErrorFlags>,
    pub limit_flags: Option<LimitFlags>,
    pub mode: Option<Mode>,
    pub test_counter: Option<u8>,
    pub power_connector_voltage: Option<f32>,
    pub power_connector_temperature: Option<f32>,
}

/// MPPT device.
#[derive(Clone, Copy)]
pub struct Mppt {
    /// Base identifier used by the MPPT.
    /// 
    /// See the manual for configuring the identifier.
    base_id: u16,

    status: Status,
}

impl Mppt {
    /// Create a new MPPT instance.
    pub fn new(base_id: u16) -> Self {
        Self {
            base_id,
            status: Status {
                ..Default::default()
            },
        }
    }

    /// Process an incoming message and update the device status as needed.
    /// 
    /// If the message identifier does not match the device, this returns `Ok`.
    pub fn receive(&mut self, frame: &Frame) -> Result<(), &'static str> {
        match frame.id() {
            Id::Standard(id) => {
                if frame.is_data_frame() == false {
                    return Err("frame is not a data frame");
                }

                let data = frame.data().expect("msg has data");

                if id.as_raw() >= self.base_id {
                    match id.as_raw() - self.base_id {
                        ID_BROADCAST_INPUT => {
                            self.status.input_voltage = lower_float(data);
                            self.status.input_current = upper_float(data);
                        }
                        ID_BROADCAST_OUTPUT => {
                            self.status.output_voltage = lower_float(data);
                            self.status.output_current = upper_float(data);
                        }
                        ID_BROADCAST_TEMPERATURE => {
                            self.status.mostfet_temperature = lower_float(data);
                            self.status.controller_temperature = upper_float(data);
                        }
                        ID_BROADCAST_AUX_POWER => {
                            self.status.rail_12v = lower_float(data);
                            self.status.rail_3v = upper_float(data);
                        }
                        ID_BROADCAST_LIMITS => {
                            self.status.maximum_output_voltage = lower_float(data);
                            self.status.maximum_input_current = upper_float(data);
                        }
                        ID_BROADCAST_STATUS => {
                            self.status.can_rx_error_count = Some(data[0]);
                            self.status.can_tx_error_count = Some(data[1]);
                            self.status.can_tx_overflow_count = Some(data[2]);
                            self.status.error_flags = Some(ErrorFlags::from_bits_truncate(data[3]));
                            self.status.limit_flags = Some(LimitFlags::from_bits_truncate(data[4]));
                            self.status.mode = match data[5] {
                                0 => Some(Mode::Standby),
                                1 => Some(Mode::On),
                                _ => return Err("mode value read was invalid"),
                            };
                            self.status.test_counter = Some(data[7]);
                        }
                        ID_BROADCAST_POWER_CONNECTOR => {
                            self.status.power_connector_voltage = lower_float(data);
                            self.status.power_connector_temperature = upper_float(data);
                        }
                        _ => {
                            return Err("message id not handled");
                        }
                    }
                }
            }
            _ => {
                return Err("message is not intended for this device");
            }
        }

        return Ok(());
    }

    /// Get the current status of the MPPT.
    pub fn status(self) -> Status {
        self.status
    }

    /// Set the operating mode of the MPPT.
    pub fn set_mode(self, mode: Mode) -> Frame {
        let id = StandardId::new(self.base_id + ID_COMMAND_MODE).unwrap();
        Frame::new_data(id, [mode as u8; 1])
    }

    /// Set the maximum output voltage of the MPPT.
    pub fn set_maximum_output_voltage(&mut self, voltage: f32) -> Frame {
        let id = StandardId::new(self.base_id + ID_COMMAND_MAX_OUTPUT_VOLTAGE).unwrap();
        Frame::new_data(id, voltage.to_le_bytes())
    }

    /// Set the maximum input current of the MPPT.
    pub fn set_maximum_input_current(&mut self, current: f32) -> Frame {
        let id = StandardId::new(self.base_id + ID_COMMAND_MAX_INPUT_CURERNT).unwrap();
        Frame::new_data(id, current.to_le_bytes())
    }
}

fn lower_float(data: &[u8]) -> Option<f32> {
    match data[0..4].try_into() {
        Ok(v) => Some(f32::from_le_bytes(v)),
        Err(_) => None,
    }
}

fn upper_float(data: &[u8]) -> Option<f32> {
    match data[4..8].try_into() {
        Ok(v) => Some(f32::from_le_bytes(v)),
        Err(_) => None,
    }
}
