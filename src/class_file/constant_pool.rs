use std::collections::HashMap;
use crate::class_file::constant_info;
use crate::class_file::constant_info::ConstantInfo;
use crate::class_file::reader::Reader;

pub struct ConstantPool{
    constant_pool:Vec<Box<dyn ConstantInfo>>,
}
const CONSTANT_INFO_MAPPER: HashMap<u8, Box<dyn ConstantInfo>> = HashMap::new();
impl ConstantPool{
    fn new()->ConstantPool{
        //初始化map
        CONSTANT_INFO_MAPPER.insert(constant_info::CONSTANT_INTEGER,Box::new(constant_info::ConstantIntegerInfo::new()));
        ConstantPool{
            constant_pool:vec![],
        }
    }
    fn read_constant_pool(reader:&mut Reader)->ConstantPool{
        let constant_pool_count = reader.read_uint16();
        let mut constant_pool = vec![];
        for i in 1..constant_pool_count{
            constant_pool.push(ConstantPool::read_constant_info(reader));
        }
        ConstantPool{
            constant_pool,
        }
    }
    fn read_constant_info(reader:&mut Reader)->Box<dyn ConstantInfo>{
        let key = reader.read_uint8();
        let mut constant_info = CONSTANT_INFO_MAPPER.get(&key).unwrap().new();
        constant_info.read_info(reader);
        constant_info
    }

}
