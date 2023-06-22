extern crate core;

use crate::class_file::ClassFile;
use std::borrow::Borrow;
use std::env;
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
    println!("constant_pool_count:{}",class_file.constant_pool.constant_pool.len());
    //获取常量池
    for i in 1..class_file.constant_pool.constant_pool.len(){
        println!("constant_pool[{}]:{:?}",i,class_file.constant_pool.constant_pool[i].to_string());
    }
    println!("access_flags:{}",class_file.access_flags);
    println!("this_class:{}",class_file.this_class);
    println!("super_class:{}",class_file.super_class);
}