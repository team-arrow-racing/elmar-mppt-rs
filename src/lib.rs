#![no_std]

use bitflags::bitflags;

/// Default identifier with rotary switch in the 0 position.
static DEFAULT_ID: u16 = 0x600;

/// Offset added by each incement of the rotary switch.
static ID_INCREMENT: u16 = 0x10;

/// Status message.
pub enum Status {
    Input = 0x00,
    Output = 0x01,
    Temperature = 0x02,
    AuxPower = 0x03,
    Limits = 0x04,
    Status = 0x05,
    PowerConnector = 0x06,
}

/// Command message.
pub enum Command {
    Mode = 0x08,
    MaxOutputVoltage = 0x0A,
    MaxInputCurrent = 0x0B,
}

/// Empty confirmation result.
pub type Confirmation = Result<(), &'static str>;

/// Float32 result.
pub type Float32Result = Result<f32, &'static str>;

/// Temperature sensor
pub enum TemperatureSensor {
    Mosfet,
    Controller,
}

/// Voltage rail.
pub enum VoltageRail {
    _12V,
    _3V3,
}

bitflags! {
    /// Error flags
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
    /// Limit flags
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
#[derive(PartialEq)]
pub enum Mode {
    Standby = 0,
    On = 1,
}

pub trait Mppt {
    /// Measured input voltage in volts.
    fn input_voltage(&mut self) -> Float32Result {
        match self.receive_frame(Status::Input) {
            Ok(v) => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Measured input current in amps.
    fn intput_current(&mut self) -> Float32Result {
        match self.receive_frame(Status::Input) {
            Ok(v) => Ok(f32::from_be_bytes(v[4..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Measured output voltage in volts.
    fn output_voltage(&mut self) -> Float32Result {
        match self.receive_frame(Status::Output) {
            Ok(v) => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Measured output current in amps.
    fn output_current(&mut self) -> Float32Result {
        match self.receive_frame(Status::Output) {
            Ok(v) => Ok(f32::from_be_bytes(v[4..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Temperature measurement in celsius.
    fn temperature(&mut self, sensor: TemperatureSensor) -> Float32Result {
        match self.receive_frame(Status::Temperature) {
            Ok(v) => match sensor {
                TemperatureSensor::Mosfet => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
                TemperatureSensor::Controller => {
                    Ok(f32::from_be_bytes(v[4..4].try_into().unwrap()))
                }
            },
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Auxiliary power supply.
    fn aux_power_voltage(&mut self, rail: VoltageRail) -> Float32Result {
        match self.receive_frame(Status::AuxPower) {
            Ok(v) => match rail {
                VoltageRail::_12V => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
                VoltageRail::_3V3 => Ok(f32::from_be_bytes(v[4..4].try_into().unwrap())),
            },
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Maximium output voltage in volts.
    fn max_output_voltage(&mut self) -> Float32Result {
        match self.receive_frame(Status::Limits) {
            Ok(v) => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Maximum input current in amps.
    fn max_input_current(&mut self) -> Float32Result {
        match self.receive_frame(Status::Limits) {
            Ok(v) => Ok(f32::from_be_bytes(v[4..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Device RX error count.
    fn rx_error_count(&mut self) -> Result<u8, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(v[0]),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Device TX error count.
    fn tx_error_count(&mut self) -> Result<u8, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(v[1]),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Device TX overflow count.
    fn tx_overflow_count(&mut self) -> Result<u8, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(v[2]),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Error status flags.
    fn error_flags(&mut self) -> Result<ErrorFlags, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(ErrorFlags::from_bits_truncate(v[3])),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Limit status flags.
    fn limit_flags(&mut self) -> Result<LimitFlags, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(LimitFlags::from_bits_truncate(v[4])),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Current operating mode.
    fn mode(&mut self) -> Result<Mode, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => match v[5] {
                0 => Ok(Mode::Standby),
                1 => Ok(Mode::On),
                _ => Err("mode value was invalid."),
            },
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Test counter.
    ///
    /// Incrementing every device second.
    fn test_counter(&mut self) -> Result<u8, &'static str> {
        match self.receive_frame(Status::Status) {
            Ok(v) => Ok(v[7]),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Output voltage (battery side of fuse).
    fn power_connector_voltage(&mut self) -> Float32Result {
        match self.receive_frame(Status::PowerConnector) {
            Ok(v) => Ok(f32::from_be_bytes(v[0..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Power connector temperature in degrees celsius.
    fn power_connector_temperature(&mut self) -> Float32Result {
        match self.receive_frame(Status::PowerConnector) {
            Ok(v) => Ok(f32::from_be_bytes(v[4..4].try_into().unwrap())),
            Err(e) => Err(e), // passthrough error
        }
    }

    /// Set operating mode.
    fn set_mode(&mut self, mode: Mode) -> Confirmation {
        self.send_frame(Command::Mode, &[mode as u8])
    }

    /// Set the maximum output voltage in volts.
    fn set_max_output_voltage(&mut self, voltage: f32) -> Confirmation {
        self.send_frame(Command::MaxOutputVoltage, &voltage.to_be_bytes())
    }

    /// Set the maximum input current in amps.
    fn set_max_input_current(&mut self, current: f32) -> Confirmation {
        self.send_frame(Command::MaxInputCurrent, &current.to_be_bytes())
    }

    /// Send CAN bus frame with command.
    fn send_frame(&mut self, command: Command, data: &[u8]) -> Confirmation;

    /// Receive CAN bus frame with status.
    fn receive_frame(&mut self, status: Status) -> Result<&[u8], &'static str>;
}
