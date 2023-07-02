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
    pub fn set_int(&self,index:u32,value:i32){
        let mut slot: &mut Slot = &mut self.slots.get(index as usize).unwrap();
        slot.set_num(value);
    }
    pub fn get_int(&self,index:u32)->i32{
        let slot:&Slot = &(&self.slots).get(index as usize).unwrap();
        *slot.get_num()
    }
    pub fn set_float(&self,index:u32,value:f32){
        let mut slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        slot.set_num(value as i32);
    }
    pub fn get_float(&self,index:u32)->f32{
        let slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        *slot.get_num() as f32
    }
    pub fn set_long(&self,index:u32,value:i64){
        let mut slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        slot.set_num(value as i32);
    }
    pub fn get_long(&self,index:u32)->i64{
        let slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        *slot.get_num() as i64
    }
    pub fn set_double(&self,index:u32,value:f64){
        let mut slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        slot.set_num(value as i32);
    }
    pub fn get_double(&self,index:u32)->f64{
        let slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        *slot.get_num() as f64
    }
    pub fn set_ref(&self,index:u32,value:Rc<RefCell<Object>>){
        let mut slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        slot.set_refer(value);
    }
    pub fn get_ref(&self,index:u32)->Rc<RefCell<Object>>{
        let slot:&mut Slot = &mut (&self.slots).get(index as usize).unwrap();
        slot.get_refer().clone()
    }
}
