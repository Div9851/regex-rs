use super::Instruction;
use crate::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverflow,
    SPOverflow,
    InvalidPC,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

/// 深さ優先探索で再帰敵にマッチングを行う関数。
fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverflow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverflow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, *addr1, sp)? || eval_depth(inst, line, *addr2, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

/// 命令列の評価を行う関数。
///
/// inst が命令列となり、その命令列を用いて入力文字列 line にマッチさせる。
///
/// 実行時にエラーが起きた場合は Err を返す。
/// マッチ成功時は Ok(true) を、失敗時は Ok(false) を返す。
pub fn eval(inst: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    eval_depth(inst, line, 0, 0)
}
