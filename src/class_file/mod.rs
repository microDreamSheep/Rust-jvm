use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::reader::Reader;

pub mod reader;
pub mod constant_pool;
pub mod constant_info;

pub struct ClassFile{
    pub magic:u32,
    pub minor_version:u16,
    pub major_version:u16,
    pub constant_pool:Rc<RefCell<ConstantPool>>,
    pub access_flags:u16,
    pub this_class:u16,
    pub super_class:u16,
    pub interfaces:Vec<u16>,
    //pub fields:Vec<Box<MemberInfo>>,
    //pub methods:Vec<Box<MemberInfo>>,
    //pub attributes:Vec<Box<Attribute>>
}
impl ClassFile{
    pub(crate) fn parse(class_data:Vec<u8>) ->ClassFile{
        let mut reader = Reader::new(class_data);
        let magic = ClassFile::read_and_check_magic(&mut reader);
        let (minor_version,major_version) = ClassFile::read_and_check_version(&mut reader);
        let constant_pool = ConstantPool::read_constant_pool(&mut reader);
        let access_flags = reader.read_uint16();
        let this_class = reader.read_uint16();
        let super_class = reader.read_uint16();
        //let interfaces = ClassFile::read_interfaces(&mut reader);
        //let fields = ClassFile::read_members(&mut reader);
        //let methods = ClassFile::read_members(&mut reader);
        //let attributes = ClassFile::read_attributes(&mut reader);
        ClassFile{
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces: vec![],
        }

    }
    fn read_and_check_magic(reader:&mut Reader)->u32{
        let magic = reader.read_uint32();
        if magic!=0xCAFEBABE {
            panic!("magic number ill")
        }
        return magic;
    }
    fn read_and_check_version(reader: &mut Reader) ->(u16, u16){
        let minor_version = reader.read_uint16();
        let major_version = reader.read_uint16();
        if major_version>=45&&major_version<=52{
            if (major_version>45&&minor_version==0)||(major_version==45) {
                return (minor_version,major_version);
            }
        }
        panic!("java.lang.UnsupportedClassVersionError!");
    }
}