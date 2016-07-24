extern crate serial;
use serial::prelude::*;
use std::time::Duration;
use std::string::String;
use std::io;
use std::num;
use std::io::{Read, Write, Error, ErrorKind};

#[derive(Debug)]
pub enum LovesenseError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

pub trait LovesenseProtocol : Read + Write {
    fn send_command_none<S: Into<Vec<u8>>>(&mut self, cmd: S) -> Result<(), LovesenseError> {
        try!(self.write(&cmd.into()).map_err(LovesenseError::Io));
        Ok(())
    }

    fn send_command_str<S: Into<Vec<u8>>>(&mut self, cmd: S) -> Result<String, LovesenseError> {
        try!(self.send_command_none(cmd));
        let s = try!(self.read_status().map_err(LovesenseError::Io));
        Ok(s)
    }

    fn send_command_int<S: Into<Vec<u8>>>(&mut self, cmd: S) -> Result<u8, LovesenseError> {
        try!(self.send_command_none(cmd));
        self.read_int()
    }

    fn read_status(&mut self) -> Result<String, Error> {
        let mut buf = vec![0;4];

        // Assume we read everything we needed to.
        let _ = try!(self.read(&mut buf));
        let ret_str = String::from_utf8(buf).unwrap();
        if ret_str == "ER;" {
            return Err(Error::new(ErrorKind::Other, "Device error!"));
        }
        Ok(ret_str)
    }

    fn read_int(&mut self) -> Result<u8, LovesenseError> {
        let mut s = try!(self.read_status().map_err(LovesenseError::Io));
        // Take semicolon off end
        let new_len = s.len() - 1;
        s.truncate(new_len);
        let i = try!(s.parse::<u8>().map_err(LovesenseError::Parse));
        Ok(i)
    }

    fn vibrate(&mut self, speed: u8) -> Result<String, LovesenseError> {
        self.send_command_str(format!("Vibrate:{};", speed))
    }

    fn rotate(&mut self, speed: u8) -> Result<String, LovesenseError> {
        self.send_command_str(format!("Rotate:{};", speed))
    }

    fn air_level(&mut self, level: u8) -> Result<String, LovesenseError> {
        self.send_command_str(format!("Air:Level:{};", level))
    }

    fn inflate(&mut self, level: u8) -> Result<String, LovesenseError> {
        self.send_command_str(format!("Air:In:{};", level))
    }

    fn deflate(&mut self, level: u8) -> Result<String, LovesenseError> {
        self.send_command_str(format!("Air:Out:{};", level))
    }

    fn start_accelerometer(&mut self) -> Result<String, LovesenseError> {
        self.send_command_str("StartMove:1;")
    }

    fn stop_accelerometer(&mut self) -> Result<String, LovesenseError> {
        self.send_command_str("StopMove:1;")
    }

    fn change_rotation_direction(&mut self) -> Result<String, LovesenseError> {
        self.send_command_str("RotateChange;")
    }

    fn power_off(&mut self) -> Result<(), LovesenseError> {
        self.send_command_none("PowerOff;")
    }

    fn battery_level(&mut self) -> Result<u8, LovesenseError> {
        self.send_command_int("BatteryLevel;")
    }

    fn device_type(&mut self) -> Result<String, LovesenseError> {
        self.send_command_str("DeviceType;")
    }

    fn device_status(&mut self) -> Result<u8, LovesenseError> {
        self.send_command_int("Status:1;")
    }

}

pub struct LovesenseDevice<'a> {
    port: Box<serial::SerialPort + 'a>
}

impl<'a> Read for LovesenseDevice<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        return self.port.read(buf);
    }
}

impl<'a> Write for LovesenseDevice<'a> {
    fn write(&mut self, command: &[u8]) -> io::Result<usize> {
        return self.port.write(command);
    }

    fn flush(&mut self) -> io::Result<()> {
        // We don't buffer writable content, so flush does nothing.
        Ok(())
    }
}

impl<'a> LovesenseProtocol for LovesenseDevice<'a> {
}

impl<'a> LovesenseDevice<'a> {
    pub fn new(port_name: &str) -> LovesenseDevice<'a> {
        LovesenseDevice { port: {
            let mut s = serial::open(&port_name).unwrap();
            // Timeout needs to be weirdly high. Bluetooth!
            let _ = s.set_timeout(Duration::from_secs(5));
            Box::new(s)
        }
        }
    }
}
