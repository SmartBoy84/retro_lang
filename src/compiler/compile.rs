use crate::compiler::{Compiler, CompilerError, asm::AsmOutput};

impl<'a> Compiler<'a> {
    fn map_op<'b>(&self, op: &'b str) -> Result<&'a str, CompilerError<'b>> {
        let x = self
            .inst_set
            .iter()
            .enumerate()
            .find_map(|(i, l)| (*l == op).then_some(i))
            .ok_or(CompilerError::UnknownInst(op))?;

        Ok(self.op_codes[x])
    }

    pub fn compile<'b>(
        &'a self,
        instructions: &'b str,
    ) -> Result<AsmOutput<'a>, CompilerError<'b>>
    where
        'b: 'a, // no biggie, keep instructions around for as long as compiler
    {
        let mut program = Vec::new();

        for l in instructions.lines() {
            let mut s = l.trim().split_ascii_whitespace();
            let o = self.map_op(s.next().ok_or(CompilerError::NewLine)?)?;

            let a = s.next().unwrap_or("00"); // "00" is embedded in the program -> 'static > 'a
            if a.len() != 2 {
                return Err(CompilerError::ShortHex(l));
            }
            program.push(a);
            program.push(o);
        }

        Ok(program.into())
    }
}
