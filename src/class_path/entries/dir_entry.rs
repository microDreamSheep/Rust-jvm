use std::fs;
use std::path::{MAIN_SEPARATOR};
use crate::class_path::Entry;

pub struct DirEntry{
    abs_path:String
}
impl DirEntry{
    pub fn new(path:String)->DirEntry{
        DirEntry{
            abs_path:path
        }
    }
}
impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> (Vec<u8>, String) {
        let file_path = self.abs_path.as_str().to_owned()+MAIN_SEPARATOR.to_string().as_str()+class_name;
        return match fs::read(file_path) {
            Ok(data) => {
                (data, "".to_string())
            }
            Err(_) => {
                (vec![], "could not find class :".to_owned() + class_name)
            }
        };
    }

    fn string(&self) -> &str {
        self.abs_path.as_str()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn dir_entry_test(){
        let a = DirEntry::new("E:\\临时文件\\class".to_string());
        let (data,_) = a.read_class("a.txt");
        assert_eq!("DirEntry",String::from_utf8(data).unwrap());
    }
}