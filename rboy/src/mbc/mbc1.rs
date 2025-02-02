use crate::mbc::{MBC, ram_banks, rom_banks};
use crate::StrResult;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_updated:bool,
    banking_mode: u8,
    rombank: usize,
    rambank: usize,
    has_battery: bool,
    rombanks: usize,
    rambanks: usize,
}

impl MBC1 {
    pub fn new(data: Vec<u8>) -> StrResult<MBC1> {
        let (has_battery, rambanks) = match data[0x147] {
            0x02 => (false, ram_banks(data[0x149])),
            0x03 => (true, ram_banks(data[0x149])),
            _ => (false, 0),
        };
        let rombanks = rom_banks(data[0x148]);
        let ramsize = rambanks * 0x2000;

        let res = MBC1 {
            rom: data,
            ram: ::std::iter::repeat(0u8).take(ramsize).collect(),
            ram_on: false,
            banking_mode: 0,
            rombank: 1,
            rambank: 0,
            ram_updated: false,
            has_battery: has_battery,
            rombanks: rombanks,
            rambanks: rambanks,
        };

        Ok(res)
    }
}

impl MBC for MBC1 {
    fn readrom(&self, a: u16) -> u8 {
        let bank = if a < 0x4000 {
            if self.banking_mode == 0 {
                0
            }
            else {
                self.rombank & 0xE0
            }
        }
        else {
            self.rombank
        };
        let idx = bank * 0x4000 | ((a as usize) & 0x3FFF);
        *self.rom.get(idx).unwrap_or(&0xFF)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on { return 0xFF }
        let rambank = if self.banking_mode == 1 { self.rambank } else { 0 };
        self.ram[(rambank * 0x2000) | ((a & 0x1FFF) as usize)]
    }

    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000 ..= 0x1FFF => { self.ram_on = v & 0xF == 0xA; },
            0x2000 ..= 0x3FFF => {
                let lower_bits = match (v as usize) & 0x1F {
                    0 => 1,
                    n => n,
                };
                self.rombank = ((self.rombank & 0x60) | lower_bits) % self.rombanks;
            },
            0x4000 ..= 0x5FFF => {
                if self.rombanks > 0x20 {
                    let upper_bits = (v as usize & 0x03) % (self.rombanks >> 5);
                    self.rombank = self.rombank & 0x1F | (upper_bits << 5)
                }
                if self.rambanks > 1 {
                    self.rambank = (v as usize) & 0x03;
                }
            },
            0x6000 ..= 0x7FFF => { self.banking_mode = v & 0x01; },
            _ => panic!("Could not write to {:04X} (MBC1)", a),
        }
    }

    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on { return }
        let rambank = if self.banking_mode == 1 { self.rambank } else { 0 };
        let address = (rambank * 0x2000) | ((a & 0x1FFF) as usize);
        if address < self.ram.len() {
            self.ram[address] = v;
            self.ram_updated = true;
        }
    }

    fn is_battery_backed(&self) -> bool {
        self.has_battery
    }

    fn loadram(&mut self, ramdata: &[u8]) -> StrResult<()> {
        if ramdata.len() != self.ram.len() {
            return Err("Loaded RAM has incorrect length");
        }

        self.ram = ramdata.to_vec();

        Ok(())
    }

    fn dumpram(&self) -> Vec<u8> {
        self.ram.to_vec()
    }

    fn check_and_reset_ram_updated(&mut self) -> bool {
        let result = self.ram_updated;
        self.ram_updated = false;
        result
    }
}
