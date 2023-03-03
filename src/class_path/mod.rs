use crate::cmd::Environment;
use crate::class_path::entries::new_entry;

pub trait Entry{
    fn read_class(&self,class_name:&str)->(Vec<u8>, String);
    fn string(&self)->&str;
}
mod entries;
pub struct ClassPath{
    boot_classpath:Box<dyn Entry>,
    ext_classpath:Box<dyn Entry>,
    user_classpath:Box<dyn Entry>
}
impl ClassPath{
    fn parse_boot_and_ext_classpath(jre_option:String)->(Box<dyn Entry>, Box<dyn Entry>){
        (new_entry(jre_option.to_owned()+"\\lib\\*"),new_entry(jre_option.to_owned()+"\\lib\\ext\\*"))
    }
    fn parse_user_classpath(mut user_classpath:String)->Box<dyn Entry>{
        if user_classpath == ""{
            user_classpath = ".".to_string();
        }
        return new_entry(user_classpath.to_owned());
    }
    pub fn read_class(&self,class_name:&str)->(Vec<u8>,String){
        let (data,_) = self.boot_classpath.read_class(class_name);
        if data.len()!=0 {
            return (data,"".to_string());
        }
        let (data,_) = self.ext_classpath.read_class(class_name);
        if data.len()!=0 {
            return (data,"".to_string());
        }
        let (data,_) = self.user_classpath.read_class(class_name);
        if data.len()!=0 {
            return (data,"".to_string());
        }
        return (vec![],"could not found class :".to_owned()+class_name);
    }
}
pub fn parse(env:Environment) ->ClassPath{
    let (boot,ext) = ClassPath::parse_boot_and_ext_classpath(env.jre_option);
    let user = ClassPath::parse_user_classpath(env.class_path_option);
    ClassPath{
        boot_classpath: boot,
        ext_classpath: ext,
        user_classpath: user,
    }
}
