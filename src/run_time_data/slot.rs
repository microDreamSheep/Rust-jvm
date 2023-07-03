use std::cell::RefCell;
use std::rc::Rc;
use crate::run_time_data::Object;

pub struct Slot{
    pub num:i32,
    pub refer: Option<Rc<RefCell<Object>>>
}

impl Slot {
    pub fn new()->Slot{
        Slot{
            num: 0,
            refer: None,
        }
    }
    pub fn set_num(&mut self, value:i32){
        self.num = value;
    }
    pub fn set_refer(&mut self, value:Option<Rc<RefCell<Object>>>){
        self.refer = value;
    }
    pub fn get_num(&self)->&i32{
        &self.num
    }
    pub fn get_refer(&self)->Option<Rc<RefCell<Object>>>{
        Some(Rc::clone(self.refer.as_ref().unwrap()))
    }
}
