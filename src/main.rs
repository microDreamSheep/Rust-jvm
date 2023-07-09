extern crate core;

use crate::class_file::ClassFile;
use std::env;
use std::io::Read;
use std::ops::Deref;
use std::time::Instant;
use crate::class_file::attribute::CodeAttribute;
use crate::class_file::constant_info::{ConstantClassInfo, ConstantInfo};
use crate::class_file::reader::ByteCodeReader;
use crate::class_path::parse;
use crate::cmd::Environment;
use crate::instructions::{byte_code_reader, get_instruction, Instruction};
use crate::run_time_data::{Frame, RThread};

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
    //io读取class文件的字节数据
    let mut file = std::fs::File::open("E://A//test.class").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let class_file = ClassFile::parse(data);
    let constants = class_file.constant_pool.borrow();
     for x in class_file.methods {
        if constants.get_utf8(&x.name_index).eq("main") {
            for x in x.attributes {
                let a =  x.as_any().downcast_ref::<CodeAttribute>().unwrap().code.clone();
                let mut frame = Frame::new(None, 50, 50, byte_code_reader::ByteCodeReader::new(a.clone()));
                loop {
                    let opcode = frame.get_reader().read_uint8();
                    let mut inst = get_instruction(opcode);
                    inst.fetch_operands(&mut frame.get_reader());
                    inst.execute(&mut frame);
                    if frame.get_reader().get_pointer() >= a.len() {
                        break;
                    }
                }

            }
        }
    }
}