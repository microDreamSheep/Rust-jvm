use std::cell::RefCell;
use std::rc::Rc;

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

}
struct Stack{
    pub max_size:u32,
    pub size:u32,
    _top:Frame,
}

struct Frame{
    pub lower:Frame,
    local_vars:LocalVars,
    operator_stack:OperatorStack,
}

struct OperatorStack{
    pub size:u32,
    slots:Vec<Slot>,
}

struct LocalVars{
    pub slots:Vec<Slot>
}
struct Slot{
    pub num:i32,
    pub refer:Rc<RefCell<Object>>
}

struct Object{}

