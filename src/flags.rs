use bitflags::bitflags;
use defmt::Format;

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
