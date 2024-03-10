#![allow(nonstandard_style)]
// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\compiler\math.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::mathparser::*;

pub trait mathListener<'input> : ParseTreeListener<'input,mathParserContextType>{
/**
 * Enter a parse tree produced by {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link mathParser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link mathParser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link mathParser#factor}.
 * @param ctx the parse tree
 */
fn enter_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link mathParser#factor}.
 * @param ctx the parse tree
 */
fn exit_factor(&mut self, _ctx: &FactorContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : mathListener<'input> }


