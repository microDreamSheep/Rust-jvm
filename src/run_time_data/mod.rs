use std::cell::{RefCell};
use std::ops::Deref;
use std::rc::Rc;
use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::run_time_data::local_var::LocalVars;
use crate::run_time_data::operand_stack::OperandStack;

mod local_var;
mod operand_stack;
mod slot;

pub struct RThread {
    pub pc:u32,
    pub stack:Stack
}
impl RThread {
    pub fn new(frame:Frame)-> RThread {
        RThread {
            pc: 0,
            stack: Stack::new(1024,frame),
        }
    }
    pub fn pc(&self)->u32{
        self.pc
    }
    pub fn set_pc(&mut self,pc:u32){
        self.pc = pc;
    }
    pub fn push_frame(&mut self,frame:Rc<RefCell<Frame>>){
        self.stack.push(frame);
    }
    pub fn pop_frame(&mut self)->Rc<RefCell<Frame>>{
        self.stack.pop()
    }
    pub fn current_frame(&self)->Rc<RefCell<Frame>>{
        self.stack.top()
    }

}
pub struct Stack{
    pub max_size:u32,
    pub size:u32,
    _top:Rc<RefCell<Frame>>,
}
impl Stack{
    pub fn new(max_size:u32,top:Frame)->Stack{
        Stack{
            max_size,
            size: 0,
            _top: Rc::new(RefCell::new(top)),
        }
    }
    pub fn push(&mut self, frame:Rc<RefCell<Frame>>){
        if self.size >= self.max_size{
            panic!("java.lang.StackOverflowError");
        }
        if self.size > 0{
            frame.deref().borrow_mut().lower = Some(Rc::clone(&self._top));
        }
        self._top = frame;
        self.size += 1;
    }
    pub fn pop(&mut self)->Rc<RefCell<Frame>>{
        if self.size == 0{
            panic!("jvm stack is empty!");
        }
        let top = Rc::clone(&self._top);
        self._top = top.deref().borrow().lower.clone().unwrap();
        self.size -= 1;
        top
    }
    pub fn top(&self)->Rc<RefCell<Frame>>{
        Rc::clone(&self._top)
    }

}

pub struct Frame{
    pub lower:Option<Rc<RefCell<Frame>>>,
    pub(crate) local_vars:LocalVars,
    pub operator_stack:OperandStack,
    pub reader:ByteCodeReader,
}


impl Frame{
    pub fn new(lower:Option<Rc<RefCell<Frame>>>,max_locals:u32,max_stack:u32,reader:ByteCodeReader)->Frame{
        Frame{
            lower,
            local_vars: LocalVars::new(max_locals),
            operator_stack: OperandStack::new(max_stack),
            reader
        }
    }
    pub(crate) fn jump(&mut self, pc: usize) {
        self.reader.set_pointer(pc);
    }
    pub(crate) fn get_reader(&mut self)->&mut ByteCodeReader{
        &mut self.reader
    }

}

pub struct Object{}



