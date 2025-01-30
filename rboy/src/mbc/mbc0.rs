use crate::mbc::cartridge_reader::CartridgeReader;
use crate::StrResult;
use crate::mbc::MBC;

pub struct MBC0 {
    cartridge_reader: CartridgeReader
}

impl MBC0 {
    pub fn new(cartridge_reader: CartridgeReader) -> MBC0 {
        MBC0 {
            cartridge_reader,
        }
    }
}

impl MBC for MBC0 {
    fn readrom(&mut self, a: u16) -> u8 { self.cartridge_reader.read_byte(a) }
    fn readram(&mut self, _a: u16) -> u8 { 0 }
    fn writerom(&mut self, _a: u16, _v: u8) { () }
    fn writeram(&mut self, _a: u16, _v: u8) { () }

    fn is_battery_backed(&self) -> bool { false }
    fn loadram(&mut self, _ramdata: &[u8]) -> StrResult<()> { Ok(()) }
    fn dumpram(&self) -> Vec<u8> { Vec::new() }
    fn check_and_reset_ram_updated(&mut self) -> bool { false }
}
