use std::cell::{RefCell};
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
    use std::any::Any;
    use crate::run_time_data::{Frame, Object};
    use crate::run_time_data::Thread;
    use std::cell::RefCell;
    use std::f32::consts::PI;
    use std::f64::consts::E;
    use std::ops::Deref;
    use std::rc::Rc;

    #[test]
    fn test_thread(){
        let mut thread = Thread::new();
        thread.push_frame(Rc::new(RefCell::new(Frame::new(None,50,50))));
        thread.push_frame(Rc::new(RefCell::new(Frame::new(None,50,50))));
        thread.push_frame(Rc::new(RefCell::new(Frame::new(None,50,50))));
        let a = thread.pop_frame();
        //测试操作数栈
        let operator_stack = &mut a.deref().borrow_mut().operator_stack;
        operator_stack.push_int(100);
        operator_stack.push_long(2997924580);
        operator_stack.push_float(PI);
        operator_stack.push_double(E);
        let obj = Rc::new(RefCell::new(Object{}));
        operator_stack.push_ref(Some(Rc::clone(&obj)));
        //获取obj的内存地址
        let obj_addr = operator_stack.pop_ref().unwrap().deref().borrow().deref() as *const dyn Any;
        //判断obj的内存地址是否相等
        assert_eq!(obj_addr,obj.deref().borrow().deref() as *const dyn Any);
        assert_eq!(operator_stack.pop_double(),E);
        assert_eq!(operator_stack.pop_float(),PI);
        assert_eq!(operator_stack.pop_long(),2997924580);
        assert_eq!(operator_stack.pop_int(),100);
        //测试局部变量表
        let binding = thread.pop_frame();
        let local_vars = &mut binding.deref().borrow_mut().local_vars;
        local_vars.set_int(0,100);
        local_vars.set_long(1,2997924580);
        local_vars.set_float(3,PI);
        local_vars.set_double(5,E);
        let obj = Rc::new(RefCell::new(Object{}));
        local_vars.set_ref(7,Some(Rc::clone(&obj)));
        //获取obj的内存地址
        let obj_addr = local_vars.get_ref(7).unwrap().deref().borrow().deref() as *const dyn Any;
        //判断obj的内存地址是否相等
        assert_eq!(obj_addr,obj.deref().borrow().deref() as *const dyn Any);
        assert_eq!(local_vars.get_double(5),E);
        assert_eq!(local_vars.get_float(3),PI);
        assert_eq!(local_vars.get_long(1),2997924580);
        assert_eq!(local_vars.get_int(0),100);

    }
}


