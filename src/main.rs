extern crate core;


use std::borrow::Borrow;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use crate::class_file::reader::Reader;
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
    let (mut data,e) = class_path.read_class("java/lang/String.class");
    println!("{},{},{}",data.get(0).unwrap(),data.get(1).unwrap(),data.get(2).unwrap());
    let mut reader = Reader{
        data,
        pointer: 0,
    };
    println!("{:?}", reader.read_bytes(3));
    println!("{:?}", reader.read_bytes(1));
}