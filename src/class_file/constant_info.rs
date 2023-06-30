use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::class_file::reader::Reader;
use crate::class_file::constant_pool::ConstantPool;

pub trait ConstantInfo {
    fn read_info( &mut self, reader: &mut Reader)->bool;
    fn new(cp:&Rc<RefCell<ConstantPool>>)->Box<dyn ConstantInfo> where Self: Sized;
    fn to_string(&self)->String;
    fn as_any(&self)->&dyn Any;
}

pub struct ConstantInfoMeta{
    pub cp:Rc<RefCell<ConstantPool>>
}
/**
 占位常量
 */
pub struct NullConstant{}

impl ConstantInfo for NullConstant{
    fn read_info(&mut self, _: &mut Reader) -> bool { false }

    fn new(_: &Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> where Self: Sized {
        Box::new(NullConstant{})
    }

    fn to_string(&self) -> String { String::from("i am only a nothing") }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
Integer常量
 */
pub struct ConstantIntegerInfo {
    pub val:i32,
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantIntegerInfo{
    fn read_info(&mut self, reader: &mut Reader) ->bool{
        self.val = reader.read_uint32() as i32;
        false
    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantIntegerInfo{
            val:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
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
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantFloatInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.val = reader.read_uint32() as f32;
        false
    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFloatInfo{
            val:0.0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
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
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantLongInfo{
    fn read_info(&mut self, reader: &mut Reader) ->bool{
        self.val = reader.read_uint64() as i64;
        true
    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantLongInfo{
            val:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
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
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantDoubleInfo{
    fn read_info(&mut self, reader: &mut Reader) ->bool{
        self.val = reader.read_uint64() as f64;
        true

    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantDoubleInfo{
            val:0.0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
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
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantUtf8Info{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        let length = reader.read_uint16();
        let mut bytes = vec![];
        for _ in 0..length{
            bytes.push(reader.read_uint8().clone());
        }
        self.val = String::from_utf8(bytes).unwrap();
        false
    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantUtf8Info{
            val:String::new(),
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
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
pub struct ConstantStringInfo {
    pub string_index: u16,
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantStringInfo{
    fn read_info(&mut self, reader: &mut Reader) ->bool{
        self.string_index = reader.read_uint16();
        false
    }
    fn new(cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantStringInfo{
            string_index:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(cp)
            }
        })
    }

    fn to_string(&self) -> String {
        "ConstantStringInfo:# ".to_owned()+ &*self.string_index.to_string() +"  "+&self.get_value()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantStringInfo {
    fn get_value(&self) ->String{
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.string_index-1));
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
}

/**
 Class常量
 */
pub struct ConstantClassInfo {
    pub name_index:u16,
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantClassInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.name_index = reader.read_uint16();
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantClassInfo{
            name_index:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(constant_pool)
            }
        })
    }

    fn to_string(&self) -> String {
        "ConstantClassInfo:".to_owned()+&self.get_name()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantClassInfo {
    pub(crate) fn get_name(&self) ->String{
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.name_index-1));
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
}
/**
 NameAndType常量
 */
pub struct ConstantNameAndTypeInfo{
    pub name_index:u16,
    pub descriptor_index:u16,
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantNameAndTypeInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.name_index = reader.read_uint16();
        self.descriptor_index = reader.read_uint16();
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantNameAndTypeInfo{
            name_index:0,
            descriptor_index:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(constant_pool)
            }
        })
    }

    fn to_string(&self) -> String {
        "ConstantNameAndTypeInfo:".to_owned()+&self.get_name()+":"+&self.get_descriptor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantNameAndTypeInfo {
    fn get_name(&self) ->String{
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.name_index-1));
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
    fn get_descriptor(&self) ->String{
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.descriptor_index-1));
        constant_info.as_any().downcast_ref::<ConstantUtf8Info>().unwrap().val.clone()
    }
}
/**
 引用常量
 */
pub struct ConstantMemberRefInfo {
    pub class_index:u16,
    pub name_and_type_index:u16,
    pub meta:ConstantInfoMeta,
}
impl ConstantInfo for ConstantMemberRefInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.class_index = reader.read_uint16();
        self.name_and_type_index = reader.read_uint16();
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMemberRefInfo {
            class_index:0,
            name_and_type_index:0,
            meta:ConstantInfoMeta{
                cp:Rc::clone(constant_pool)
            },
        })
    }

    fn to_string(&self) -> String {
        "ConstantMemberRefInfo:".to_owned()+&self.get_class_name()+":"+&self.get_name_and_descriptor().0+":"+&self.get_name_and_descriptor().1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ConstantMemberRefInfo {
    fn get_class_name(&self) ->String{
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.class_index-1));
        constant_info.as_any().downcast_ref::<ConstantClassInfo>().unwrap().get_name()
    }
    fn get_name_and_descriptor(&self) ->(String,String){
        let binding = (&self.meta).cp.deref().borrow();
        let constant_info = binding.get(&(self.name_and_type_index - 1));
        let constant_name_and_type_info = constant_info.as_any().downcast_ref::<ConstantNameAndTypeInfo>().unwrap();
        let name = constant_name_and_type_info.get_name();
        let descriptor = constant_name_and_type_info.get_descriptor();
        (name,descriptor)
    }
}
/**
 MethodRef常量
 */
pub struct ConstantMethodRefInfo {
    pub ref_info: ConstantMemberRefInfo,
}
impl ConstantInfo for ConstantMethodRefInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.ref_info.read_info(reader);
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                meta:ConstantInfoMeta{
                    cp:Rc::clone(constant_pool)
                },
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
pub struct ConstantFieldRefInfo {
    pub ref_info: ConstantMemberRefInfo,
}
impl ConstantInfo for ConstantFieldRefInfo{
    fn read_info(&mut self, reader: &mut Reader)->bool {
        self.ref_info.read_info(reader);
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantFieldRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                meta:ConstantInfoMeta{
                    cp:Rc::clone(constant_pool)
                },
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
pub struct ConstantInterfaceMethodRefInfo{
    pub ref_info: ConstantMemberRefInfo,
}
impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut Reader)->bool{
        self.ref_info.read_info(reader);
        false
    }
    fn new(constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantInterfaceMethodRefInfo {
            ref_info: ConstantMemberRefInfo {
                class_index:0,
                name_and_type_index:0,
                meta:ConstantInfoMeta{
                    cp:Rc::clone(constant_pool)
                },
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

impl ConstantInterfaceMethodRefInfo {
    pub fn get_class_name(&self) ->String{
        self.ref_info.get_class_name()
    }
    pub fn get_name_and_descriptor(&self) ->(String,String){
        self.ref_info.get_name_and_descriptor()
    }
}

/**
 methodHandle常量
 */
pub struct ConstantMethodHandleInfo{
    pub reference_kind: u8,
    pub reference_index: u16,
}
impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut Reader)->bool{
        self.reference_kind = *reader.read_uint8();
        self.reference_index = reader.read_uint16();
        false
    }
    fn new(_constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMethodHandleInfo {
            reference_kind:0,
            reference_index:0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantMethodHandleInfo:".to_owned()+&self.reference_kind.to_string()+":"+&self.reference_index.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 methodType常量
 */
pub struct ConstantMethodTypeInfo{
    pub descriptor_index: u16,
}
impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut Reader)->bool{
        self.descriptor_index = reader.read_uint16();
        false
    }
    fn new(_constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantMethodTypeInfo {
            descriptor_index:0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantMethodTypeInfo:".to_owned()+&self.descriptor_index.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 invokeDynamic常量
 */
pub struct ConstantInvokeDynamicInfo{
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}
impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut Reader)->bool{
        self.bootstrap_method_attr_index = reader.read_uint16();
        self.name_and_type_index = reader.read_uint16();
        false
    }
    fn new(_constant_pool:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
        Box::new(ConstantInvokeDynamicInfo {
            bootstrap_method_attr_index:0,
            name_and_type_index:0,
        })
    }

    fn to_string(&self) -> String {
        "ConstantInvokeDynamicInfo:".to_owned()+&self.bootstrap_method_attr_index.to_string()+":"+&self.name_and_type_index.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
const CONSTANT_METHOD_HANDLE:u8 = 15;
const CONSTANT_METHOD_TYPE:u8 = 16;
const CONSTANT_INVOKE_DYNAMIC:u8 = 18;

pub(crate) fn get(id: &u8, cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn ConstantInfo> {
    match *id {
        CONSTANT_INTEGER => ConstantIntegerInfo::new(cp),
        CONSTANT_FLOAT => ConstantFloatInfo::new(cp),
        CONSTANT_LONG => ConstantLongInfo::new(cp),
        CONSTANT_DOUBLE => ConstantDoubleInfo::new(cp),
        CONSTANT_UTF8 => ConstantUtf8Info::new(cp),
        CONSTANT_STRING => ConstantStringInfo::new(cp),
        CONSTANT_CLASS => ConstantClassInfo::new(cp),
        CONSTANT_FIELD_REF => ConstantFieldRefInfo::new(cp),
        CONSTANT_METHOD_REF => ConstantMethodRefInfo::new(cp),
        CONSTANT_INTERFACE_METHOD_REF => ConstantInterfaceMethodRefInfo::new(cp),
        CONSTANT_NAME_AND_TYPE => ConstantNameAndTypeInfo::new(cp),
        CONSTANT_METHOD_HANDLE => ConstantMethodHandleInfo::new(cp),
        CONSTANT_METHOD_TYPE => ConstantMethodTypeInfo::new(cp),
        CONSTANT_INVOKE_DYNAMIC => ConstantInvokeDynamicInfo::new(cp),

        _ => {
            panic!("java.lang.ClassFormatError: constant pool tag! id = {}",id)
        }
    }
}