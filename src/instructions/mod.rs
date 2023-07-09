use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::comparisons::if_cond::IFACmpEq;
use crate::instructions::constants::*;
use crate::instructions::loads::*;
use crate::run_time_data::Frame;
use crate::instructions::stores::*;
use crate::instructions::stack::*;
use crate::instructions::math::*;
use crate::instructions::conversions::*;
use crate::instructions::comparisons::*;
use crate::instructions::control::*;


pub(crate) mod byte_code_reader;
mod constants;
mod loads;
mod stores;
mod stack;
mod math;
mod conversions;
mod comparisons;
mod control;
mod bytecode_constants;


pub trait Instruction{
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader);
    fn execute(&mut self, frame: &mut Frame);
}
pub fn get_instruction(key:u8) ->Box<dyn Instruction>{
    Box::new(Goto{ offset: 0 })
}
