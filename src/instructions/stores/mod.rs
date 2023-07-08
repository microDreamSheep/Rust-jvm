use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

/*
------------i_store系列指令----------------
*/
fn execute_int(frame: &mut Frame, index: usize) {
    let val = frame.operator_stack.pop_int();
    frame.local_vars.set_int(index as u32, val);
}
/**
i_store指令
*/
pub struct IStore {
    index: usize,
}
impl Instruction for IStore {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, self.index);
    }
}
/**
 i_store_0指令把操作数栈顶的int型数值存入第一个局部变量
 */
pub struct IStore0;
impl Instruction for IStore0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 0);
    }
}
/**
 i_store_1指令把操作数栈顶的int型数值存入第二个局部变量
 */
pub struct IStore1;
impl Instruction for IStore1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 1);
    }
}
/**
 i_store_2指令把操作数栈顶的int型数值存入第三个局部变量
 */
pub struct IStore2;
impl Instruction for IStore2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 2);
    }
}
/**
 i_store_3指令把操作数栈顶的int型数值存入第四个局部变量
 */
pub struct IStore3;
impl Instruction for IStore3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_int(frame, 3);
    }
}

/*
------------l_store系列指令----------------
*/
fn execute_long(frame: &mut Frame, index: usize) {
    let val = frame.operator_stack.pop_long();
    frame.local_vars.set_long(index as u32, val);
}
/**
l_store指令
*/
pub struct LStore {
    index: usize,
}
impl Instruction for LStore {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, self.index);
    }
}
/**
 l_store_0指令把操作数栈顶的long型数值存入第一个局部变量
 */
pub struct LStore0;
impl Instruction for LStore0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 0);
    }
}
/**
 l_store_1指令把操作数栈顶的long型数值存入第二个局部变量
 */
pub struct LStore1;
impl Instruction for LStore1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 1);
    }
}
/**
 l_store_2指令把操作数栈顶的long型数值存入第三个局部变量
 */
pub struct LStore2;
impl Instruction for LStore2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 2);
    }
}
/**
 l_store_3指令把操作数栈顶的long型数值存入第四个局部变量
 */
pub struct LStore3;
impl Instruction for LStore3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_long(frame, 3);
    }
}

/*
------------f_store系列指令----------------
 */
fn execute_float(frame: &mut Frame, index: usize) {
    let val = frame.operator_stack.pop_float();
    frame.local_vars.set_float(index as u32, val);
}
/**
f_store指令
*/
pub struct FStore {
    index: usize,
}
impl Instruction for FStore {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, self.index);
    }
}
/**
 f_store_0指令把操作数栈顶的float型数值存入第一个局部变量
 */
pub struct FStore0;
impl Instruction for FStore0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_float(frame, 0);
    }
}
/**
 f_store_1指令把操作数栈顶的float型数值存入第二个局部变量
 */
pub struct FStore1;
impl Instruction for FStore1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {

        execute_float(frame, 1);
    }
}
/**
 f_store_2指令把操作数栈顶的float型数值存入第三个局部变量
 */
pub struct FStore2;
impl Instruction for FStore2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame:&mut Frame) {
        execute_float(frame, 2);
    }
}
/**
 f_store_3指令把操作数栈顶的float型数值存入第四个局部变量
 */
pub struct FStore3;
impl Instruction for FStore3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_float(frame, 3);
    }
}

/*
------------d_store系列指令----------------
 */
fn execute_double(frame: &mut Frame, index: usize) {
    let val = frame.operator_stack.pop_double();
    frame.local_vars.set_double(index as u32, val);
}
/**
d_store指令
*/
pub struct DStore {
    index: usize,
}
impl Instruction for DStore {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_double(frame, self.index);
    }
}
/**
 d_store_0指令把操作数栈顶的double型数值存入第一个局部变量
 */
pub struct DStore0;
impl Instruction for DStore0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_double(frame, 0);
    }
}
/**
 d_store_1指令把操作数栈顶的double型数值存入第二个局部变量
 */
pub struct DStore1;
impl Instruction for DStore1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_double(frame, 1);
    }
}
/**
 d_store_2指令把操作数栈顶的double型数值存入第三个局部变量
 */
pub struct DStore2;
impl Instruction for DStore2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_double(frame, 2);
    }
}
/**
 d_store_3指令把操作数栈顶的double型数值存入第四个局部变量
 */
pub struct DStore3;
impl Instruction for DStore3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_double(frame, 3);
    }
}

/*
------------a_store系列指令----------------
 */
fn execute_ref(frame: &mut Frame, index: usize) {
    let val = frame.operator_stack.pop_ref();
    frame.local_vars.set_ref(index as u32, val);
}
/**
a_store指令
*/
pub struct AStore {
    index: usize,
}
impl Instruction for AStore {
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        execute_ref(frame, self.index);
    }
}
/**
 a_store_0指令把操作数栈顶的引用型数值存入第一个局部变量
 */
pub struct AStore0;
impl Instruction for AStore0 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_ref(frame, 0);
    }
}
/**
 a_store_1指令把操作数栈顶的引用型数值存入第二个局部变量
 */
pub struct AStore1;
impl Instruction for AStore1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_ref(frame, 1);
    }
}
/**
 a_store_2指令把操作数栈顶的引用型数值存入第三个局部变量
 */
pub struct AStore2;
impl Instruction for AStore2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_ref(frame, 2);
    }
}
/**
 a_store_3指令把操作数栈顶的引用型数值存入第四个局部变量
 */
pub struct AStore3;
impl Instruction for AStore3 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        execute_ref(frame, 3);
    }
}

