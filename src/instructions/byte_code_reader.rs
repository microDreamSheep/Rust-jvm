struct ByteCodeReader{
    pc: usize,
    code: Vec<u8>,
}

impl ByteCodeReader {
    fn reset(&mut self, pc: usize, code: Vec<u8>) {
        self.pc = pc;
        self.code = code;
    }
    fn read_uint8(&mut self) -> u8 {
        let data = self.code.get(self.pc).unwrap().clone();
        self.pc += 1;
        data
    }
    fn read_uint16(&mut self) -> u16 {
        let high = self.code.get(self.pc).unwrap().clone() as u16;
        let low = self.code.get(self.pc + 1).unwrap().clone() as u16;
        let data: u16 = high << 8 | low;
        self.pc += 2;
        data
    }
    fn read_uint32(&mut self) -> u32 {
        let h1 = self.code.get(self.pc).unwrap().clone() as u32;
        let h2 = self.code.get(self.pc + 1).unwrap().clone() as u32;
        let l1 = self.code.get(self.pc + 2).unwrap().clone() as u32;
        let l2 = self.code.get(self.pc + 3).unwrap().clone() as u32;
        let data: u32 = h1 << 24 | h2 << 16 | l1 << 8 | l2;
        self.pc += 4;
        data
    }

}