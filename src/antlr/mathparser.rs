// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\antlr_parser\math.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::mathlistener::*;
use super::mathvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const LOG:isize=4; 
		pub const ADD:isize=5; 
		pub const SUB:isize=6; 
		pub const MUL:isize=7; 
		pub const DIV:isize=8; 
		pub const POW:isize=9; 
		pub const ID:isize=10; 
		pub const NUMBER:isize=11; 
		pub const WS:isize=12;
	pub const RULE_root:usize = 0; 
	pub const RULE_expr:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"root", "expr"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;10] = [
		None, Some("'('"), Some("')'"), Some("','"), Some("'log_'"), Some("'+'"), 
		Some("'-'"), Some("'*'"), Some("'/'"), Some("'^'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
		None, None, None, None, Some("LOG"), Some("ADD"), Some("SUB"), Some("MUL"), 
		Some("DIV"), Some("POW"), Some("ID"), Some("NUMBER"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,mathParserExt<'input>, I, mathParserContextType , dyn mathListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type mathTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, mathParserContextType , dyn mathListener<'input> + 'a>;

/// Parser for math grammar
pub struct mathParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				mathParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> mathParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> mathParser<'input, I, DefaultErrorStrategy<'input,mathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for mathParser
pub trait mathParserContext<'input>:
	for<'x> Listenable<dyn mathListener<'input> + 'x > + 
	for<'x> Visitable<dyn mathVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=mathParserContextType>
{}

antlr_rust::coerce_from!{ 'input : mathParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn mathParserContext<'input> + 'input
where
    T: mathVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn mathVisitor<'input> + 'x))
    }
}

impl<'input> mathParserContext<'input> for TerminalNode<'input,mathParserContextType> {}
impl<'input> mathParserContext<'input> for ErrorNode<'input,mathParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn mathParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn mathListener<'input> + 'input }

pub struct mathParserContextType;
antlr_rust::tid!{mathParserContextType}

impl<'input> ParserNodeType<'input> for mathParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn mathParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct mathParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> mathParserExt<'input>{
}
antlr_rust::tid! { mathParserExt<'a> }

impl<'input> TokenAware<'input> for mathParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for mathParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for mathParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "math.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn mathParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => mathParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> mathParser<'input, I, DefaultErrorStrategy<'input,mathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 7)
				}
				1=>{
					recog.precpred(None, 6)
				}
				2=>{
					recog.precpred(None, 5)
				}
			_ => true
		}
	}
}
//------------------- root ----------------
pub type RootContextAll<'input> = RootContext<'input>;


pub type RootContext<'input> = BaseParserRuleContext<'input,RootContextExt<'input>>;

#[derive(Clone)]
pub struct RootContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> mathParserContext<'input> for RootContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for RootContext<'input>{
		fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_root(self);
		}
		fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
			listener.exit_root(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for RootContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_root(self);
	}
}

impl<'input> CustomRuleContext<'input> for RootContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_root }
	//fn type_rule_index() -> usize where Self: Sized { RULE_root }
}
antlr_rust::tid!{RootContextExt<'a>}

impl<'input> RootContextExt<'input>{
	fn new(parent: Option<Rc<dyn mathParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RootContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RootContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RootContextAttrs<'input>: mathParserContext<'input> + BorrowMut<RootContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> RootContextAttrs<'input> for RootContext<'input>{}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn root(&mut self,)
	-> Result<Rc<RootContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RootContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_root);
        let mut _localctx: Rc<RootContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(4);
			recog.expr_rec(0)?;

			recog.base.set_state(5);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	AddContext(AddContext<'input>),
	NumberContext(NumberContext<'input>),
	ParensContext(ParensContext<'input>),
	LogContext(LogContext<'input>),
	VarContext(VarContext<'input>),
	FunctionContext(FunctionContext<'input>),
	PowerContext(PowerContext<'input>),
	MultiplyContext(MultiplyContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> mathParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			AddContext(inner) => inner,
			NumberContext(inner) => inner,
			ParensContext(inner) => inner,
			LogContext(inner) => inner,
			VarContext(inner) => inner,
			FunctionContext(inner) => inner,
			PowerContext(inner) => inner,
			MultiplyContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn mathVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn mathListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn mathListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> mathParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn mathParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: mathParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type AddContext<'input> = BaseParserRuleContext<'input,AddContextExt<'input>>;

pub trait AddContextAttrs<'input>: mathParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ADD
	/// Returns `None` if there is no child corresponding to token ADD
	fn ADD(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(ADD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SUB
	/// Returns `None` if there is no child corresponding to token SUB
	fn SUB(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(SUB, 0)
	}
}

impl<'input> AddContextAttrs<'input> for AddContext<'input>{}

pub struct AddContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddContextExt<'a>}

impl<'input> mathParserContext<'input> for AddContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for AddContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_add(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_add(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for AddContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_add(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AddContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AddContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AddContext<'input> {}

impl<'input> AddContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AddContext(
				BaseParserRuleContext::copy_from(ctx,AddContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumberContext<'input> = BaseParserRuleContext<'input,NumberContextExt<'input>>;

pub trait NumberContextAttrs<'input>: mathParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUMBER
	/// Returns `None` if there is no child corresponding to token NUMBER
	fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, 0)
	}
}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

pub struct NumberContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NumberContextExt<'a>}

impl<'input> mathParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for NumberContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_number(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_number(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for NumberContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for NumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for NumberContext<'input> {}

impl<'input> NumberContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::NumberContext(
				BaseParserRuleContext::copy_from(ctx,NumberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParensContext<'input> = BaseParserRuleContext<'input,ParensContextExt<'input>>;

pub trait ParensContextAttrs<'input>: mathParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParensContextAttrs<'input> for ParensContext<'input>{}

pub struct ParensContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParensContextExt<'a>}

impl<'input> mathParserContext<'input> for ParensContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for ParensContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parens(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_parens(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for ParensContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_parens(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParensContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParensContext<'input> {}

impl<'input> ParensContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParensContext(
				BaseParserRuleContext::copy_from(ctx,ParensContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogContext<'input> = BaseParserRuleContext<'input,LogContextExt<'input>>;

pub trait LogContextAttrs<'input>: mathParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LOG
	/// Returns `None` if there is no child corresponding to token LOG
	fn LOG(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(LOG, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LogContextAttrs<'input> for LogContext<'input>{}

pub struct LogContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogContextExt<'a>}

impl<'input> mathParserContext<'input> for LogContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for LogContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_log(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_log(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for LogContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_log(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for LogContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for LogContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for LogContext<'input> {}

impl<'input> LogContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::LogContext(
				BaseParserRuleContext::copy_from(ctx,LogContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarContext<'input> = BaseParserRuleContext<'input,VarContextExt<'input>>;

pub trait VarContextAttrs<'input>: mathParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(ID, 0)
	}
}

impl<'input> VarContextAttrs<'input> for VarContext<'input>{}

pub struct VarContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarContextExt<'a>}

impl<'input> mathParserContext<'input> for VarContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for VarContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_var(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_var(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for VarContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_var(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for VarContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for VarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for VarContext<'input> {}

impl<'input> VarContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::VarContext(
				BaseParserRuleContext::copy_from(ctx,VarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FunctionContext<'input> = BaseParserRuleContext<'input,FunctionContextExt<'input>>;

pub trait FunctionContextAttrs<'input>: mathParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(ID, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input>{}

pub struct FunctionContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FunctionContextExt<'a>}

impl<'input> mathParserContext<'input> for FunctionContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for FunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_function(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_function(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for FunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for FunctionContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for FunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for FunctionContext<'input> {}

impl<'input> FunctionContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::FunctionContext(
				BaseParserRuleContext::copy_from(ctx,FunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PowerContext<'input> = BaseParserRuleContext<'input,PowerContextExt<'input>>;

pub trait PowerContextAttrs<'input>: mathParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token POW
	/// Returns `None` if there is no child corresponding to token POW
	fn POW(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(POW, 0)
	}
}

impl<'input> PowerContextAttrs<'input> for PowerContext<'input>{}

pub struct PowerContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PowerContextExt<'a>}

impl<'input> mathParserContext<'input> for PowerContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for PowerContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_power(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_power(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for PowerContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_power(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for PowerContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for PowerContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for PowerContext<'input> {}

impl<'input> PowerContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::PowerContext(
				BaseParserRuleContext::copy_from(ctx,PowerContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultiplyContext<'input> = BaseParserRuleContext<'input,MultiplyContextExt<'input>>;

pub trait MultiplyContextAttrs<'input>: mathParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token MUL
	/// Returns `None` if there is no child corresponding to token MUL
	fn MUL(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(MUL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token DIV
	/// Returns `None` if there is no child corresponding to token DIV
	fn DIV(&self) -> Option<Rc<TerminalNode<'input,mathParserContextType>>> where Self:Sized{
		self.get_token(DIV, 0)
	}
}

impl<'input> MultiplyContextAttrs<'input> for MultiplyContext<'input>{}

pub struct MultiplyContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MultiplyContextExt<'a>}

impl<'input> mathParserContext<'input> for MultiplyContext<'input>{}

impl<'input,'a> Listenable<dyn mathListener<'input> + 'a> for MultiplyContext<'input>{
	fn enter(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_multiply(self);
	}
	fn exit(&self,listener: &mut (dyn mathListener<'input> + 'a)) {
		listener.exit_multiply(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn mathVisitor<'input> + 'a> for MultiplyContext<'input>{
	fn accept(&self,visitor: &mut (dyn mathVisitor<'input> + 'a)) {
		visitor.visit_multiply(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = mathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MultiplyContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MultiplyContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MultiplyContext<'input> {}

impl<'input> MultiplyContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MultiplyContext(
				BaseParserRuleContext::copy_from(ctx,MultiplyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> mathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 2;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(35);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = NumberContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(8);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = LogContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(9);
					recog.base.match_token(LOG,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(10);
					recog.expr_rec(0)?;

					recog.base.set_state(11);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(12);
					recog.expr_rec(0)?;

					recog.base.set_state(13);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = FunctionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(15);
					recog.base.match_token(ID,&mut recog.err_handler)?;

					recog.base.set_state(16);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(17);
					recog.expr_rec(0)?;

					recog.base.set_state(22);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(18);
							recog.base.match_token(T__2,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(19);
							recog.expr_rec(0)?;

							}
							} 
						}
						recog.base.set_state(24);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
					}
					recog.base.set_state(26);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__2 {
						{
						recog.base.set_state(25);
						recog.base.match_token(T__2,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(28);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = VarContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(30);
					recog.base.match_token(ID,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = ParensContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(31);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(32);
					recog.expr_rec(0)?;

					recog.base.set_state(33);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(48);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(46);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = PowerContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(37);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(38);
							recog.base.match_token(POW,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(39);
							recog.expr_rec(8)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MultiplyContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(40);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(41);
							_la = recog.base.input.la(1);
							if { !(_la==MUL || _la==DIV) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(42);
							recog.expr_rec(7)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(43);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(44);
							_la = recog.base.input.la(1);
							if { !(_la==ADD || _la==SUB) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(45);
							recog.expr_rec(6)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(50);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x0e\x36\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x02\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x07\x03\x17\x0a\x03\x0c\x03\x0e\x03\x1a\x0b\x03\
	\x03\x03\x05\x03\x1d\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x05\x03\x26\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x31\x0a\x03\x0c\x03\x0e\x03\x34\
	\x0b\x03\x03\x03\x02\x03\x04\x04\x02\x04\x02\x04\x03\x02\x09\x0a\x03\x02\
	\x07\x08\x02\x3c\x02\x06\x03\x02\x02\x02\x04\x25\x03\x02\x02\x02\x06\x07\
	\x05\x04\x03\x02\x07\x08\x07\x02\x02\x03\x08\x03\x03\x02\x02\x02\x09\x0a\
	\x08\x03\x01\x02\x0a\x26\x07\x0d\x02\x02\x0b\x0c\x07\x06\x02\x02\x0c\x0d\
	\x05\x04\x03\x02\x0d\x0e\x07\x03\x02\x02\x0e\x0f\x05\x04\x03\x02\x0f\x10\
	\x07\x04\x02\x02\x10\x26\x03\x02\x02\x02\x11\x12\x07\x0c\x02\x02\x12\x13\
	\x07\x03\x02\x02\x13\x18\x05\x04\x03\x02\x14\x15\x07\x05\x02\x02\x15\x17\
	\x05\x04\x03\x02\x16\x14\x03\x02\x02\x02\x17\x1a\x03\x02\x02\x02\x18\x16\
	\x03\x02\x02\x02\x18\x19\x03\x02\x02\x02\x19\x1c\x03\x02\x02\x02\x1a\x18\
	\x03\x02\x02\x02\x1b\x1d\x07\x05\x02\x02\x1c\x1b\x03\x02\x02\x02\x1c\x1d\
	\x03\x02\x02\x02\x1d\x1e\x03\x02\x02\x02\x1e\x1f\x07\x04\x02\x02\x1f\x26\
	\x03\x02\x02\x02\x20\x26\x07\x0c\x02\x02\x21\x22\x07\x03\x02\x02\x22\x23\
	\x05\x04\x03\x02\x23\x24\x07\x04\x02\x02\x24\x26\x03\x02\x02\x02\x25\x09\
	\x03\x02\x02\x02\x25\x0b\x03\x02\x02\x02\x25\x11\x03\x02\x02\x02\x25\x20\
	\x03\x02\x02\x02\x25\x21\x03\x02\x02\x02\x26\x32\x03\x02\x02\x02\x27\x28\
	\x0c\x09\x02\x02\x28\x29\x07\x0b\x02\x02\x29\x31\x05\x04\x03\x0a\x2a\x2b\
	\x0c\x08\x02\x02\x2b\x2c\x09\x02\x02\x02\x2c\x31\x05\x04\x03\x09\x2d\x2e\
	\x0c\x07\x02\x02\x2e\x2f\x09\x03\x02\x02\x2f\x31\x05\x04\x03\x08\x30\x27\
	\x03\x02\x02\x02\x30\x2a\x03\x02\x02\x02\x30\x2d\x03\x02\x02\x02\x31\x34\
	\x03\x02\x02\x02\x32\x30\x03\x02\x02\x02\x32\x33\x03\x02\x02\x02\x33\x05\
	\x03\x02\x02\x02\x34\x32\x03\x02\x02\x02\x07\x18\x1c\x25\x30\x32";

