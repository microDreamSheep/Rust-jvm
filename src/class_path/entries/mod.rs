use std::path::MAIN_SEPARATOR;
use crate::class_path::entries::composite_entry::CompositeEntry;
use crate::class_path::entries::dir_entry::DirEntry;
use crate::class_path::entries::wildcard_entry::WildcardEntry;
use crate::class_path::entries::zip_entry::ZipEntry;
use crate::class_path::Entry;

mod zip_entry;
mod dir_entry;
mod composite_entry;
mod wildcard_entry;
pub fn new_entry(path:String)->Box<dyn Entry>{
    if path.contains(";") {
        return Box::new(CompositeEntry::new(path));
    }
    if path.ends_with("*"){
        return Box::new(WildcardEntry::new(path));
    }
    if path.ends_with(".jar")||path.ends_with(".zip") {
        return Box::new(ZipEntry::new(path));
    }
    return Box::new(DirEntry::new(path));
}