use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

pub mod if_cond;

pub struct LCmp;
impl Instruction for LCmp {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_long();
        let val1 = frame.operator_stack.pop_long();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else {
            -1
        };
        frame.operator_stack.push_int(result);
    }
}

pub struct FCmpL;
impl Instruction for FCmpL {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_float();
        let val1 = frame.operator_stack.pop_float();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else {
            -1
        };
        frame.operator_stack.push_int(result);
    }
}

pub struct FCmpG;
impl Instruction for FCmpG {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_float();
        let val1 = frame.operator_stack.pop_float();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else {
            -1
        };
        frame.operator_stack.push_int(result);
    }
}

pub struct DCmpL;
impl Instruction for DCmpL {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_double();
        let val1 = frame.operator_stack.pop_double();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else {
            -1
        };
        frame.operator_stack.push_int(result);
    }
}

pub struct DCmpG;
impl Instruction for DCmpG {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_double();
        let val1 = frame.operator_stack.pop_double();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else {
            -1
        };
        frame.operator_stack.push_int(result);
    }
}
