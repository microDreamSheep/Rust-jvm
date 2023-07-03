use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::run_time_data::Frame;

mod byte_code_reader;
mod constants;
mod loads;


trait Instruction{
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader);
    fn execute(&mut self, frame: &mut Frame);
}
