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
    pub fn read_constant_pool(reader:&mut Reader)->ConstantPool{
        let constant_pool_count = reader.read_uint16();
        let mut constant_pool = vec![];
        let mut pool = ConstantPool{
            constant_pool
        };
        for _ in 1..constant_pool_count{
            pool.constant_pool.push(ConstantPool::read_constant_info(reader,&pool));
        }
        pool
    }
    fn read_constant_info(reader:&mut Reader,cp:&ConstantPool)->Box<dyn ConstantInfo>{
        let key = reader.read_uint8();
        let mut constant_info = constant_info::CONSTANT_INFO_MAPPER.get(&key).unwrap().new();
        constant_info.read_info(reader,cp);
        constant_info
    }
    pub(crate) fn get(&self, index: &u16) ->&Box<dyn ConstantInfo>{
        &self.constant_pool.get(*index as usize).unwrap()
    }
}
