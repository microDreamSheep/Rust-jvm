use crate::class_path::entries::new_entry;
use crate::class_path::Entry;

pub struct CompositeEntry{
    entries:Vec<Box<dyn Entry>>,
    string:String
}
impl CompositeEntry{
    pub fn new(path:String)->CompositeEntry {
        let mut ce = CompositeEntry{
            entries: Vec::new(),
            string:"".to_string()
        };
        let paths:Vec<&str> = path.split(";").collect();
        for x in paths {
            let entry = new_entry(x.to_string());
            ce.string.push_str(entry.string());
            ce.string.push('\n');
            ce.entries.push(entry);
        }
        return ce;
    }
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> (Vec<u8>, String) {
        for x in &self.entries {
            let (data,_) = x.read_class(class_name);
            if data.len()!=0 {
                return (data,"".to_string());
            }
        }
        return (vec![],"could not find class :".to_owned()+class_name);
    }

    fn string(&self) -> &str {
        self.string.as_str()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn composite_entry_test(){
        let a = CompositeEntry::new("E:\\临时文件\\class".to_string());
        print!("{}",a.string())
    }
}
