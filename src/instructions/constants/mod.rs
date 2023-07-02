use crate::class_file::reader::Reader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;


/**
 空指令
 */
struct Nop;
impl Instruction for Nop {
    fn fetch_operands(&mut self, _: &mut Reader) {
        // Do nothing
    }

    fn execute(&mut self, _: &mut Frame) {
        // Do nothing
    }
}
