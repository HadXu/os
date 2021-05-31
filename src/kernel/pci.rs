use crate::println;
use bit_field::BitField;
use x86_64::instructions::port::Port;
pub struct DeviceConfig {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub status: u16,
    pub command: u16,
    pub base_addresses: [u32; 6],
    pub interrupt_pin: u8,
    pub interrupt_line: u8,
}

struct ConfigRegister {
    data_port: Port<u32>,
    addr_port: Port<u32>,
    addr: u32,
}

impl ConfigRegister {
    pub fn new(bus: u8, device: u8, function: u8, offset: u8) -> Self {
        Self {
            data_port: Port::new(0xCFC),
            addr_port: Port::new(0xCF8),
            addr: 0x8000_0000
                | ((bus as u32) << 16)
                | ((device as u32) << 11)
                | ((function as u32) << 8)
                | ((offset as u32) & 0xFC),
        }
    }

    pub fn read(&mut self) -> u32 {
        unsafe {
            self.addr_port.write(self.addr);
            self.data_port.read()
        }
    }

    pub fn write(&mut self, data: u32) {
        unsafe {
            self.addr_port.write(self.addr);
            self.data_port.write(data);
        }
    }
}

fn get_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    let mut register = ConfigRegister::new(bus, device, function, 0x00);
    register.read().get_bits(0..16) as u16
}

fn check_device(bus: u8, device: u8) {
    let function = 0;
    let vendor_id = get_vendor_id(bus, device, function);
    if vendor_id == 0xFFFF {
        return;
    }
    println!("{}", vendor_id);
}

pub fn init() {
    for bus in 0..256 {
        check_bus(bus as u8);
    }
}

fn check_bus(bus: u8) {
    for device in 0..32 {
        check_device(bus, device);
    }
}
