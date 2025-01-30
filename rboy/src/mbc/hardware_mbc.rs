use crate::mbc::{mbc0, mbc1, mbc2, mbc3, mbc5, MBC};
use crate::mbc::cartridge_reader::CartridgeReader;
use crate::StrResult;

pub struct HardwareMBC {
    mbc: Box<dyn MBC>,
}

impl HardwareMBC {
    pub fn new() -> StrResult<HardwareMBC> {
        Ok(HardwareMBC {
            mbc: HardwareMBC::get_mbc(),
        })
    }

    fn get_mbc() -> Box<dyn MBC+'static> {
        let mut cartridge_reader = CartridgeReader::new();
        match cartridge_reader.read_byte(0x147) {
            0x00 => Box::new(mbc0::MBC0::new(cartridge_reader)) as Box<dyn MBC>,
            0x01 ..= 0x03 => Box::new(mbc1::MBC1::new(cartridge_reader)) as Box<dyn MBC>,
            0x05 ..= 0x06 => Box::new(mbc2::MBC2::new(cartridge_reader)) as Box<dyn MBC>,
            0x0F ..= 0x13 => Box::new(mbc3::MBC3::new(cartridge_reader)) as Box<dyn MBC>,
            0x19 ..= 0x1E => Box::new(mbc5::MBC5::new(cartridge_reader)) as Box<dyn MBC>,
            _ => { panic!("Unsupported MBC type") },
        }
    }
}

impl MBC for HardwareMBC {
    fn readrom(&mut self, a: u16) -> u8 {
        self.mbc.readrom(a)
    }

    fn readram(&mut self, a: u16) -> u8 {
        self.mbc.readram(a)
    }

    fn writerom(&mut self, a: u16, v: u8) {
        self.mbc.writerom(a, v)
    }

    fn writeram(&mut self, a: u16, v: u8) {
        self.mbc.writeram(a, v)
    }

    fn check_and_reset_ram_updated(&mut self) -> bool {
        self.mbc.check_and_reset_ram_updated()
    }

    fn is_battery_backed(&self) -> bool {
        self.mbc.is_battery_backed()
    }

    fn loadram(&mut self, ramdata: &[u8]) -> StrResult<()> {
        self.mbc.loadram(ramdata)
    }

    fn dumpram(&self) -> Vec<u8> {
        self.mbc.dumpram()
    }
}
