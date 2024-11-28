const MEM_SIZE: usize = 4096;
const START_MEM: u16 = 0x200;

pub struct Chip {
    pub v: [u8; 16],
    pub i: u16,
    pub sp: u16,
    pub st: u8,
    pub dt: u8,
    pub pc: u16,
    pub mem: [u8; MEM_SIZE],
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            v: [0; 16],
            i: 0,
            sp: 0,
            st: 0,
            dt: 0,
            pc: START_MEM,
            mem: [0; MEM_SIZE],
        }
    }

    pub fn load(&mut self, filepath: String) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = std::fs::read(filepath)?;

        for (dst, src) in self.mem[(START_MEM as usize)..].iter_mut().zip(&buffer) {
            *dst = *src
        }

        Ok(())
    }
}
