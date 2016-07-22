use serial;
use std::string::String;

pub struct LovesenseDevice<'a> {
    port: Box<serial::SerialPort + 'a>
}

impl<'a> LovesenseDevice<'a> {
    pub fn new(port_name: &str) -> LovesenseDevice<'a> {
        LovesenseDevice { port: Box::new(serial::open(&port_name).unwrap()) }
    }

    fn write(mut self, command: &[u8]) -> Result<String, String> {
        let result = self.port.write(command);
        let s;
        match result {
            Ok(size) => s = size,
            Err(_) => panic!("Something went wrong!")
        };
        if s != command.len() {
            panic!("Didn't write all bytes!");
        }
        let mut buf: Vec<u8> = (0..255).collect();
        self.port.read(&mut buf [..]);
        let ret_str = String::from_utf8(buf).unwrap();
        if ret_str == "ER;" {
            return Err(ret_str);
        }
        return Ok(ret_str);
    }

    pub fn set_vibrate(self, speed: u8) -> () {
        let command = format!("Vibrate:{};", speed);
        self.write(&command.into_bytes());
    }

    pub fn set_rotate(self, speed: u8) -> () {
        let command = format!("Rotate:{};", speed);
        self.write(&command.into_bytes());
    }

    pub fn set_air_level(self, level: u8) -> () {
        let command = format!("Air:Level:{};", level);
        self.write(&command.into_bytes());
    }

    pub fn inflate(self, level: u8) -> () {
        let command = format!("Air:In:{};", level);
        self.write(&command.into_bytes());
    }

    pub fn deflate(self, level: u8) -> () {
        let command = format!("Air:Out:{};", level);
        self.write(&command.into_bytes());
    }

    pub fn start_accelerometer(self) -> () {
        self.write(b"StartMove:1;");
    }

    pub fn stop_accelerometer(self) -> () {
        self.write(b"StopMove:1;");
    }

    pub fn change_rotation_direction(self) -> () {
        self.write(b"RotateChange;");
    }

    pub fn power_off(self) -> () {
        self.write(b"PowerOff;");
    }

    pub fn battery_level(self) -> () {
        self.write(b"BatteryLevel;");
    }

    pub fn device_type(self) -> () {
        self.write(b"DeviceType;");
    }

    pub fn device_status(self) -> () {
        self.write(b"DeviceStatus;");
    }
}
