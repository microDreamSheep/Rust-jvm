use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use crate::class_file::reader::Reader;
use crate::class_file::constant_pool::ConstantPool;

pub trait ConstantInfo {
    fn read_info(&mut self,reader:&mut Reader,constant_pool:&ConstantPool);
    fn new(&self)->Box<dyn ConstantInfo>;
    fn to_string(&self)->String;
    fn as_any(&self)->&dyn Any;
}

const NULL_CP: ConstantPool = ConstantPool{
    constant_pool:vec![],
};

/**
Integer常量
 */
pub struct ConstantIntegerInfo {
    pub val:i32,
}
impl ConstantInfo for ConstantIntegerInfo{
    fn read_info(&mut self, reader: &mut Reader,_:&ConstantPool) {
        self.val = reader.read_uint32() as i32;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantIntegerInfo{
            val:0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantIntegerInfo:".to_owned()+&self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 Float常量
 */
pub struct ConstantFloatInfo {
    pub val:f32,
}
impl ConstantInfo for ConstantFloatInfo{
    fn read_info(&mut self, reader: &mut Reader,_:&ConstantPool) {
        self.val = reader.read_uint32() as f32;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFloatInfo{
            val:0.0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantFloatInfo:".to_owned()+&self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 Long常量
 */
pub struct ConstantLongInfo {
    pub val:i64,
}
impl ConstantInfo for ConstantLongInfo{
    fn read_info(&mut self, reader: &mut Reader,_:&ConstantPool) {
        self.val = reader.read_uint64() as i64;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantLongInfo{
            val:0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantLongInfo:".to_owned()+&self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 Double常量
 */
pub struct ConstantDoubleInfo {
    pub val:f64,
}
impl ConstantInfo for ConstantDoubleInfo{
    fn read_info(&mut self, reader: &mut Reader,_:&ConstantPool) {
        self.val = reader.read_uint64() as f64;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantDoubleInfo{
            val:0.0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantDoubleInfo:".to_owned()+&self.val.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 Utf8常量
 */
pub struct ConstantUtf8Info {
    pub val:String,
}
impl ConstantInfo for ConstantUtf8Info{
    fn read_info(&mut self, reader: &mut Reader,_:&ConstantPool) {
        let length = reader.read_uint16();
        let mut bytes = vec![];
        for _ in 0..length{
            bytes.push(reader.read_uint8().clone());
        }
        self.val = String::from_utf8(bytes).unwrap();
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantUtf8Info{
            val:String::new(),
        })
    }

    fn to_string(&self) -> String {
        "ConstantUtf8Info:".to_owned()+&self.val.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 String常量
 */
pub struct ConstantStringInfo<'a> {
    pub string_index: u16,
    pub cp: &'a ConstantPool,
}
impl ConstantInfo for ConstantStringInfo<'_>{
    fn read_info(&mut self, reader: &mut Reader,constant_pool:&ConstantPool) {
        self.string_index = reader.read_uint16();
        self.cp = constant_pool;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantStringInfo{
            string_index:0,
            cp: &NULL_CP,
        })
    }

    fn to_string(&self) -> String {
        "ConstantStringInfo:".to_owned()+&self.get_value()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantStringInfo<'_> {
    fn get_value(&self) ->String{
        let constant_info = self.cp.get(&self.string_index);
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
}
/**
 Class常量
 */
pub struct ConstantClassInfo<'a> {
    pub name_index:u16,
    pub cp:&'a ConstantPool,
}
impl ConstantInfo for ConstantClassInfo<'_>{
    fn read_info(&mut self, reader: &mut Reader,constant_pool:&ConstantPool) {
        self.name_index = reader.read_uint16();
        self.cp = constant_pool;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantClassInfo{
            name_index:0,
            cp: &NULL_CP,
        })
    }

    fn to_string(&self) -> String {
        "ConstantClassInfo:".to_owned()+&self.get_name()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantClassInfo<'_> {
    fn get_name(&self) ->String{
        let constant_info = self.cp.get(&self.name_index);
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
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
impl ConstantInfo for ConstantNameAndTypeInfo<'_>{
    fn read_info(&mut self, reader: &mut Reader,constant_pool:&ConstantPool) {
        self.name_index = reader.read_uint16();
        self.descriptor_index = reader.read_uint16();
        self.cp = constant_pool;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantNameAndTypeInfo{
            name_index:0,
            descriptor_index:0,
            cp: &NULL_CP,
        })
    }

    fn to_string(&self) -> String {
        "ConstantNameAndTypeInfo:".to_owned()+&self.get_name()+":"+&self.get_descriptor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantNameAndTypeInfo<'_> {
    fn get_name(&self) ->String{
        let constant_info = self.cp.get(&self.name_index);
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
    fn get_descriptor(&self) ->String{
        let constant_info = self.cp.get(&self.descriptor_index);
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
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
impl ConstantInfo for ConstantMemberRefInfo<'_>{
    fn read_info(&mut self, reader: &mut Reader,constant_pool:&ConstantPool) {
        self.class_index = reader.read_uint16();
        self.name_and_type_index = reader.read_uint16();
        self.cp = constant_pool;
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMemberRefInfo {
            class_index:0,
            name_and_type_index:0,
            cp: &NULL_CP,
        })
    }

    fn to_string(&self) -> String {
        "ConstantMemberRefInfo:".to_owned()+&self.get_class_name()+":"+&self.get_name_and_descriptor().0+":"+&self.get_name_and_descriptor().1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantMemberRefInfo<'_> {
    fn get_class_name(&self) ->String{
        let constant_info = self.cp.get(&self.class_index);
        constant_info.as_any().downcast_ref::<ConstantClassInfo>().unwrap().get_name()
    }
    fn get_name_and_descriptor(&self) ->(String,String){
        let constant_info = self.cp.get(&self.name_and_type_index);
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
impl ConstantInfo for ConstantMethodRefInfo<'_>{
    fn read_info(&mut self, reader: &mut Reader,cp:&ConstantPool) {
        self.ref_info.read_info(reader,cp);
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp: &NULL_CP,
            },
        })
    }

    fn to_string(&self) -> String {
        "ConstantMethodRefInfo:".to_owned() + &self.get_class_name()+":"+&self.get_name_and_descriptor().0+":"+&self.get_name_and_descriptor().1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantMethodRefInfo<'_> {
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
impl ConstantInfo for ConstantFieldRefInfo<'_> {
    fn read_info(&mut self, reader: &mut Reader,cp:&ConstantPool) {
        self.ref_info.read_info(reader,cp);
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFieldRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp: &NULL_CP,
            },
        })
    }

    fn to_string(&self) -> String {
        "ConstantFieldRefInfo:".to_owned() +&self.get_class_name()+":"+&self.get_name_and_descriptor().0+":"+&self.get_name_and_descriptor().1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantFieldRefInfo<'_> {
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
impl ConstantInfo for ConstantInterfaceMethodRefInfo<'_> {
    fn read_info(&mut self, reader: &mut Reader,cp:&ConstantPool) {
        self.ref_info.read_info(reader,cp);
    }
    fn new(&self) -> Box<dyn ConstantInfo> {
        Box::new(ConstantInterfaceMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                cp: &NULL_CP,
            },
        })
    }

    fn to_string(&self) -> String {
        "ConstantInterfaceMethodRefInfo:".to_owned()+&self.get_class_name()+":"+&self.get_name_and_descriptor().0+":"+&self.get_name_and_descriptor().1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConstantInterfaceMethodRefInfo<'_> {
    pub fn get_class_name(&self) ->String{
        self.ref_info.get_class_name()
    }
    pub fn get_name_and_descriptor(&self) ->(String,String){
        self.ref_info.get_name_and_descriptor()
    }
}


pub const CONSTANT_INTEGER:u8 = 3;
pub const CONSTANT_FLOAT:u8 = 4;
pub const CONSTANT_LONG:u8 = 5;
pub const CONSTANT_DOUBLE:u8 = 6;
pub const CONSTANT_UTF8:u8 = 1;
pub const CONSTANT_STRING:u8 = 8;
pub const CONSTANT_CLASS:u8 = 7;
pub const CONSTANT_FIELD_REF:u8 = 9;
pub const CONSTANT_METHOD_REF:u8 = 10;
pub const CONSTANT_INTERFACE_METHOD_REF:u8 = 11;
pub const CONSTANT_NAME_AND_TYPE:u8 = 12;
//const CONSTANT_METHOD_HANDLE:u8 = 15;
//const CONSTANT_METHOD_TYPE:u8 = 16;
//const CONSTANT_INVOKE_DYNAMIC:u8 = 18;


pub const CONSTANT_INFO_MAPPER: HashMap<u8, Box<dyn ConstantInfo>> = {
    let mut map:HashMap<_, Box<(dyn ConstantInfo + 'static)>> = HashMap::new();
    map.insert(CONSTANT_INTEGER, Box::new(ConstantIntegerInfo{ val: 0 }));
    map.insert(CONSTANT_FLOAT, Box::new(ConstantFloatInfo{ val: 0.0 }));
    map.insert(CONSTANT_LONG, Box::new(ConstantLongInfo{ val: 0 }));
    map.insert(CONSTANT_DOUBLE, Box::new(ConstantDoubleInfo{ val: 0.0 }));
    map.insert(CONSTANT_UTF8, Box::new(ConstantUtf8Info{ val: "".to_owned() }));
    map.insert(CONSTANT_STRING, Box::new(ConstantStringInfo{ string_index: 0,cp: &NULL_CP }));
    map.insert(CONSTANT_CLASS, Box::new(ConstantClassInfo{ name_index: 0,cp: &NULL_CP }));
    map.insert(CONSTANT_FIELD_REF, Box::new(ConstantFieldRefInfo{ ref_info: ConstantMemberRefInfo{ class_index: 0,name_and_type_index: 0,cp: &NULL_CP } }));
    map.insert(CONSTANT_METHOD_REF, Box::new(ConstantMethodRefInfo{ ref_info: ConstantMemberRefInfo{ class_index: 0,name_and_type_index: 0,cp: &NULL_CP } }));
    map.insert(CONSTANT_INTERFACE_METHOD_REF, Box::new(ConstantInterfaceMethodRefInfo{ ref_info: ConstantMemberRefInfo{ class_index: 0,name_and_type_index: 0,cp: &NULL_CP } }));
    map.insert(CONSTANT_NAME_AND_TYPE, Box::new(ConstantNameAndTypeInfo{ name_index: 0,descriptor_index: 0,cp: &NULL_CP }));
    map
};
