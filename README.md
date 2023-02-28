[![crates.io](https://img.shields.io/crates/v/elmar-mppt.svg)](https://crates.io/crates/elmar-mppt)
[![crates.io](https://img.shields.io/crates/d/elmar-mppt.svg)](https://crates.io/crates/elmar-mppt)

# Elmar MPPT Driver

Driver for Elmar MPPT devices.

Curerntly this crate only supports STM32 devices using the `bxcan` CAN Bus driver.

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
elmar-mppt = "0.1.0"
```

## References

- [Elmar MPPT User Manual](https://www.prohelion.com/wp-content/uploads/2021/02/Elmar_Solar_MPPT_Best_2021.pdf)

## License

Licensed under [Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
