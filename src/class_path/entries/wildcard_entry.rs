use std::fs;
use crate::class_path::entries::new_entry;
use crate::class_path::Entry;

pub struct WildcardEntry{
    entries:Vec<Box<dyn Entry>>,
    string:String,
}
impl WildcardEntry{
    pub fn new(path:String)-> WildcardEntry {
        let mut ce = WildcardEntry{
            string:"".to_string(),
            entries: Vec::new()
        };
        let data = path.as_bytes();
        let data = &data[0..data.len()-1];
        let paths = fs::read_dir(String::from_utf8(data.to_owned()).unwrap()).unwrap();
        for path in paths {
            let is_dir = path.as_ref().unwrap().file_type().unwrap().is_dir();
            let path_str = path.as_ref().unwrap().path().as_os_str().to_str().unwrap().to_string();
            if !(path_str.ends_with(".jar")|| path_str.ends_with(".zip")|| is_dir) {
                continue;
            }
            let entry = new_entry(path_str);
            ce.string.push_str(entry.string());
            ce.string.push('\n');
            ce.entries.push(entry);
        }
        return ce;
    }
}

impl Entry for WildcardEntry {
    fn read_class(&self, class_name: &str) -> (Vec<u8>, String) {
        for x in &self.entries {
            let (data,_) = x.read_class(class_name);
            if data.len()!=0 {
                return (data,"".to_string());
            }
        }
        return (vec![],"could not find class :".to_owned() +class_name);
    }

    fn string(&self) -> &str {
        self.string.as_str()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn wildcard_entry_test(){
        let a = WildcardEntry::new("E:\\临时文件\\class\\*".to_string());
        let (data,_)  =a.read_class("Desktop\\a.txt");
        print!("{}",String::from_utf8(data).unwrap());
    }
}