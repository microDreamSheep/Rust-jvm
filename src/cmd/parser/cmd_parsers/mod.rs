use crate::cmd::Environment;
use crate::cmd::parser::Parser;

pub struct ClassPath{}
impl Parser for ClassPath{
    fn parse(&self,env: &mut Environment, value: String) {
        env.class_path_option = value;
    }
}

pub struct HelpFlag{}
impl Parser for HelpFlag{
    fn parse(&self,env: &mut Environment, _: String) {
        env.help_flag = true;
    }
}

pub struct VersionFlag{}
impl Parser for VersionFlag{
    fn parse(&self,env: &mut Environment, _: String) {
        env.version_flag = true;
    }
}

pub struct Class{}
impl Parser for Class{
    fn parse(&self,env: &mut Environment, value: String) {
        env.class = value;
    }
}

pub struct ArgsParser {}
impl Parser for ArgsParser {
    fn parse(&self, env: &mut Environment, value: String) {
        env.args.push(value);
    }
}

pub struct JreOption{}
impl Parser for JreOption {
    fn parse(&self, env: &mut Environment, value: String) {
        env.jre_option = value;
    }
}