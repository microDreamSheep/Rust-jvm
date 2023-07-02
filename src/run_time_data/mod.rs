use std::cell::{Ref, RefCell};
use std::ops::Deref;
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
struct Stack{
    pub max_size:u32,
    pub size:u32,
    _top:Rc<RefCell<Frame>>,
}
impl Stack{
    pub fn new(max_size:u32)->Stack{
        Stack{
            max_size,
            size: 0,
            _top: Rc::new(RefCell::new(Frame::new(None,0,0))),
        }
    }
    pub fn push(&mut self, mut frame:Rc<RefCell<Frame>>){
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
    local_vars:LocalVars,
    operator_stack:OperandStack,
}
impl Frame{
    pub fn new(lower:Option<Rc<RefCell<Frame>>>,max_locals:u32,max_stack:u32)->Frame{
        Frame{
            lower,
            local_vars: LocalVars::new(max_locals),
            operator_stack: OperandStack::new(max_stack),
        }
    }

}

pub struct Object{}

//test
#[cfg(test)]
mod tests{
    use crate::run_time_data::Frame;
    use crate::run_time_data::Thread;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_thread(){
        let mut thread = Thread::new();
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        let frame = Rc::new(RefCell::new(Frame::new(None,0,0)));
        thread.push_frame(frame);
        assert_eq!(thread.stack.size,9);
        thread.pop_frame();
        assert_eq!(thread.stack.size,8);
        thread.pop_frame();
        assert_eq!(thread.stack.size,7);
        thread.pop_frame();
        assert_eq!(thread.stack.size,6);
        thread.pop_frame();
        assert_eq!(thread.stack.size,5);
        thread.pop_frame();
        assert_eq!(thread.stack.size,4);
        thread.pop_frame();
        assert_eq!(thread.stack.size,3);
        thread.pop_frame();
        assert_eq!(thread.stack.size,2);
        thread.pop_frame();
        assert_eq!(thread.stack.size,1);
        thread.pop_frame();
        assert_eq!(thread.stack.size,0);
    }
}


