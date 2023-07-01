use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::reader::Reader;

pub struct Attribute{

}

impl Attribute {
    pub fn read_attributes(reader: &mut Reader, cp:Rc<RefCell<ConstantPool>>)->Vec<Box<dyn AttributeInfo>>{
        let attributes_count = reader.read_uint16();
        let mut attributes:Vec<Box<dyn AttributeInfo>> = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count{
            attributes.push(get_attributes(reader, &cp));
        }
        attributes
    }

}
pub fn get_attributes(reader: &mut Reader, cp:&Rc<RefCell<ConstantPool>>) ->Box<dyn AttributeInfo>{
    let name_index = reader.read_uint16();
    let name = cp.borrow().get_utf8(&name_index);
    let attribute = get_attribute_info(name, reader, cp);
    attribute
}
fn get_attribute_info(name:String, reader:&mut Reader, cp:&Rc<RefCell<ConstantPool>>) ->Box<dyn AttributeInfo>{
    match name.as_str() {
        "Code" => CodeAttribute::new(name,reader,cp),
        "ConstantValue" => ConstantValueAttribute::new(name,reader,cp),
        "Deprecated" => DeprecatedAttribute::new(name,reader,cp),
        "Exceptions" => ExceptionsAttribute::new(name,reader,cp),
        "LineNumberTable" => LineNumberTableAttribute::new(name,reader,cp),
        "LocalVariableTable" => LocalVariableTableAttribute::new(name,reader,cp),
        "SourceFile" => SourceFileAttribute::new(name,reader,cp),
        "Synthetic" => SyntheticAttribute::new(name,reader,cp),
        _ => UnparsedAttribute::new(name,reader,cp),
    }
}



pub trait AttributeInfo{
    fn new(name:String,reader: &mut Reader,cp:&Rc<RefCell<ConstantPool>>)-> Box<dyn AttributeInfo> where Self:Sized;
    fn to_string(&self)->String;
    fn get_name(&self)->String;
    fn as_any(&self)->&dyn Any;
}
/**
 未知属性
 */
pub struct UnparsedAttribute{
    pub name:String,
    pub length:u64,
    pub info:Vec<u8>,
}
impl AttributeInfo for UnparsedAttribute{

    fn new(name:String,reader: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let name = name;
        let length = reader.read_uint64();
        let info:Vec<u8> = Vec::with_capacity(length as usize);
        Box::new(UnparsedAttribute{
            name,
            length,
            info,
        })
    }


    fn to_string(&self) -> String {
        "unparsed attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/**
 Deprecated属性
 */
pub struct DeprecatedAttribute{
    pub name:String,
}
impl AttributeInfo for DeprecatedAttribute{
    fn new(name:String,_: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        Box::new(DeprecatedAttribute{
            name,
        })
    }

    fn to_string(&self) -> String {
        "deprecated attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/**
 Synthetic属性
 */
pub struct SyntheticAttribute{
    pub name:String,
}
impl AttributeInfo for SyntheticAttribute{
    fn new(name:String,_: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        Box::new(SyntheticAttribute{
            name,
        })
    }

    fn to_string(&self) -> String {
        "synthetic attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
SourceFile属性
 */
pub struct SourceFileAttribute{
    pub name:String,
    pub source_name:String,
}
impl AttributeInfo for SourceFileAttribute{
    fn new(name:String,reader: &mut Reader,cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let _ = reader.read_uint64();
        let source_name_index = reader.read_uint16();
        //通过常量池索引获取常量池中的字符串
        let source_name = cp.borrow().get_utf8(&source_name_index).to_string();

        Box::new(SourceFileAttribute{
            name,
            source_name,
        })
    }


    fn to_string(&self) -> String {
        "source file attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
/**
 ConstantValue属性
 */
pub struct ConstantValueAttribute{
    pub name:String,
    pub constant_value_index:u16,
    pub cp:Rc<RefCell<ConstantPool>>,
}
impl AttributeInfo for ConstantValueAttribute {
    fn new(name: String, reader: &mut Reader, cp: &Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self: Sized {
        let _ = reader.read_uint64();
        let constant_value_index = reader.read_uint16();
        Box::new(ConstantValueAttribute {
            name,
            constant_value_index,
            cp:Rc::clone(cp),
        })
    }

    fn to_string(&self) -> String {
        "constant value attribute".to_string() + &self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/**
 ExceptionEntry
 */
pub struct ExceptionTableEntry{
    pub start_pc:u16,
    pub end_pc:u16,
    pub handler_pc:u16,
    pub catch_type:u16,
}
impl ExceptionTableEntry{

    pub fn new(reader:&mut Reader)->ExceptionTableEntry{
        let start_pc = reader.read_uint16();
        let end_pc = reader.read_uint16();
        let handler_pc = reader.read_uint16();
        let catch_type = reader.read_uint16();
        ExceptionTableEntry{
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

/**
 Code属性
 */
pub struct CodeAttribute{
    pub name:String,
    pub max_stack:u16,
    pub max_locals:u16,
    pub code:Vec<u8>,
    pub exception_table:Vec<ExceptionTableEntry>,
    pub attributes:Vec<Box<dyn AttributeInfo>>,
    pub cp:Rc<RefCell<ConstantPool>>,
}
impl AttributeInfo for CodeAttribute{
    fn new(name:String,reader: &mut Reader,cp:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let _ = reader.read_uint64();
        let max_stack = reader.read_uint16();
        let max_locals = reader.read_uint16();
        let code_length = reader.read_uint32();
        let code = reader.read_bytes(code_length as usize);

        let exception_table_length = reader.read_uint16();
        let mut exception_table:Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length{
            exception_table.push(ExceptionTableEntry::new(reader));
        }
        let attributes = Attribute::read_attributes(reader,Rc::clone(cp));
        Box::new(CodeAttribute{
            name,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
            cp:Rc::clone(cp),
        })
    }

    fn to_string(&self) -> String {
        "code attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/*
Exceptions属性
**/
pub struct ExceptionsAttribute{
    pub name:String,
    pub exception_index_table:Vec<u16>,
}
impl AttributeInfo for ExceptionsAttribute{
    fn new(name:String,reader: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let _ = reader.read_uint64();
        let exception_index_table_length = reader.read_uint16();
        let mut exception_index_table:Vec<u16> = Vec::with_capacity(exception_index_table_length as usize);
        for _ in 0..exception_index_table_length{
            exception_index_table.push(reader.read_uint16());
        }
        Box::new(ExceptionsAttribute{
            name,
            exception_index_table,
        })
    }

    fn to_string(&self) -> String {
        "exceptions attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/**
 LineNumberTableEntry
 */
pub struct LineNumberTableEntry{
    pub start_pc:u16,
    pub line_number:u16,
}
impl LineNumberTableEntry{
    pub fn new(reader: &mut Reader) -> LineNumberTableEntry{
        let start_pc = reader.read_uint16();
        let line_number = reader.read_uint16();
        LineNumberTableEntry{
            start_pc,
            line_number,
        }
    }
}

/**
 LineNumberTable属性
 */
pub struct LineNumberTableAttribute{
    pub name:String,
    pub line_number_table:Vec<LineNumberTableEntry>,
}
impl AttributeInfo for LineNumberTableAttribute{
    fn new(name:String,reader: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let _ = reader.read_uint64();
        let line_number_table_length = reader.read_uint16();
        let mut line_number_table:Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length{
            line_number_table.push(LineNumberTableEntry::new(reader));
        }
        Box::new(LineNumberTableAttribute{
            name,
            line_number_table,
        })
    }

    fn to_string(&self) -> String {
        "line number table attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/*
LocalVariableTableEntry
**/
pub struct LocalVariableTableEntry{
    pub start_pc:u16,
    pub length:u16,
    pub name_index:u16,
    pub descriptor_index:u16,
    pub index:u16,
}
impl LocalVariableTableEntry{
    pub fn new(reader: &mut Reader) -> LocalVariableTableEntry{
        let start_pc = reader.read_uint16();
        let length = reader.read_uint16();
        let name_index = reader.read_uint16();
        let descriptor_index = reader.read_uint16();
        let index = reader.read_uint16();
        LocalVariableTableEntry{
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        }
    }
}
/**
 LocalVariableTable属性
 */
pub struct LocalVariableTableAttribute{
    pub name:String,
    pub local_variable_table:Vec<LocalVariableTableEntry>,
}
impl AttributeInfo for LocalVariableTableAttribute{
    fn new(name:String,reader: &mut Reader,_:&Rc<RefCell<ConstantPool>>) -> Box<dyn AttributeInfo> where Self:Sized {
        let _ = reader.read_uint64();
        let local_variable_table_length = reader.read_uint16();
        let mut local_variable_table:Vec<LocalVariableTableEntry> = Vec::with_capacity(local_variable_table_length as usize);
        for _ in 0..local_variable_table_length{
            local_variable_table.push(LocalVariableTableEntry::new(reader));
        }
        Box::new(LocalVariableTableAttribute{
            name,
            local_variable_table,
        })
    }

    fn to_string(&self) -> String {
        "local variable table attribute".to_string()+&self.name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}