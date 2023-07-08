use crate::instructions::byte_code_reader::ByteCodeReader;
use crate::instructions::Instruction;
use crate::run_time_data::Frame;

struct DAdd;
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

struct FAdd;
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

struct IAdd;
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

struct LAdd;
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

struct DSub;
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

struct FSub;
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

struct ISub;
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

struct LSub;
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

struct DMul;
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

struct FMul;
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

struct IMul;
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

struct LMul;
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

struct DDiv;
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

struct FDiv;
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

struct IDiv;
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

struct LDiv;
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

struct DRem;
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

struct FRem;
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

struct IRem;
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

struct LRem;
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

struct INeg;
impl Instruction for INeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_int();
        frame.operator_stack.push_int(-val);
    }
}

struct LNeg;
impl Instruction for LNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_long();
        frame.operator_stack.push_long(-val);
    }
}

struct FNeg;
impl Instruction for FNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_float();
        frame.operator_stack.push_float(-val);
    }
}

struct DNeg;
impl Instruction for DNeg{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self, frame: &mut Frame){
        let val = frame.operator_stack.pop_double();
        frame.operator_stack.push_double(-val);
    }
}

struct IShl;
impl Instruction for IShl{
    fn fetch_operands(&mut self, _: &mut ByteCodeReader) {
        //do nothing
    }

    fn execute(&mut self,frame: &mut Frame) {
        let val1 = frame.operator_stack.pop_int();
        let val2 = frame.operator_stack.pop_int();
        fram.operator_stack.push_int(val2 << (val1 & 0x1f));
    }
}

struct LShl;
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

struct IShr;
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

struct LShr;
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

struct IUShr;
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

struct LUShr;
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

struct IAnd;
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

struct LAnd;
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

struct IOr;
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

struct LOr;
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

struct IXor;
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

struct LXor;
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

struct IInc{
    index: usize,
    const_val: i32,
}
impl Instruction for IInc{
    fn fetch_operands(&mut self, reader: &mut ByteCodeReader) {
        self.index = reader.read_u8() as usize;
        self.const_val = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame){
        let mut val = frame.local_vars.get_int(self.index as u32);
        val += self.const_val;
        frame.local_vars.set_int(self.index as u32, val);
    }
}