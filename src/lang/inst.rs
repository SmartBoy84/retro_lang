use crate::lang::{Inst, OpCode, Program, ProgramError, UnresolvedJump};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum VarMode {
    NewVar,
    SetVar,
}

const UNRESOLVED_ADDR: u8 = 0;

impl<'a> Program<'a> {
    pub fn push_inst(&mut self, op: OpCode, val: u8) {
        self.instructions.push(Inst { op, val });
    }

    pub fn set_inst(&mut self, i: usize, op: OpCode, val: u8) {
        self.instructions[i] = Inst { op, val }
    }

    fn get_var_loc(&self, name: &str) -> Result<&u8, ProgramError> {
        self.stack.get(name).ok_or(ProgramError::VarUndefined)
    }

    fn load(&mut self, name: &str) -> Result<(), ProgramError> {
        let name = name.trim();

        if let Ok(n) = name.parse::<i8>() {
            self.push_inst(OpCode::LDC, n as u8);
        } else if name == "reg" {
            /* noop for reg keyword - already loaded! */
        } else {
            let var = self.get_var_loc(name)?;
            self.push_inst(OpCode::LDM, *var);
        }
        Ok(())
    }

    fn handle_op(&mut self, op: char, operand: &'a str) -> Result<(), ProgramError> {
        let operand = operand.trim();

        match op {
            '+' | '-' => {
                if let Ok(n) = operand.parse::<i8>() {
                    self.push_inst(OpCode::ADC, if op == '+' { n } else { -n } as u8);
                } else {
                    if op == '+' {
                        let rhs_addr = self.get_var_loc(&operand)?;
                        self.push_inst(OpCode::ADM, *rhs_addr);
                    } else {
                        /*
                        Idea: need a subroutine
                        add 1 until hit 0 (overflow) -> this is 0xff - 1 => counter value when 0 is the negated result => VERY slow
                        Unfortunately, can only jump to constants - need to isnert new subroutine everytime
                        */
                        return Err(ProgramError::UnsupportedOperator);
                    }
                }
            }
            _ => return Err(ProgramError::UnsupportedOperator),
        };
        Ok(())
    }

    pub fn handle_expr(&mut self, expr: &'a str) -> Result<(), ProgramError> {
        // handle expression then push into accumulator

        if let Some(_) = expr.find('=') {
            return Err(ProgramError::BadSyntax);
        }

        let mut expr_pointer = expr.chars().enumerate();

        let mut old_i;
        let mut old_op;

        if let Some((i, op)) = expr_pointer.find(|(_, c)| matches!(c, '+' | '-')) {
            if i == 0 {
                // at start - find end
                match expr_pointer.find(|(_, c)| matches!(c, '+' | '-')) {
                    Some((i2, op)) if i2 - i > 1 => {
                        self.load(&expr[..i2])?;
                        old_op = op;
                        old_i = i2 + 1;
                    }
                    None => {
                        self.load(expr)?;
                        return Ok(());
                    }
                    _ => return Err(ProgramError::BadSyntax),
                }
            } else {
                self.load(&expr[..i])?;
                old_i = i + 1;
                old_op = op;
            }
        } else {
            self.load(expr)?;
            return Ok(());
        }

        loop {
            let Some((i, op)) = expr_pointer
                .by_ref()
                .skip_while(|(_, c)| !matches!(c, '+' | '-'))
                .next()
            else {
                self.handle_op(old_op, &expr[old_i..])?;
                break;
            };

            if i - old_i == 0 {
                // can't chain operators
                return Err(ProgramError::BadSyntax);
            }

            self.handle_op(old_op, &expr[old_i..i])?;

            old_op = op;
            old_i = i + 1; // +1 to skip over operator
        }
        Ok(())
        // at this point, expression result is in accumulator
    }

    pub fn handle_var(&mut self, arg_str: &'a str, var_mode: VarMode) -> Result<(), ProgramError> {
        let mut inst = arg_str.split('=').map(|s| s.trim());
        let Some(name) = inst.next() else {
            return Err(ProgramError::BadSyntax);
        };

        let mut char_found = false;
        for b in name.chars() {
            if b == ' ' {
                return Err(ProgramError::BadVarName);
            }
            if !b.is_numeric() && !b.is_ascii_punctuation() {
                char_found = true;
            }
        }
        if !char_found {
            return Err(ProgramError::BadVarName);
        }

        // special case - accumulator register
        if name == "reg" {
            if var_mode == VarMode::NewVar {
                return Err(ProgramError::ReservedName);
            }
            let Some(expr) = inst.next() else {
                return Err(ProgramError::BadSyntax);
            };
            return self.handle_expr(expr);
        }

        let id = match (self.stack.get(name), &var_mode) {
            (Some(_), VarMode::NewVar) => return Err(ProgramError::VarAlreadyDefined),

            (None, VarMode::SetVar) => return Err(ProgramError::VarUndefined),

            (Some(i), VarMode::SetVar) => *i,

            (None, VarMode::NewVar) => {
                self.stack.insert(name, self.stack_i);
                self.stack_i += 1;
                self.stack_i - 1
            }
        };
        match inst.next() {
            Some(expr) => self.handle_expr(expr)?,
            None if var_mode == VarMode::NewVar => self.push_inst(OpCode::CLR, 0),
            _ => (), // for just var name put register result into it
        }
        self.push_inst(OpCode::STR, id);

        Ok(())
    }

    pub fn add_goto(&mut self, label: &'a str, __debug_line: &'a str) -> Result<(), ProgramError> {
        let mut l = label.trim().split_ascii_whitespace();
        let Some(name) = l.next() else {
            return Err(ProgramError::LabelNotSpecified);
        };

        let inst = match l.next() {
            Some(cond_expr) => {
                self.handle_expr(cond_expr)?;
                OpCode::BRQ
            }
            None => OpCode::BRA,
        };

        let i = match self.jumps.get(name) {
            Some(&i) => i,
            None => {
                self.unresolved_jumps.push(UnresolvedJump {
                    from: self.instructions.len() as usize,
                    to: name,
                    __debug_line,
                });
                UNRESOLVED_ADDR
            }
        };

        self.push_inst(inst, i);

        Ok(())
    }
}
