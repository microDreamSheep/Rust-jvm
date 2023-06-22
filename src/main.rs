extern crate core;

use crate::class_file::ClassFile;
use std::env;
use std::ops::Deref;
use crate::class_path::parse;
use crate::cmd::Environment;

mod cmd;
mod class_path;
mod class_file;

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
    let (mut data,_) = class_path.read_class("java/lang/String.class");
    let class_file = ClassFile::parse(data);
    println!("magic:{}",class_file.magic);
    println!("minor_version:{}",class_file.minor_version);
    println!("major_version:{}",class_file.major_version);
    //获取常量池
    let mut constant = class_file.constant_pool.deref().borrow();
    println!("constant_pool_count:{}",constant.constant_pool.len());
    for i in 0..constant.constant_pool.len()-1{
        println!("constant_pool[{}]:{:?}",i,constant.constant_pool[i].to_string());
    }
    println!("access_flags:{}",class_file.access_flags);
    println!("this_class:{}",class_file.this_class);
    println!("super_class:{}",class_file.super_class);
}