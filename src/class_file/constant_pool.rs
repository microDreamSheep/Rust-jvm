use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::class_file::constant_info;
use crate::class_file::constant_info::{ConstantInfo, ConstantUtf8Info, NullConstant};
use crate::class_file::reader::Reader;

pub struct ConstantPool{
    pub constant_pool:Vec<Box<dyn ConstantInfo>>,
}
impl ConstantPool{
    fn new()->ConstantPool{
        ConstantPool{
            constant_pool:vec![],
        }
    }
    pub fn read_constant_pool(reader:&mut Reader)->Rc<RefCell<ConstantPool>>{
        let constant_pool_count = reader.read_uint16();
        let pool = ConstantPool::new();
        let arc = Rc::new(RefCell::new(pool));
        for _ in 1..constant_pool_count-1{
            ConstantPool::read_constant_info(reader,&arc);
        }
        arc
    }
    fn read_constant_info(reader:&mut Reader, cp: &Rc<RefCell<ConstantPool>>){
        let key = reader.read_uint8();
        let mut constant_info = constant_info::get(&key,cp);
        let is = constant_info.read_info(reader);
        cp.borrow_mut().constant_pool.push(constant_info);
        if is {
            cp.borrow_mut().constant_pool.push(Box::new(NullConstant{}))
        }
    }
    pub fn get_utf8(&self, index: &u16) -> String {
        let utf8 = self.get(&(index-1)).as_any().downcast_ref::<ConstantUtf8Info>().unwrap();
        utf8.val.clone()
    }

    pub(crate) fn get(&self, index: &u16) ->&Box<dyn ConstantInfo>{
        &self.constant_pool.get(*index as usize).unwrap()
    }
}
