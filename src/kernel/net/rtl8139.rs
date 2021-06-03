use crate::{kernel, println};
use x86_64::instructions::port::Port;
use smoltcp::wire::{EthernetAddress, IpCidr, Ipv4Address};
#[derive(Clone)]
pub struct Ports {
    pub mac: [Port<u8>; 6]
}

impl Ports {
    pub fn new(io_base: u16) -> Self {
        Self {
            mac: [
                Port::new(io_base + 0x00),
                Port::new(io_base + 0x01),
                Port::new(io_base + 0x02),
                Port::new(io_base + 0x03),
                Port::new(io_base + 0x04),
                Port::new(io_base + 0x05),
            ]
        }
    }

    fn mac(&mut self) -> [u8; 6] {
        unsafe {
            [
                self.mac[0].read(),
                self.mac[1].read(),
                self.mac[2].read(),
                self.mac[3].read(),
                self.mac[4].read(),
                self.mac[5].read(),
            ]
        }
    }
}

#[derive(Clone)]
pub struct RTL8139 {
    ports: Ports,
    eth_addr: Option<EthernetAddress>,
}

impl RTL8139 {
    pub fn new(io_base: u16) -> Self {
        Self {
            ports: Ports::new(io_base),
            eth_addr: None,
        }
    }

    pub fn init(&mut self) {
        // Read MAC addr
        self.eth_addr = Some(EthernetAddress::from_bytes(&self.ports.mac()));
    }
}

pub fn init() {
    if let Some(mut pci_device) = kernel::pci::find_device(0x10EC, 0x8139) {
        pci_device.enable_bus_mastering();
        let io_base = (pci_device.base_addresses[0] as u16) & 0xFFF0;
        let mut net_device = RTL8139::new(io_base);
        net_device.init();
        if let Some(eth_addr) = net_device.eth_addr {
            println!("Addr: {}", eth_addr);
        }
    }
}