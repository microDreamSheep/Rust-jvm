use crate::class_file::reader::Reader;

mod byte_code_reader;
mod constants;


trait Instruction{
    fn fetch_operands(&mut self, reader: &mut Reader);
    fn execute(&mut self, frame: &mut Frame);
}
