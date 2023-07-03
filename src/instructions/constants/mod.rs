use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;


/**
 空指令
 */
struct Nop;
impl Instruction for Nop {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, _: &mut Frame) {
        // Do nothing
    }
}
/**
Const_null指令把null引用推入操作数栈顶
 */
struct AConstNull;
impl Instruction for AConstNull {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_ref(None);
    }
}
/**
D_const_0指令把double型0推入操作数栈顶
 */
struct DConst0;
impl Instruction for DConst0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_double(0.0);
    }
}
/**
D_const_1指令把double型1推入操作数栈顶
 */
struct DConst1;
impl Instruction for DConst1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_double(1.0);
    }
}
/**
F_const_0指令把float型0推入操作数栈顶
 */
struct FConst0;
impl Instruction for FConst0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_float(0.0);
    }
}
/**
F_const_1指令把float型1推入操作数栈顶
 */
struct FConst1;
impl Instruction for FConst1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_float(1.0);
    }
}
/**
F_const_2指令把float型2推入操作数栈顶
 */
struct FConst2;
impl Instruction for FConst2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_float(2.0);
    }
}
/**
I_const_m1指令把int型-1推入操作数栈顶
 */
struct IConstM1;
impl Instruction for IConstM1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(-1);
    }
}
/**
I_const_0指令把int型0推入操作数栈顶
 */
struct IConst0;
impl Instruction for IConst0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(0);
    }
}
/**
I_const_1指令把int型1推入操作数栈顶
 */
struct IConst1;
impl Instruction for IConst1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(1);
    }
}
/**
I_const_2指令把int型2推入操作数栈顶
 */
struct IConst2;
impl Instruction for IConst2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(2);
    }
}
/**
I_const_3指令把int型3推入操作数栈顶
 */
struct IConst3;
impl Instruction for IConst3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(3);
    }
}
/**
I_const_4指令把int型4推入操作数栈顶
 */
struct IConst4;
impl Instruction for IConst4 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(4);
    }
}
/**
I_const_5指令把int型5推入操作数栈顶
 */
struct IConst5;
impl Instruction for IConst5 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(5);
    }
}
/**
L_const_0指令把long型0推入操作数栈顶
 */
struct LConst0;
impl Instruction for LConst0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_long(0);
    }
}
/**
L_const_1指令把long型1推入操作数栈顶
 */
struct LConst1;
impl Instruction for LConst1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_long(1);
    }
}
/**
BIPush指令从操作数中获取一个byte型整数，扩展成int型，然后推入栈顶
 */
struct BIPush {
    val: i8,
}
impl Instruction for BIPush {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.val = reader.read_int8();
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(self.val as i32);
    }
}
/**
SIPush指令从操作数中获取一个short型整数，扩展成int型，然后推入栈顶
 */
struct SIPush {
    val: i16,
}
impl Instruction for SIPush {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.val = reader.read_int16();
    }
    fn execute(&mut self, frame: &mut Frame) {
        frame.operator_stack.push_int(self.val as i32);
    }
}