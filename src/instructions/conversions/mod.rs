use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

pub struct D2F;
impl Instruction for D2F {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_double();
        frame.operator_stack.push_float(val as f32);
    }
}

pub struct D2I;
impl Instruction for D2I {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_double();
        frame.operator_stack.push_int(val as i32);
    }
}

pub struct D2L;
impl Instruction for D2L {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_double();
        frame.operator_stack.push_long(val as i64);
    }
}