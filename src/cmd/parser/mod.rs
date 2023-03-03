use std::collections::HashMap;
use crate::cmd::Environment;
use crate::cmd::parser::cmd_parsers::{ArgsParser, Class, ClassPath, HelpFlag, JreOption, VersionFlag};

mod cmd_parsers;
trait Parser {
    fn parse(&self,env:&mut Environment,value:String);
}

pub struct ParserMap {
    flag_parser_map:HashMap<String,Box<dyn Parser>>,
    kv_parser_map:HashMap<String,Box<dyn Parser>>,
    args_parser:ArgsParser
}
impl ParserMap {
    pub fn new() -> ParserMap {
        let mut parsers = ParserMap {
            flag_parser_map: HashMap::new(),
            kv_parser_map: HashMap::new(),
            args_parser: ArgsParser {},
        };
        parsers.kv_parser_map.insert("class".to_string(), Box::new(Class {}));
        parsers.kv_parser_map.insert("class_path".to_string(), Box::new(ClassPath {}));
        parsers.kv_parser_map.insert("cp".to_string(), Box::new(ClassPath {}));
        parsers.kv_parser_map.insert("Xjre".to_string(), Box::new(JreOption {}));
        parsers.flag_parser_map.insert("-help".to_string(), Box::new(HelpFlag {}));
        parsers.flag_parser_map.insert("-version".to_string(), Box::new(VersionFlag {}));
        parsers.flag_parser_map.insert("-v".to_string(), Box::new(VersionFlag {}));

        parsers
    }
    pub fn flag_parser(&self, env:&mut Environment, key:String) ->(){
        match self.flag_parser_map.get(key.as_str()) {
            None => {
                self.args_parser.parse(env,key);
            }
            Some(parser) => {
                parser.parse(env,"".to_string());
            }
        }
    }
    pub fn kv_parser(&self, env:&mut Environment, key:String,value:String) ->(){
        match self.kv_parser_map.get(key.as_str()) {
            None => {
                self.args_parser.parse(env,key+"="+ value.as_str());
            }
            Some(parser) => {
                parser.parse(env,value);
            }
        }
    }
}