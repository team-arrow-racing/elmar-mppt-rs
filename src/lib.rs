#![no_std]

use bitflags::bitflags;

/// Default identifier with rotary switch in the 0 position.
static DEFAULT_ID: u16 = 0x600;

/// Offset added by each incement of the rotary switch.
static ID_INCREMENT: u16 = 0x10;

/// Status message.
enum Status {
    Input = 0x00,
    Output = 0x01,
    Temperature = 0x02,
    AuxPower = 0x03,
    Limits = 0x04,
    Status = 0x05,
    PowerConnector = 0x06,
}

/// Command message.
enum Command {
    Mode = 0x08,
    MaxOutputVoltage = 0x0A,
    MaxInputCurrent = 0x0B,
}

/// Empty confirmation result.
type Confirmation = Result<(), &'static str>;

/// Float32 result.
type Float32Result = Result<f32, &'static str>;

/// Voltage rail.
enum VoltageRail {
    _12V,
    _3V3,
}

bitflags! {
    /// Error flags
    struct ErrorFlags: u8 {
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
    struct LimitFlags: u8 {
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
enum Mode {
    Standby = 0,
    On = 1,
}

trait Mppt {

    /// Measured input voltage in volts.
    fn input_voltage() -> Float32Result;

    /// Measured input current in amps.
    fn intput_current() -> Float32Result;

    /// Measured output voltage in volts.
    fn output_voltage() -> Float32Result;

    /// Measured output current in amps.
    fn output_current() -> Float32Result;

    /// Auxiliary power supply.
    fn aux_power_voltage(rail: VoltageRail) -> Float32Result;

    /// Maximium output voltage in volts.
    fn max_output_voltage() -> Float32Result;

    /// Maximum input current in amps.
    fn max_input_current() -> Float32Result;

    /// Device RX error count.
    fn rx_error_count() -> Result<u8, &'static str>;

    /// Device TX error count.
    fn tx_error_count() -> Result<u8, &'static str>;

    /// Device TX overflow count.
    fn tx_overflow_count() -> Result<u8, &'static str>;

    /// Error status flags.
    fn error_flags() -> Result<ErrorFlags, &'static str>;

    /// Limit status flags.
    fn limit_flags() -> Result<LimitFlags, &'static str>;

    /// Current operating mode.
    fn mode() -> Result<Mode, &'static str>;

    /// Test counter.
    ///
    /// Incrementing every device second.
    fn test_counter() -> Result<u8, &'static str>;

    /// Output voltage (battery side of fuse).
    fn power_connector_voltage() -> Float32Result;

    /// Power connector temperature in degrees celsius.
    fn power_connector_temperature() -> Float32Result;

    /// Set operating mode.
    fn set_mode(&self, mode: Mode) -> Confirmation {
        self.send_frame(Command::Mode, &[mode as u8])
    }

    /// Set the maximum output voltage in volts.
    fn set_max_output_voltage(&self, voltage: f32) -> Confirmation {
        self.send_frame(Command::MaxOutputVoltage, &voltage.to_be_bytes())
    }

    /// Set the maximum input current in amps.
    fn set_max_input_current(&self, current: f32) -> Confirmation {
        self.send_frame(Command::MaxInputCurrent, &current.to_be_bytes())
    }

    /// Send CAN bus frame with command.
    fn send_frame(&self, command: Command, data: &[u8]) -> Confirmation;
}
