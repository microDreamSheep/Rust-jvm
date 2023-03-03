use crate::class_file::reader::Reader;

pub mod reader;

struct ClassFile{
    magic:u32,
    minor_version:u16,
    major_version:u16,
    //constant_pool:ConstantPoll,
    access_flags:u16,
    this_class:u16,
    super_class:u16,
    interfaces:Vec<u16>,
    //fields:Vec<Box<MemberInfo>>,
    //methods:Vec<Box<MemberInfo>>,
    //attributes:Vec<Box<Attribute>>
}
impl ClassFile{
    fn parse(class_data:Vec<u8>)->ClassFile{
        let mut reader = Reader::new(class_data);
        let magic = ClassFile::read_and_check_magic(&mut reader);
        let (minor_version,major_version) = ClassFile::read_and_check_version(&mut reader);
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