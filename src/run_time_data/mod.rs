use std::cell::RefCell;
use std::rc::Rc;
use crate::run_time_data::local_var::LocalVars;
use crate::run_time_data::operand_stack::OperandStack;

mod local_var;
mod operand_stack;
mod slot;

struct Thread{
    pub pc:u32,
    pub stack:Stack
}
impl Thread{
    pub fn new()->Thread{
        Thread{
            pc: 0,
            stack: Stack::new(1024),
        }
    }
    pub fn pc(&self)->u32{
        self.pc
    }
    pub fn set_pc(&mut self,pc:u32){
        self.pc = pc;
    }
    pub fn push_frame(&mut self,frame:Frame){
        self.stack.push(frame);
    }
    pub fn pop_frame(&mut self)->Frame{
        self.stack.pop()
    }
    pub fn current_frame(&self)->&Frame{
        self.stack.top()
    }

}
struct Stack{
    pub max_size:u32,
    pub size:u32,
    _top:Box<Frame>,
}
impl Stack{
    pub fn new(max_size:u32)->Stack{
        Stack{
            max_size,
            size: 0,
            _top: Box::new(Frame::new(None,0,0)),
        }
    }
    pub fn push(&mut self, mut frame:Frame){
        if self.size >= self.max_size{
            panic!("java.lang.StackOverflowError");
        }
        if self.size > 0{
            frame.lower = Some(self._top);
        }
        self._top = Box::new(frame);
        self.size += 1;
    }
    pub fn pop(&mut self)->Frame{
        if self.size == 0{
            panic!("jvm stack is empty!");
        }
        let top = self._top;
        self._top = top.lower.unwrap();
        self.size -= 1;
        *top
    }
    pub fn top(&self)->&Box<Frame>{
        if self.size == 0{
            panic!("jvm stack is empty!");
        }
        &self._top
    }


}

pub struct Frame{
    pub lower:Option<Box<Frame>>,
    local_vars:LocalVars,
    operator_stack:OperandStack,
}
impl Frame{
    pub fn new(lower:Option<Box<Frame>>,max_locals:u32,max_stack:u32)->Frame{
        Frame{
            lower,
            local_vars: LocalVars::new(max_locals),
            operator_stack: OperandStack::new(max_stack),
        }
    }

}

struct Object{}

