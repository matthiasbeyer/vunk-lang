// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::decl::Decl;
use crate::def::Def;
use crate::ifelse::IfElse;
use crate::letin::LetIns;
use crate::literal::Literal;
use crate::name::VariableName;
use crate::op::BinaryOp;
use crate::op::UnaryOp;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expr {
    Variable(VariableName),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Literal(Literal),
    LetIn(LetIns),
    IfElse(IfElse),
    Decl(Decl),
    Def(Def),
}
