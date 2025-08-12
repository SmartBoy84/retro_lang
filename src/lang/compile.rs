use crate::lang::{
    CompileError, Inst, OpCode, Program, ProgramError, UnresolvedJump, inst::VarMode,
};

impl<'a> Program<'a> {
    pub fn append_instructions(&mut self, instructions: &'a str) -> Result<(), CompileError> {
        for (n, l) in instructions.lines().map(|l| l.trim()).enumerate() {
            let mut line = l;

            if let Some((new_l, _)) = l.split_once("#") {
                // ez comment support
                line = new_l;
            }

            if let Some((new_l, label)) = line.split_once(';') {
                if label.len() == 0 {
                    return Err(CompileError(super::ProgramError::NoLabelLen, l, n));
                }

                line = new_l.trim();
                if line.len() == 0 {
                    return Err(CompileError(ProgramError::EmptyLineLabelled, l, n));
                }

                self.jumps
                    .insert(label.trim(), self.instructions.len() as u8);
            }

            let mut inst = line.splitn(2, ' ');
            let Some(action) = inst.next() else {
                continue;
            };

            if action.len() == 0 {
                continue;
            }

            if action == "noop" {
                self.push_inst(OpCode::ADC, 0);
                continue;
            }

            if let Some(args) = inst.next() {
                match action {
                    "var" => self.handle_var(args, VarMode::NewVar),
                    "goto" => self.add_goto(args, l),
                    _ => self.handle_var(line, VarMode::SetVar),
                }
            } else {
                self.handle_var(line, VarMode::SetVar)
            }
            .map_err(|e| CompileError(e, line, n))?;
        }

        self.resolve_gotos()?;

        Ok(())
    }

    fn resolve_gotos(&mut self) -> Result<(), CompileError> {
        for i in 0..self.unresolved_jumps.len() {
            let UnresolvedJump {
                from,
                to,
                __debug_line,
            } = self.unresolved_jumps[i];
            let Some(i) = self.jumps.get(to) else {
                return Err(CompileError(
                    ProgramError::LabelNotFound,
                    __debug_line,
                    from,
                ));
            };

            let Inst { op, .. } = self.instructions[from];
            self.set_inst(from, op, *i);
        }
        self.unresolved_jumps.clear();
        Ok(())
    }
}
