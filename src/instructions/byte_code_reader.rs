pub struct ByteCodeReader{
    pc: usize,
    code: Vec<u8>,
}

impl ByteCodeReader {

    pub fn new(vec:Vec<u8>) -> ByteCodeReader {
        ByteCodeReader {
            pc: 0,
            code: vec,
        }
    }

    pub fn reset(&mut self, pc: usize, code: Vec<u8>) {
        self.pc = pc;
        self.code = code;
    }
    pub fn read_uint8(&mut self) -> u8 {
        let data:u8 = *self.code.get(self.pc).unwrap();
        self.pc += 1;
        data
    }

    pub fn read_int8(&mut self) -> i8 {
        self.read_uint8() as i8
    }

    pub fn read_uint16(&mut self) -> u16 {
        let high = self.code.get(self.pc).unwrap().clone() as u16;
        let low = self.code.get(self.pc + 1).unwrap().clone() as u16;
        let data: u16 = high << 8 | low;
        self.pc += 2;
        data
    }

    pub fn read_int16(&mut self) -> i16 {
        self.read_uint16() as i16
    }

    pub fn read_uint32(&mut self) -> u32 {
        let h1 = self.code.get(self.pc).unwrap().clone() as u32;
        let h2 = self.code.get(self.pc + 1).unwrap().clone() as u32;
        let l1 = self.code.get(self.pc + 2).unwrap().clone() as u32;
        let l2 = self.code.get(self.pc + 3).unwrap().clone() as u32;
        let data: u32 = h1 << 24 | h2 << 16 | l1 << 8 | l2;
        self.pc += 4;
        data
    }
    pub fn set_pointer(&mut self, pc: usize) {
        self.pc = pc;
    }
    pub fn get_pointer(&self) -> usize {
        self.pc
    }
}