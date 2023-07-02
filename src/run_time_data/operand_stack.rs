use std::cell::RefCell;
use std::rc::Rc;
use crate::run_time_data::Object;
use crate::run_time_data::slot::Slot;

pub struct OperandStack{
    pub size:u32,
    slots:Vec<Slot>,
}
impl OperandStack{
    pub fn new(max_stack:u32)->OperandStack{
        let mut vec = Vec::with_capacity(max_stack as usize);
        for _ in 0..max_stack{
            vec.push(Slot::new());
        }
        OperandStack{
            size: 0,
            slots: vec,
        }
    }
    pub fn push_int(&mut self,value:i32){
        let mut slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        slot.set_num(value);
        self.size += 1;
    }
    pub fn pop_int(&mut self)->i32{
        self.size -= 1;
        let slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        *slot.get_num()
    }
    pub fn push_float(&mut self,value:f32){
        let mut slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        slot.set_num(value as i32);
        self.size += 1;
    }
    pub fn pop_float(&mut self)->f32{
        self.size -= 1;
        let slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        *slot.get_num() as f32
    }
    pub fn push_long(&mut self,value:i64){
        let mut slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        slot.set_num(value as i32);
        self.size += 2;
    }
    pub fn pop_long(&mut self)->i64{
        self.size -= 2;
        let slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        *slot.get_num() as i64
    }
    pub fn push_double(&mut self,value:f64){
        let mut slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        slot.set_num(value as i32);
        self.size += 2;
    }
    pub fn pop_double(&mut self)->f64{
        self.size -= 2;
        let slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        *slot.get_num() as f64
    }
    pub fn push_ref(&mut self,value:Rc<RefCell<Object>>){
        let mut slot: &Slot = (&self.slots).get(self.size as usize).unwrap();
        slot.set_refer(value);
        self.size += 1;
    }

}