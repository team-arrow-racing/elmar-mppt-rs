use elmar_mppt::*;

struct Dummy {
    buffer: Vec<u8>,
}

impl Dummy {
    fn new() -> Self {
        Dummy {
            buffer: Vec::new(),
        }
    }
}

impl Mppt for Dummy {
    fn send_frame(&mut self, command: Command, data: &[u8]) -> elmar_mppt::Confirmation {
        self.buffer = Vec::from(data);

        Ok(())
    }

    fn receive_frame(&mut self, status: Status) -> Result<&[u8], &'static str> {
        Ok(&self.buffer)
    }
}

fn setup() -> Dummy {
    let dummy = Dummy::new();

    dummy
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
