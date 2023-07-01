use std::cell::RefCell;
use std::rc::Rc;
use crate::class_file::attribute::{Attribute, AttributeInfo};
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::reader::Reader;

pub struct MemberInfo {
    pub cp:Rc<RefCell<ConstantPool>>,
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Box<dyn AttributeInfo>>,
}
impl MemberInfo{
    pub(crate) fn read_members(reader: &mut Reader, cp: Rc<RefCell<ConstantPool>>) -> Vec<Box<MemberInfo>> {
        let member_count = reader.read_uint16();
        let mut members = vec![];
        for _ in 0..member_count {
            members.push(MemberInfo::read_member(reader, &cp));
        }
        members
    }
    fn read_member(reader: &mut Reader, cp: &Rc<RefCell<ConstantPool>>) -> Box<MemberInfo> {
        let access_flags = reader.read_uint16();
        let name_index = reader.read_uint16();
        let descriptor_index = reader.read_uint16();
        let attributes = Attribute::read_attributes(reader, Rc::clone(cp));
        Box::new(MemberInfo {
            cp:Rc::clone(cp),
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
    pub fn get_name(&self) -> String {
        self.cp.borrow().get_utf8(&self.name_index)
    }
    pub fn get_descriptor(&self) -> String {
        self.cp.borrow().get_utf8(&self.descriptor_index)
    }
}