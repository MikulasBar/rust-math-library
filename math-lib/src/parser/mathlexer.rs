// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\parser\math.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const T__2:isize=3; 
	pub const LOG:isize=4; 
	pub const PI:isize=5; 
	pub const E:isize=6; 
	pub const ADD:isize=7; 
	pub const SUB:isize=8; 
	pub const MUL:isize=9; 
	pub const DIV:isize=10; 
	pub const POW:isize=11; 
	pub const ID:isize=12; 
	pub const NUMBER:isize=13; 
	pub const WS:isize=14;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;16] = [
		"T__0", "T__1", "T__2", "LOG", "PI", "E", "ADD", "SUB", "MUL", "DIV", 
		"POW", "ID", "NUMBER", "WS", "DIGIT", "ID_CHAR"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;12] = [
		None, Some("'('"), Some("')'"), Some("','"), Some("'log_'"), Some("'pi'"), 
		Some("'e'"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'^'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
		None, None, None, None, Some("LOG"), Some("PI"), Some("E"), Some("ADD"), 
		Some("SUB"), Some("MUL"), Some("DIV"), Some("POW"), Some("ID"), Some("NUMBER"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct mathLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for mathLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for mathLexer<'input,Input>{
	type Target = BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for mathLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "mathLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				mathLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		mathLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct mathLexerActions {
}

impl mathLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>> for mathLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> mathLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,mathLexerActions,Input,LocalTokenFactory<'input>>> for mathLexerActions{
}
impl<'input> TokenAware<'input> for mathLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for mathLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x10\x5a\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\
		\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x03\x02\x03\x02\
		\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
		\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\
		\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x06\x0d\x3f\x0a\
		\x0d\x0d\x0d\x0e\x0d\x40\x03\x0e\x06\x0e\x44\x0a\x0e\x0d\x0e\x0e\x0e\x45\
		\x03\x0e\x03\x0e\x06\x0e\x4a\x0a\x0e\x0d\x0e\x0e\x0e\x4b\x05\x0e\x4e\x0a\
		\x0e\x03\x0f\x06\x0f\x51\x0a\x0f\x0d\x0f\x0e\x0f\x52\x03\x0f\x03\x0f\x03\
		\x10\x03\x10\x03\x11\x03\x11\x02\x02\x12\x03\x03\x05\x04\x07\x05\x09\x06\
		\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\
		\x1d\x10\x1f\x02\x21\x02\x03\x02\x05\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x03\
		\x02\x32\x3b\x05\x02\x43\x5c\x61\x61\x63\x7c\x02\x5c\x02\x03\x03\x02\x02\
		\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\
		\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\
		\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\
		\x02\x02\x17\x03\x02\x02\x02\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\x02\
		\x02\x02\x1d\x03\x02\x02\x02\x03\x23\x03\x02\x02\x02\x05\x25\x03\x02\x02\
		\x02\x07\x27\x03\x02\x02\x02\x09\x29\x03\x02\x02\x02\x0b\x2e\x03\x02\x02\
		\x02\x0d\x31\x03\x02\x02\x02\x0f\x33\x03\x02\x02\x02\x11\x35\x03\x02\x02\
		\x02\x13\x37\x03\x02\x02\x02\x15\x39\x03\x02\x02\x02\x17\x3b\x03\x02\x02\
		\x02\x19\x3e\x03\x02\x02\x02\x1b\x43\x03\x02\x02\x02\x1d\x50\x03\x02\x02\
		\x02\x1f\x56\x03\x02\x02\x02\x21\x58\x03\x02\x02\x02\x23\x24\x07\x2a\x02\
		\x02\x24\x04\x03\x02\x02\x02\x25\x26\x07\x2b\x02\x02\x26\x06\x03\x02\x02\
		\x02\x27\x28\x07\x2e\x02\x02\x28\x08\x03\x02\x02\x02\x29\x2a\x07\x6e\x02\
		\x02\x2a\x2b\x07\x71\x02\x02\x2b\x2c\x07\x69\x02\x02\x2c\x2d\x07\x61\x02\
		\x02\x2d\x0a\x03\x02\x02\x02\x2e\x2f\x07\x72\x02\x02\x2f\x30\x07\x6b\x02\
		\x02\x30\x0c\x03\x02\x02\x02\x31\x32\x07\x67\x02\x02\x32\x0e\x03\x02\x02\
		\x02\x33\x34\x07\x2d\x02\x02\x34\x10\x03\x02\x02\x02\x35\x36\x07\x2f\x02\
		\x02\x36\x12\x03\x02\x02\x02\x37\x38\x07\x2c\x02\x02\x38\x14\x03\x02\x02\
		\x02\x39\x3a\x07\x31\x02\x02\x3a\x16\x03\x02\x02\x02\x3b\x3c\x07\x60\x02\
		\x02\x3c\x18\x03\x02\x02\x02\x3d\x3f\x05\x21\x11\x02\x3e\x3d\x03\x02\x02\
		\x02\x3f\x40\x03\x02\x02\x02\x40\x3e\x03\x02\x02\x02\x40\x41\x03\x02\x02\
		\x02\x41\x1a\x03\x02\x02\x02\x42\x44\x05\x1f\x10\x02\x43\x42\x03\x02\x02\
		\x02\x44\x45\x03\x02\x02\x02\x45\x43\x03\x02\x02\x02\x45\x46\x03\x02\x02\
		\x02\x46\x4d\x03\x02\x02\x02\x47\x49\x07\x30\x02\x02\x48\x4a\x05\x1f\x10\
		\x02\x49\x48\x03\x02\x02\x02\x4a\x4b\x03\x02\x02\x02\x4b\x49\x03\x02\x02\
		\x02\x4b\x4c\x03\x02\x02\x02\x4c\x4e\x03\x02\x02\x02\x4d\x47\x03\x02\x02\
		\x02\x4d\x4e\x03\x02\x02\x02\x4e\x1c\x03\x02\x02\x02\x4f\x51\x09\x02\x02\
		\x02\x50\x4f\x03\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x50\x03\x02\x02\
		\x02\x52\x53\x03\x02\x02\x02\x53\x54\x03\x02\x02\x02\x54\x55\x08\x0f\x02\
		\x02\x55\x1e\x03\x02\x02\x02\x56\x57\x09\x03\x02\x02\x57\x20\x03\x02\x02\
		\x02\x58\x59\x09\x04\x02\x02\x59\x22\x03\x02\x02\x02\x08\x02\x40\x45\x4b\
		\x4d\x52\x03\x08\x02\x02";
