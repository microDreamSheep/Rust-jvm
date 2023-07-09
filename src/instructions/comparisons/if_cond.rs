use std::any::Any;
use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

pub struct IFEq {
    offset: i16,
}
impl Instruction for IFEq {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val == 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFNe {
    offset: i16,
}
impl Instruction for IFNe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val != 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFLt {
    offset: i16,
}
impl Instruction for IFLt {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val < 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFLe {
    offset: i16,
}
impl Instruction for IFLe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val <= 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFGt {
    offset: i16,
}
impl Instruction for IFGt {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val > 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFGe {
    offset: i16,
}
impl Instruction for IFGe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operator_stack.pop_int();
        if val >= 0 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpEq {
    offset: i16,
}
impl Instruction for IFIcmpEq {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 == val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpNe {
    offset: i16,
}
impl Instruction for IFIcmpNe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 != val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpLt {
    offset: i16,
}
impl Instruction for IFIcmpLt {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 < val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpLe {
    offset: i16,
}
impl Instruction for IFIcmpLe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 <= val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpGt {
    offset: i16,
}
impl Instruction for IFIcmpGt {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 > val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFIcmpGe {
    offset: i16,
}
impl Instruction for IFIcmpGe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val2 = frame.operator_stack.pop_int();
        let val1 = frame.operator_stack.pop_int();
        if val1 >= val2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFACmpEq {
    pub(crate) offset: i16,
}
impl Instruction for IFACmpEq {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let ref2 = &frame.operator_stack.pop_ref().unwrap() as *const dyn Any;
        let ref1 = &frame.operator_stack.pop_ref().unwrap() as *const dyn Any;
        if ref1 == ref2 {
            frame.jump(self.offset as usize);
        }
    }
}

pub struct IFACmpNe {
    offset: i16,
}
impl Instruction for IFACmpNe {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.offset = reader.read_int16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let ref2 = &frame.operator_stack.pop_ref().unwrap() as *const dyn Any;
        let ref1 = &frame.operator_stack.pop_ref().unwrap() as *const dyn Any;
        if ref1 != ref2 {
            frame.jump(self.offset as usize);
        }
    }
}
