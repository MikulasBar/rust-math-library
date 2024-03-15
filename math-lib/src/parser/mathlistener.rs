#![allow(nonstandard_style)]
// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\parser\math.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::mathparser::*;

pub trait mathListener<'input> : ParseTreeListener<'input,mathParserContextType>{
/**
 * Enter a parse tree produced by {@link mathParser#prog}.
 * @param ctx the parse tree
 */
fn enter_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link mathParser#prog}.
 * @param ctx the parse tree
 */
fn exit_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code add}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code add}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code number}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code number}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parens}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_parens(&mut self, _ctx: &ParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parens}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_parens(&mut self, _ctx: &ParensContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code e}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_e(&mut self, _ctx: &EContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code e}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_e(&mut self, _ctx: &EContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code log}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_log(&mut self, _ctx: &LogContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code log}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_log(&mut self, _ctx: &LogContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code function}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code function}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code pi}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_pi(&mut self, _ctx: &PiContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code pi}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_pi(&mut self, _ctx: &PiContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code power}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code power}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiply}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_multiply(&mut self, _ctx: &MultiplyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiply}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_multiply(&mut self, _ctx: &MultiplyContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : mathListener<'input> }


