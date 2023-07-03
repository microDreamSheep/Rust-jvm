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
    pub fn get_slots(&mut self) ->&mut Vec<Slot>{
        &mut self.slots
    }
    pub fn get_slot(&mut self,index:usize) ->&mut Slot{
        self.slots.get_mut(index).unwrap()
    }

    pub fn push_int(&mut self,value:i32){
        let size = self.size;
        self.get_slot(size as usize).set_num(value);
        self.size += 1;
    }
    pub fn pop_int(&mut self) ->i32{
        self.size -= 1;
        let size = self.size;
        let slot: &Slot = self.get_slots().get(size as usize).unwrap();
        *slot.get_num()
    }
    pub fn push_float(&mut self,value:f32){
        //float to int
        let int = value.to_bits() as i32;
        self.push_int(int);
    }
    pub fn pop_float(&mut self) ->f32{
        let int = self.pop_int() as u32;
        f32::from_bits(int)
    }
    pub fn push_long(&mut self,value:i64){
        let size = self.size;
        self.get_slots().get_mut(size as usize).unwrap().set_num((value & 0xFFFFFFFF) as i32);
        self.get_slots().get_mut(size as usize + 1).unwrap().set_num((value >> 32) as i32);
        self.size += 2;
    }
    pub fn pop_long(&mut self) ->i64{
        let high = self.pop_int() as u32;
        let low = self.pop_int() as u32;
        ((high as i64) << 32) | (low as i64)
    }
    pub fn push_double(&mut self,value:f64){
        //double to long
        let long = value.to_bits() as i64;
        self.push_long(long);
    }
    pub fn pop_double(&mut self) ->f64{
        let long = self.pop_long() as u64;
        f64::from_bits(long)
    }
    pub fn push_ref(&mut self,value:Rc<RefCell<Object>>){
        let size = self.size;
        self.get_slots().get_mut(size as usize).unwrap().set_refer(value);
        self.size += 1;
    }
    pub fn pop_ref(&mut self) ->Rc<RefCell<Object>>{
        self.size -= 1;
        let size = self.size;
        let slot: &Slot = self.get_slots().get(size as usize).unwrap();
        slot.get_refer()
    }
}