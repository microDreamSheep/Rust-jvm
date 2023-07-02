use crate::class_file::reader::Reader;
use crate::instructions::Instruction;


/**
 空指令
 */
struct Nop;
impl Instruction for Nop {
    fn fetch_operands(&mut self, _: &mut Reader) {
        // Do nothing
    }

    fn execute(&self, _: &mut Frame) {
        // Do nothing
    }
}
