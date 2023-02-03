use elmar_mppt::*;

struct Dummy {
    buffer: Vec<u8>,
}

impl Dummy {
    fn new() -> Self {
        Dummy {
            buffer: Vec::with_capacity(8),
        }
    }
}

impl Mppt for Dummy {
    fn send_frame(&mut self, command: Command, data: &[u8]) -> elmar_mppt::Confirmation {
        let _ = command;

        self.buffer = Vec::from(data);

        Ok(())
    }

    fn receive_frame(&mut self, status: Status) -> Result<&[u8], &'static str> {
        let _ = status;

        Ok(&self.buffer)
    }
}

fn setup() -> Dummy {
    let dummy = Dummy::new();

    dummy
}

#[test]
fn test_get_input_voltage() {
    let mut m = setup();

    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);

    assert_eq!(m.input_voltage().unwrap(), 0f32);
}

#[test]
fn test_get_input_current() {
    let mut m = setup();

    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);
    m.buffer.push(0x00);

    assert_eq!(m.intput_current().unwrap(), 0f32);
}

#[test]
fn test_set_mode() {
    let mut m = setup();

    assert_eq!(m.set_mode(Mode::On), Ok(()));
    assert_eq!(m.buffer.len(), 1); // message length
    assert_eq!(m.buffer[0], 1); // message value

    assert_eq!(m.set_mode(Mode::Standby), Ok(()));
    assert_eq!(m.buffer.len(), 1); // message length
    assert_eq!(m.buffer[0], 0); // message value
}
