use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

pub struct DAdd;
impl Instruction for DAdd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_double();
        let val2 = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(val1 + val2);
    }
}

pub struct FAdd;
impl Instruction for FAdd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_float();
        let val2 = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(val1 + val2);
    }
}

pub struct IAdd;
impl Instruction for IAdd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val1 + val2);
    }
}

pub struct LAdd;
impl Instruction for LAdd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val1 + val2);
    }
}

pub struct DSub;
impl Instruction for DSub{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_double();
        let val2 = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(val2 - val1);
    }
}

pub struct FSub;
impl Instruction for FSub{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_float();
        let val2 = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(val2 - val1);
    }
}

pub struct ISub;
impl Instruction for ISub{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 - val1);
    }
}

pub struct LSub;
impl Instruction for LSub{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 - val1);
    }
}

pub struct DMul;
impl Instruction for DMul{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame) {

        let val1 = frame.operator_stack.pop_double();
        let val2 = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(val1 * val2);
    }
}

pub struct FMul;
impl Instruction for FMul{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_float();
        let val2 = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(val1 * val2);
    }
}

pub struct IMul;
impl Instruction for IMul{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val1 * val2);
    }
}

pub struct LMul;
impl Instruction for LMul{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val1 * val2);
    }
}

pub struct DDiv;
impl Instruction for DDiv{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_double();
        let val2 = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(val2 / val1);
    }
}

pub struct FDiv;
impl Instruction for FDiv{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_float();
        let val2 = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(val2 / val1);
    }
}

pub struct IDiv;
impl Instruction for IDiv{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 / val1);
    }
}

pub struct LDiv;
impl Instruction for LDiv{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 / val1);
    }
}

pub struct DRem;
impl Instruction for DRem{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_double();
        let val2 = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(val2 % val1);
    }
}

pub struct FRem;
impl Instruction for FRem{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_float();
        let val2 = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(val2 % val1);
    }
}

pub struct IRem;
impl Instruction for IRem{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 % val1);
    }
}

pub struct LRem;
impl Instruction for LRem{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 % val1);
    }
}

pub struct INeg;
impl Instruction for INeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(-val);
    }
}

pub struct LNeg;
impl Instruction for LNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(-val);
    }
}

pub struct FNeg;
impl Instruction for FNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(-val);
    }
}

pub struct DNeg;
impl Instruction for DNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(-val);
    }
}

pub struct IShl;
impl Instruction for IShl{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 << (val1 & 0x1f));
    }
}

pub struct LShl;
impl Instruction for LShl{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 << (val1 & 0x3f));
    }
}

pub struct IShr;
impl Instruction for IShr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 >> (val1 & 0x1f));
    }
}

pub struct LShr;
impl Instruction for LShr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 >> (val1 & 0x3f));
    }
}

pub struct IUShr;
impl Instruction for IUShr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 >> (val1 & 0x1f));
    }
}

pub struct LUShr;
impl Instruction for LUShr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 >> (val1 & 0x3f));
    }
}

pub struct IAnd;
impl Instruction for IAnd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 & val1);
    }
}

pub struct LAnd;
impl Instruction for LAnd{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 & val1);
    }
}

pub struct IOr;
impl Instruction for IOr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 | val1);
    }
}

pub struct LOr;
impl Instruction for LOr{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 | val1);
    }
}

pub struct IXor;
impl Instruction for IXor{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(val2 ^ val1);
    }
}

pub struct LXor;
impl Instruction for LXor{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_long();
        let val2 = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(val2 ^ val1);
    }
}

pub struct IInc{
    index: usize,
    const_val: i32,
}
impl Instruction for IInc{
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_uint8() as usize;
        self.const_val = reader.read_int8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame){
        let mut val = frame.local_vars.get_int(self.index as u32);
        val += self.const_val;
        frame.local_vars.set_int(self.index as u32, val);
    }
}