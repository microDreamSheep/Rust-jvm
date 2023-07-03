use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

/*
------------i_load系列指令----------------
 */
fn execute_int(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_int(index as u32);
    frame.operator_stack.push_int(val);
}

/**
i_load指令
 */
struct ILoad{
    index: u8,
}
impl Instruction for ILoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, self.index);
    }
}
/**
 i_load_0指令把第一个int型局部变量推入操作数栈顶
 */
struct ILoad0;
impl Instruction for ILoad0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 0);
    }
}
/**
 i_load_1指令把第二个int型局部变量推入操作数栈顶
 */
struct ILoad1;
impl Instruction for ILoad1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 1);
    }
}
/**
 i_load_2指令把第三个int型局部变量推入操作数栈顶
 */
struct ILoad2;
impl Instruction for ILoad2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 2);
    }
}
/**
 i_load_3指令把第四个int型局部变量推入操作数栈顶
 */
struct ILoad3;
impl Instruction for ILoad3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 3);
    }
}
/*
------------l_load系列指令----------------
 */
fn execute_long(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_long(index as u32);
    frame.operator_stack.push_long(val);
}
struct LLoad{
    index: u8,
}
impl Instruction for LLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, self.index);
    }
}
/**
 l_load_0指令把第一个long型局部变量推入操作数栈顶
 */
struct LLoad0;
impl Instruction for LLoad0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
       execute_long(frame, 0);
    }
}
/**
 l_load_1指令把第二个long型局部变量推入操作数栈顶
 */
struct LLoad1;
impl Instruction for LLoad1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 1);
    }
}
/**
 l_load_2指令把第三个long型局部变量推入操作数栈顶
 */
struct LLoad2;
impl Instruction for LLoad2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 2);
    }
}
/**
 l_load_3指令把第四个long型局部变量推入操作数栈顶
 */
struct LLoad3;
impl Instruction for LLoad3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 3);
    }
}
/*
------------f_load系列指令----------------
 */
fn execute_float(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_float(index as u32);
    frame.operator_stack.push_float(val);
}
struct FLoad{
    index: u8,
}
impl Instruction for FLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8();
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, self.index);
    }
}
/**
 f_load_0指令把第一个float型局部变量推入操作数栈顶
 */
struct FLoad0;
impl Instruction for FLoad0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, 0);
    }
}
/**
 f_load_1指令把第二个float型局部变量推入操作数栈顶
 */
struct FLoad1;
impl Instruction for FLoad1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, 1);
    }
}
/**
 f_load_2指令把第三个float型局部变量推入操作数栈顶
 */
struct FLoad2;
impl Instruction for FLoad2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, 2);
    }
}
/**
 f_load_3指令把第四个float型局部变量推入操作数栈顶
 */
struct FLoad3;
impl Instruction for FLoad3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, 3);
    }
}
/*
------------d_load系列指令----------------
 */
fn execute_double(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_double(index as u32);
    frame.operator_stack.push_double(val);
}
struct DLoad{
    index: u8,
}
impl Instruction for DLoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8();
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_double(frame, self.index);
    }
}
/**
 d_load_0指令把第一个double型局部变量推入操作数栈顶
 */
struct DLoad0;
impl Instruction for DLoad0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
        execute_double(frame, 0);
    }
}
/**
 d_load_1指令把第二个double型局部变量推入操作数栈顶
 */
struct DLoad1;
impl Instruction for DLoad1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {
       execute_double(frame, 1);
    }
}
/**
 d_load_2指令把第三个double型局部变量推入操作数栈顶
 */
struct DLoad2;
impl Instruction for DLoad2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {

        execute_double(frame, 2);
    }
}
/**
 d_load_3指令把第四个double型局部变量推入操作数栈顶
 */
struct DLoad3;
impl Instruction for DLoad3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {

        execute_double(frame, 3);
    }
}
/*
------------a_load系列指令----------------
 */
fn execute_ref(frame: &mut Frame, index: u8) {
    let val = frame.local_vars.get_ref(index as u32);
    frame.operator_stack.push_ref(val)
}
struct ALoad{
    index: u8,
}
impl Instruction for ALoad {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8();
    }
    fn execute(&mut self, frame: &mut Frame) {

        execute_ref(frame, self.index);
    }
}
/**
 a_load_0指令把第一个引用型局部变量推入操作数栈顶
 */
struct ALoad0;
impl Instruction for ALoad0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame) {

        execute_ref(frame, 0);
    }
}
/**
 a_load_1指令把第二个引用型局部变量推入操作数栈顶
 */
struct ALoad1;
impl Instruction for ALoad1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self, frame: &mut Frame){

        execute_ref(frame, 1);
    }
}
/**
 a_load_2指令把第三个引用型局部变量推入操作数栈顶
 */
struct ALoad2;
impl Instruction for ALoad2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self,frame: &mut Frame) {

        execute_ref(frame, 2);
    }
}
/**
 a_load_3指令把第四个引用型局部变量推入操作数栈顶
 */
struct ALoad3;
impl Instruction for ALoad3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }
    fn execute(&mut self,frame: &mut Frame) {

        execute_ref(frame, 3);
    }
}