use std::fs::File;
use crate::class_path::Entry;
use std::io::prelude::*;

pub struct ZipEntry{
    abs_path:String
}
impl ZipEntry{
    pub fn new(path:String)->ZipEntry{
        ZipEntry{
            abs_path:path
        }
    }
}
impl Entry for ZipEntry {
    fn read_class(&self, class_name: &str) -> (Vec<u8>, String) {
        let zip_file = File::open(self.abs_path.as_str()).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if file.name().replace("\\","/")==class_name.replace("\\","/") {
                let mut buffer = Vec::new();
                // read the whole file
                let _ = file.read_to_end(&mut buffer);
                return (buffer,"".to_string());
            }
        };
        return  (vec![],"could not find class file".to_string());
    }

    fn string(&self) -> &str {
        self.abs_path.as_str()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn zip_entry_test(){
        let a = ZipEntry::new("E:\\Desktop.zip".to_string());
        a.read_class("a.txt");
        let (data,_) = a.read_class("Desktop\\a.txt");
        print!("{:?}",String::from_utf8(data));
    }
}