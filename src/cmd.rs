use std::string::ToString;
use crate::cmd::constant::{HELP_STRING, VERSION};
use crate::cmd::parser::ParserMap;
mod constant;
#[derive(Debug)]
pub struct Environment {
    pub help_flag:bool,
    pub version_flag:bool,
    pub class_path_option:String,
    pub class:String,
    pub jre_option:String,
    pub args:Vec<String>
}
impl Environment{
    pub fn parse_environment(args:Vec<String>)->Option<Environment>{
        let env = Environment::parse(Environment::new(),args);
        if env.help_flag {
            print!("{}",HELP_STRING);
        }
        if env.version_flag{
            print!("{}",VERSION);
        }
        return  Option::from(env);
    }
    fn new()->Environment{
        Environment{
            help_flag:false,
            version_flag:false,
            class_path_option: ".".to_string(),
            class:"".to_string(),
            args:Vec::new(),
            jre_option:"".to_string()
        }
    }
    fn parse(mut env:Environment, args:Vec<String>) ->Environment{
        let parsers = ParserMap::new();
        for (i,x) in args.iter().enumerate() {
            if i == 0 {continue}
            if !x.contains("=") {
                parsers.flag_parser(&mut env, x.to_string());
                continue;
            }
            let map:Vec<&str> = x.split("=").collect();
            let key = map.get(0).unwrap();
            let value = map.get(1).unwrap();
            parsers.kv_parser(&mut env, key.to_string(), value.to_string());
        }
        env
    }
}

mod parser;

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn parse_environment_test(){
        let null:String = "null".to_string();
        //类信息->class
        let class = Environment::parse_environment(vec![null.clone(), "class=class_test".to_string()]).unwrap();
        assert_eq!(class.class, "class_test", "class参数解析失败");
        //class path->cp
        let class_path_cp = Environment::parse_environment(vec![null.clone(), "cp=class_path_cp".to_string()]).unwrap();
        assert_eq!(class_path_cp.class_path_option, "class_path_cp", "class_path(cp形式)参数解析失败");
        //class path->class_path
        let class_path = Environment::parse_environment(vec![null.clone(), "class_path=class_path".to_string()]).unwrap();
        assert_eq!(class_path.class_path_option, "class_path", "class_path(class_path形式)参数解析失败");
        //version-> -v
        let version_v = Environment::parse_environment(vec![null.clone(), "-v".to_string()]).unwrap();
        assert_eq!(version_v.version_flag, true, "version(-v)参数解析失败");
        //version-> -version
        let version_version = Environment::parse_environment(vec![null.clone(), "-version".to_string()]).unwrap();
        assert_eq!(version_version.version_flag, true, "version(-version)参数解析失败");

    }
}