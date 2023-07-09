use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

pub struct Goto{
    pub(crate) offset: i16,
}
impl Instruction for Goto{
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.jump(self.offset as usize);
    }
}