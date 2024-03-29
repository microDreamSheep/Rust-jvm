use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::constant_info;
use crate::class_file::constant_info::{ConstantInfo, ConstantUtf8Info, NullConstant};
use crate::class_file::reader::ByteCodeReader;

pub struct ConstantPool{
    pub constant_pool:Vec<Box<dyn ConstantInfo>>,
}
impl ConstantPool{
    fn new()->ConstantPool{
        ConstantPool{
            constant_pool:vec![],
        }
    }
    pub fn read_constant_pool(reader:&mut ByteCodeReader) ->Rc<RefCell<ConstantPool>>{
        let constant_pool_count = reader.read_uint16();
        let pool = ConstantPool::new();
        let arc = Rc::new(RefCell::new(pool));
        let mut is_dou = false;
        for _ in 0..constant_pool_count-1{
            if !is_dou {
                is_dou = 1 == ConstantPool::read_constant_info(reader, &arc);

                continue;
            }
            is_dou = false;
        }
        arc
    }
    fn read_constant_info(reader:&mut ByteCodeReader, cp: &Rc<RefCell<ConstantPool>>) ->u16{
        let key = reader.read_uint8();
        let mut constant_info = constant_info::get(&key,cp);
        let is = constant_info.read_info(reader);
        cp.borrow_mut().constant_pool.push(constant_info);
        if is {
            cp.borrow_mut().constant_pool.push(Box::new(NullConstant{}));
            return  1;
        }
        return 0;
    }
    pub fn get_utf8(&self, index: &u16) -> String {
        let utf8 = self.get(&(index-1)).as_any().downcast_ref::<ConstantUtf8Info>().unwrap();
        utf8.val.clone()
    }

    pub(crate) fn get(&self, index: &u16) ->&Box<dyn ConstantInfo>{
        &self.constant_pool.get(*index as usize).unwrap()
    }
}
