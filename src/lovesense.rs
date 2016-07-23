extern crate serial;
use serial::prelude::*;
use std::time::Duration;
use std::string::String;

pub struct LovesenseDevice<'a> {
    port: Box<serial::SerialPort + 'a>
}
impl<'a> LovesenseDevice<'a> {
    pub fn new(port_name: &str) -> LovesenseDevice<'a> {
        LovesenseDevice { port: {
            let mut s = serial::open(&port_name).unwrap();
            s.set_timeout(Duration::from_secs(1));
            Box::new(s)
        }
        }
    }

    fn write(&mut self, command: &[u8]) -> Result<String, String> {
        let result = self.port.write(command);
        let s;
        match result {
            Ok(size) => s = size,
            Err(_) => panic!("Something went wrong!")
        };
        if s != command.len() {
            panic!("Didn't write all bytes!");
        }
        // This doesn't really handle accelerometer stuff but oh well.
        let mut buf = Vec::with_capacity(2);
        let mut val : usize = self.port.read(&mut buf [..]).unwrap();
        while val == 0 {
            val = self.port.read(&mut buf [..]).unwrap();
        }
        println!("Read size: {}", val);
        let ret_str = String::from_utf8(buf).unwrap();
        println!("{}", ret_str);
        if ret_str == "ER;" {
            return Err(ret_str);
        }
        Ok(ret_str)
    }

    pub fn set_vibrate(&mut self, speed: u8) -> Result<String, String> {
        let command = format!("Vibrate:{};", speed);
        self.write(&command.into_bytes())
    }

    pub fn set_rotate(&mut self, speed: u8) -> Result<String, String> {
        let command = format!("Rotate:{};", speed);
        self.write(&command.into_bytes())
    }

    pub fn set_air_level(&mut self, level: u8) -> Result<String, String> {
        let command = format!("Air:Level:{};", level);
        self.write(&command.into_bytes())
    }

    pub fn inflate(&mut self, level: u8) -> Result<String, String> {
        let command = format!("Air:In:{};", level);
        self.write(&command.into_bytes())
    }

    pub fn deflate(&mut self, level: u8) -> Result<String, String> {
        let command = format!("Air:Out:{};", level);
        self.write(&command.into_bytes())
    }

    pub fn start_accelerometer(&mut self) -> Result<String, String> {
        self.write(b"StartMove:1;")
    }

    pub fn stop_accelerometer(&mut self) -> Result<String, String> {
        self.write(b"StopMove:1;")
    }

    pub fn change_rotation_direction(&mut self) -> Result<String, String> {
        self.write(b"RotateChange;")
    }

    pub fn power_off(&mut self) -> Result<String, String> {
        self.write(b"PowerOff;")
    }

    pub fn battery_level(&mut self) -> Result<String, String> {
        self.write(b"BatteryLevel;")
    }

    pub fn device_type(&mut self) -> Result<String, String> {
        self.write(b"DeviceType;")
    }

    pub fn device_status(&mut self) -> Result<String, String> {
        self.write(b"DeviceStatus:1;")
    }
}
