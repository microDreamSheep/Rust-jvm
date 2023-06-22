use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::constant_info;
use crate::class_file::constant_info::ConstantInfo;
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
        let constant_pool = vec![];
        let pool = ConstantPool{
            constant_pool
        };
        let mut arc = Rc::new(RefCell::new(pool));
        for _ in 1..constant_pool_count-1{
            arc.borrow_mut().constant_pool.push(ConstantPool::read_constant_info(reader,&arc));
        }
        arc
    }
    fn read_constant_info(reader:&mut Reader, cp: &Rc<RefCell<ConstantPool>>) ->Box<dyn ConstantInfo>{
        let key = reader.read_uint8();
        let mut constant_info = constant_info::get(&key,cp);
        constant_info.read_info(reader);
        constant_info
    }
    pub(crate) fn get(&self, index: &u16) ->&Box<dyn ConstantInfo>{
        &self.constant_pool.get(*index as usize).unwrap()
    }
}
