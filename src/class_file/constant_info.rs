use std::collections::HashMap;
use crate::class_file::reader::Reader;
use crate::class_file::constant_pool::ConstantPool;

pub trait ConstantInfo{
    fn read_info(&mut self,reader:&mut Reader);
    fn new(&self,cp:&ConstantPool)->Box<dyn ConstantInfo>;
}
const CONSTANT_INTEGER:u8 = 3;
const CONSTANT_FLOAT:u8 = 4;
const CONSTANT_LONG:u8 = 5;
const CONSTANT_DOUBLE:u8 = 6;
const CONSTANT_UTF8:u8 = 1;
const CONSTANT_STRING:u8 = 8;
const CONSTANT_CLASS:u8 = 7;
const CONSTANT_FIELD_REF:u8 = 9;
const CONSTANT_METHOD_REF:u8 = 10;
const CONSTANT_INTERFACE_METHOD_REF:u8 = 11;
const CONSTANT_NAME_AND_TYPE:u8 = 12;
//const CONSTANT_METHOD_HANDLE:u8 = 15;
//const CONSTANT_METHOD_TYPE:u8 = 16;
//const CONSTANT_INVOKE_DYNAMIC:u8 = 18;


/**
 Integer常量
 */
pub struct ConstantIntegerInfo {
    pub val:i32,
}
impl ConstantInfo for ConstantIntegerInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.val = reader.read_uint32() as i32;
    }
    fn new(&self,_:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantIntegerInfo{
            val:0,
        })
    }
}
/**
 Float常量
 */
pub struct ConstantFloatInfo {
    pub val:f32,
}
impl ConstantInfo for ConstantFloatInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.val = reader.read_uint32() as f32;
    }
    fn new(&self,_:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFloatInfo{
            val:0.0,
        })
    }
}
/**
 Long常量
 */
pub struct ConstantLongInfo {
    pub val:i64,
}
impl ConstantInfo for ConstantLongInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.val = reader.read_uint64() as i64;
    }
    fn new(&self,_:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantLongInfo{
            val:0,
        })
    }
}
/**
 Double常量
 */
pub struct ConstantDoubleInfo {
    pub val:f64,
}
impl ConstantInfo for ConstantDoubleInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.val = reader.read_uint64() as f64;
    }
    fn new(&self,_:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantDoubleInfo{
            val:0.0,
        })
    }
}
/**
 Utf8常量
 */
pub struct ConstantUtf8Info {
    pub val:String,
}
impl ConstantInfo for ConstantUtf8Info{
    fn read_info(&mut self, reader: &mut Reader) {
        let length = reader.read_uint16();
        let mut bytes = vec![];
        for _ in 0..length{
            bytes.push(reader.read_uint8().clone());
        }
        self.val = String::from_utf8(bytes).unwrap();
    }
    fn new(&self,_:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantUtf8Info{
            val:String::new(),
        })
    }
}
/**
 String常量
 */
pub struct ConstantStringInfo<'a> {
    pub string_index:u16,
    pub cp:&'a ConstantPool,
}

impl ConstantInfo for ConstantStringInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.string_index = reader.read_uint16();
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantStringInfo{
            string_index:0,
            cp,
        })
    }
}

impl ConstantStringInfo {
    fn get_value(&self) ->String{
        let constant_info = self.cp.get(&self.string_index).unwrap();
        let constant_utf8_info = constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap();
        constant_utf8_info.val.clone()
    }
}
/**
 Class常量
 */
pub struct ConstantClassInfo<'a> {
    pub name_index:u16,
    pub cp:&'a ConstantPool,
}
impl ConstantInfo for ConstantClassInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.name_index = reader.read_uint16();
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantClassInfo{
            name_index:0,
            cp,
        })
    }
}
impl ConstantClassInfo {
    fn get_name(&self) ->String{
        let constant_info = self.cp.get(&self.name_index).unwrap();
        let constant_utf8_info = constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap();
        constant_utf8_info.val.clone()
    }
}
/**
 NameAndType常量
 */
pub struct ConstantNameAndTypeInfo<'a> {
    pub name_index:u16,
    pub descriptor_index:u16,
    pub cp:&'a ConstantPool,
}
impl ConstantInfo for ConstantNameAndTypeInfo{
    fn read_info(&mut self, reader: &mut Reader) {
        self.name_index = reader.read_uint16();
        self.descriptor_index = reader.read_uint16();
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantNameAndTypeInfo{
            name_index:0,
            descriptor_index:0,
            cp,
        })
    }
}
/**
 引用常量
 */
pub struct ConstantMemberRefInfo<'a> {
    pub class_index:u16,
    pub name_and_type_index:u16,
    pub cp:&'a ConstantPool,
}
impl ConstantInfo for ConstantMemberRefInfo {
    fn read_info(&mut self, reader: &mut Reader) {
        self.class_index = reader.read_uint16();
        self.name_and_type_index = reader.read_uint16();
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMemberRefInfo {
            class_index:0,
            name_and_type_index:0,
            cp,
        })
    }
}
impl ConstantMemberRefInfo {
    fn get_class_name(&self) ->String{
        let constant_info = self.cp.get(&self.class_index).unwrap();
        let constant_class_info = constant_info.as_any().downcast_ref::<ConstantClassInfo>().unwrap();
        constant_class_info.get_name()
    }
    fn get_name_and_descriptor(&self) ->(String,String){
        let constant_info = self.cp.get(&self.name_and_type_index).unwrap();
        let constant_name_and_type_info = constant_info.as_any().downcast_ref::<ConstantNameAndTypeInfo>().unwrap();
        let name = constant_name_and_type_info.get_name();
        let descriptor = constant_name_and_type_info.get_descriptor();
        (name,descriptor)
    }
}
/**
 MethodRef常量
 */
pub struct ConstantMethodRefInfo<'a> {
    pub ref_info: ConstantMemberRefInfo<'a>,
}
impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut Reader) {
        self.ref_info.read_info(reader);
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp,
            },
        })
    }
}
impl ConstantMethodRefInfo {
    pub fn get_class_name(&self) ->String{
        self.ref_info.get_class_name()
    }
    pub fn get_name_and_descriptor(&self) ->(String,String){
        self.ref_info.get_name_and_descriptor()
    }
}
/**
 FieldRef常量
 */
pub struct ConstantFieldRefInfo<'a> {
    pub ref_info: ConstantMemberRefInfo<'a>,
}
impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut Reader) {
        self.ref_info.read_info(reader);
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFieldRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp,
            },
        })
    }
}
impl ConstantFieldRefInfo {
    pub fn get_class_name(&self) ->String{
        self.ref_info.get_class_name()
    }
    pub fn get_name_and_descriptor(&self) ->(String,String){
        self.ref_info.get_name_and_descriptor()
    }
}
/**
 InterfaceMethodRef常量
 */
pub struct ConstantInterfaceMethodRefInfo<'a> {
    pub ref_info: ConstantMemberRefInfo<'a>,
}
impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut Reader) {
        self.ref_info.read_info(reader);
    }
    fn new(&self,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
        Box::new(ConstantInterfaceMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp,
            },
        })
    }
}
impl ConstantInterfaceMethodRefInfo {
    pub fn get_class_name(&self) ->String{
        self.ref_info.get_class_name()
    }
    pub fn get_name_and_descriptor(&self) ->(String,String){
        self.ref_info.get_name_and_descriptor()
    }
}