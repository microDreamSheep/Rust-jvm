use std::rc::Rc;
use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

/**
 pop方法从操作数栈中弹出引用类型变量
 */
pub struct Pop;
impl Instruction for Pop {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        frame.operator_stack.pop_slot();
    }
}
/**
 pop2方法从操作数栈中弹出两个变量
 */
pub struct Pop2;
impl Instruction for Pop2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        frame.operator_stack.pop_slot();
        frame.operator_stack.pop_slot();
    }
}
/**
 dup方法复制栈顶变量并把复制值压入栈顶
 */
pub struct Dup;
impl Instruction for Dup {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot));
        frame.operator_stack.push_slot(slot);
    }
}
/**
 dup_x1方法复制栈顶变量并把复制值压入栈顶
 */
pub struct DupX1;
impl Instruction for DupX1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot1));
        frame.operator_stack.push_slot(slot2);
        frame.operator_stack.push_slot(slot1);
    }
}
/**
 dup_x2方法复制栈顶变量并把复制值压入栈顶
 */
pub struct DupX2;
impl Instruction for DupX2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        let slot3 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot1));
        frame.operator_stack.push_slot(slot3);
        frame.operator_stack.push_slot(slot2);
        frame.operator_stack.push_slot(slot1);
    }
}
/**
 dup2方法复制栈顶变量并把复制值压入栈顶
 */
pub struct Dup2;
impl Instruction for Dup2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot2));
        frame.operator_stack.push_slot(Rc::clone(&slot1));
        frame.operator_stack.push_slot(slot2);
        frame.operator_stack.push_slot(slot1);
    }
}
/**
 dup2_x1方法复制栈顶变量并把复制值压入栈顶
 */
pub struct Dup2X1;
impl Instruction for Dup2X1 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        let slot3 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot2));
        frame.operator_stack.push_slot(Rc::clone(&slot1));
        frame.operator_stack.push_slot(slot3);
        frame.operator_stack.push_slot(slot2);
        frame.operator_stack.push_slot(slot1);
    }
}
/**
 dup2_x2方法复制栈顶变量并把复制值压入栈顶
 */
pub struct Dup2X2;
impl Instruction for Dup2X2 {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        let slot3 = frame.operator_stack.pop_slot();
        let slot4 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(Rc::clone(&slot2));
        frame.operator_stack.push_slot(Rc::clone(&slot1));
        frame.operator_stack.push_slot(slot4);
        frame.operator_stack.push_slot(slot3);
        frame.operator_stack.push_slot(slot2);
        frame.operator_stack.push_slot(slot1);
    }
}
/**
 swap方法交换栈顶两个变量
 */
pub struct Swap;
impl Instruction for Swap {
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        // Do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let slot1 = frame.operator_stack.pop_slot();
        let slot2 = frame.operator_stack.pop_slot();
        frame.operator_stack.push_slot(slot1);
        frame.operator_stack.push_slot(slot2);
    }
}
