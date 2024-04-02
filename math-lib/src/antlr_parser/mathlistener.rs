#![allow(nonstandard_style)]
// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\antlr_parser\math.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::mathparser::*;

pub trait mathListener<'input> : ParseTreeListener<'input,mathParserContextType>{
/**
 * Enter a parse tree produced by {@link mathParser#root}.
 * @param ctx the parse tree
 */
fn enter_root(&mut self, _ctx: &RootContext<'input>) { }
/**
 * Exit a parse tree produced by {@link mathParser#root}.
 * @param ctx the parse tree
 */
fn exit_root(&mut self, _ctx: &RootContext<'input>) { }
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
 * Enter a parse tree produced by the {@code var}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_var(&mut self, _ctx: &VarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code var}
 * labeled alternative in {@link mathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_var(&mut self, _ctx: &VarContext<'input>) { }
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


