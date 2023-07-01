use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::attribute::{Attribute, AttributeInfo};
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::reader::Reader;

pub struct MethodInfo {
    pub cp:Rc<RefCell<ConstantPool>>,
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Box<dyn AttributeInfo>>,
}
impl MethodInfo {
    pub(crate) fn read_methods(reader: &mut Reader, cp: Rc<RefCell<ConstantPool>>) -> Vec<Box<MethodInfo>> {
        let method_count = reader.read_uint16();
        let mut members = vec![];
        for _ in 0..method_count {
            members.push(MethodInfo::read_method(reader, &cp));
        }
        members
    }
    fn read_method(reader: &mut Reader, cp: &Rc<RefCell<ConstantPool>>) -> Box<MethodInfo> {
        let pool = Rc::clone(cp);
        Box::new(MethodInfo {
            cp:Rc::clone(cp),
            access_flags: reader.read_uint16(),
            name_index: reader.read_uint16(),
            descriptor_index: reader.read_uint16(),
            attributes: Attribute::read_attributes(reader, pool),
        })
    }
    pub fn get_name(&self) -> String {
        self.cp.borrow().get_utf8(&self.name_index)
    }
    pub fn get_descriptor(&self) -> String {
        self.cp.borrow().get_utf8(&self.descriptor_index)
    }
}