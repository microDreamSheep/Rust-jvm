extern crate core;

use crate::class_file::ClassFile;
use std::env;
use std::ops::Deref;
use std::time::Instant;
use crate::class_file::constant_info::{ConstantClassInfo, ConstantInfo};
use crate::class_path::parse;
use crate::cmd::Environment;

mod cmd;
mod class_path;
mod class_file;
mod instructions;
mod run_time_data;

fn main() {
    match Environment::parse_environment(env::args().collect()){
        None => {
            panic!("参数异常")
        }
        Some(environment) => {
            if environment.version_flag|| environment.help_flag {
                return;
            }
            //开始执行
            start(environment);
        }
    }
}
fn start(env:Environment){
    let class_path = parse(env);
    let (data,_) = class_path.read_class("java/lang/String.class");

    let start = Instant::now();
    let class_file = ClassFile::parse(data);

}