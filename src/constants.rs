#![allow(unused)]

/// Default identifier with rotary switch in the 0 position.
pub static ID_BASE: u16 = 0x600;

/// Offset added by each incement of the rotary switch.
pub static ID_INC: u16 = 0x10;

// message identifiers (normalized for base identifier offset)
pub const ID_BROADCAST_INPUT: u16 = 0x00;
pub const ID_BROADCAST_OUTPUT: u16 = 0x01;
pub const ID_BROADCAST_TEMPERATURE: u16 = 0x02;
pub const ID_BROADCAST_AUX_POWER: u16 = 0x03;
pub const ID_BROADCAST_LIMITS: u16 = 0x04;
pub const ID_BROADCAST_STATUS: u16 = 0x05;
pub const ID_BROADCAST_POWER_CONNECTOR: u16 = 0x06;

// command message identifiers (normalized for base identifier offset)
pub const ID_COMMAND_MODE: u16 = 0x08;
pub const ID_COMMAND_MAX_OUTPUT_VOLTAGE: u16 = 0x0A;
pub const ID_COMMAND_MAX_INPUT_CURERNT: u16 = 0x0B;
