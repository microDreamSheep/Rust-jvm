pub struct Reader{
    pub(crate) data:Vec<u8>,
    pub(crate) pointer:usize
}

impl Reader {
    pub fn new(data:Vec<u8>)->Reader{
        Reader{
            data,
            pointer: 0,
        }
    }

    pub fn read_uint8(&mut self) -> &u8 {
        let data = self.data.get(self.pointer).unwrap();
        self.pointer+=1;
        data
    }
    pub fn read_uint16(&mut self) -> u16{
        let high = self.data.get(self.pointer).unwrap().clone() as u16;
        let low = self.data.get(self.pointer+1).unwrap().clone() as u16;
        let data: u16 = high<<8|low;
        self.pointer+=2;
        data
    }
    pub fn read_uint32(&mut self) -> u32{
        let h1 = self.data.get(self.pointer).unwrap().clone() as u32;
        let h2 = self.data.get(self.pointer+1).unwrap().clone() as u32;
        let l1 = self.data.get(self.pointer+2).unwrap().clone() as u32;
        let l2 = self.data.get(self.pointer+3).unwrap().clone() as u32;
        let data: u32 = h1<<24|h2<<16|l1<<8|l2;
        self.pointer+=4;
        data
    }
    pub fn read_uint64(&mut self) ->u64{
        let h1 = self.data.get(self.pointer).unwrap().clone() as u64;
        let h2 = self.data.get(self.pointer+1).unwrap().clone() as u64;
        let h3 = self.data.get(self.pointer+2).unwrap().clone() as u64;
        let h4 = self.data.get(self.pointer+3).unwrap().clone() as u64;
        let l1 = self.data.get(self.pointer+4).unwrap().clone() as u64;
        let l2 = self.data.get(self.pointer+5).unwrap().clone() as u64;
        let l3 = self.data.get(self.pointer+6).unwrap().clone() as u64;
        let l4 = self.data.get(self.pointer+7).unwrap().clone() as u64;
        let data:u64 = h1<<56|h2<<48|h3<<40|h4<<32|l1<<24|l2<<16|l3<<8|l4;
        self.pointer+=8;
        data
    }
    fn _read_uint16s(){}
    pub fn read_bytes(&mut self, size:usize)->Vec<u8>{
        let mut vec:Vec<u8> = Vec::new();
        for _ in 0..size {
            vec.push(*(self.read_uint8()));
        }
        vec
    }
}