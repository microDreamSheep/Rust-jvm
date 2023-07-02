use std::cell::RefCell;
use std::rc::Rc;
use crate::run_time_data::Object;
use crate::run_time_data::slot::Slot;

pub struct LocalVars{
    pub slots:Vec<Slot>
}
impl LocalVars {
    pub fn new(max_locals:u32) ->LocalVars{
        let mut vec = Vec::with_capacity(max_locals as usize);

        //初始化
        for _ in 0..max_locals{
            vec.push(Slot::new());
        }
        LocalVars{
            slots: vec,
        }
    }
    pub fn get_slots(&mut self) ->&mut Vec<Slot>{
        &mut self.slots
    }
    pub fn set_int(&mut self,index:u32,value:i32){
        self.get_slots().get_mut(index as usize).unwrap().set_num(value);
    }
    pub fn get_int(&mut self, index:u32) ->i32{
        let slot: &Slot = self.get_slots().get(index as usize).unwrap();
        *slot.get_num()
    }
    pub fn set_float(&mut self,index:u32,value:f32){
        self.get_slots().get_mut(index as usize).unwrap().set_num(value as i32);
    }
    pub fn get_float(&mut self,index:u32)->f32{
        let slot: &Slot = self.get_slots().get(index as usize).unwrap();
        *slot.get_num() as f32
    }
    pub fn set_long(&mut self,index:u32,value:i64){
        self.get_slots().get_mut(index as usize).unwrap().set_num(value as i32);
    }
    pub fn get_long(&mut self,index:u32)->i64{
        let slots = self.get_slots();
        let high = slots.get(index as usize).unwrap().get_num();
        let low = slots.get(index as usize + 1).unwrap().get_num();
        (((*high as i64) << 32) | (*low as i64)) as i64
    }
    pub fn set_double(&mut self,index:u32,value:f64){
        self.get_slots().get_mut(index as usize).unwrap().set_num(value as i32);
    }
    pub fn get_double(&mut self, index:u32) ->f64{
        let slots = self.get_slots();
        let high = slots.get(index as usize).unwrap().get_num();
        let low = slots.get(index as usize + 1).unwrap().get_num();
        (((*high as i64) << 32) | (*low as i64)) as f64
    }
    pub fn set_ref(&mut self,index:u32,value:Rc<RefCell<Object>>){
        self.get_slots().get_mut(index as usize).unwrap().set_refer(value);
    }
    pub fn get_ref(&mut self, index:u32) ->Rc<RefCell<Object>>{
        let slot: &Slot = self.get_slots().get(index as usize).unwrap();
        Rc::clone( &slot.get_refer())
    }
}
