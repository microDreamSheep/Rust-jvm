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
        //float to int
        let int = value.to_bits() as i32;
        self.set_int(index,int);
    }
    pub fn get_float(&mut self,index:u32)->f32{
        let int = self.get_int(index) as u32;
        f32::from_bits(int)
    }
    pub fn set_long(&mut self,index:u32,value:i64){
        let low = (value & 0xFFFFFFFF) as i32;
        let high = (value >> 32) as i32;
        self.set_int(index,low);
        self.set_int(index + 1,high);
    }
    pub fn get_long(&mut self,index:u32)->i64{
        let low = self.get_int(index) as u32;
        let high = self.get_int(index + 1) as u32;
        ((high as i64) << 32) | (low as i64)
    }
    pub fn set_double(&mut self,index:u32,value:f64){
        //double to long
        let long = value.to_bits() as i64;
        self.set_long(index,long);
    }
    pub fn get_double(&mut self,index:u32)->f64{
        let long = self.get_long(index) as u64;
        f64::from_bits(long)
    }
    pub fn set_ref(&mut self,index:u32,value:Option<Rc<RefCell<Object>>>){
        self.get_slots().get_mut(index as usize).unwrap().set_refer(value);
    }
    pub fn get_ref(&mut self, index:u32) ->Option<Rc<RefCell<Object>>>{
        let slot: &Slot = self.get_slots().get(index as usize).unwrap();
        slot.get_refer()
    }
}
