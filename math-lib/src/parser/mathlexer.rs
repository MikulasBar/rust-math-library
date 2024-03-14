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
	pub const FN_NAME:isize=3; 
	pub const LOG:isize=4; 
	pub const LOGBASE:isize=5; 
	pub const LOG10:isize=6; 
	pub const LN:isize=7; 
	pub const SIN:isize=8; 
	pub const COS:isize=9; 
	pub const TAN:isize=10; 
	pub const ADD:isize=11; 
	pub const SUB:isize=12; 
	pub const MUL:isize=13; 
	pub const DIV:isize=14; 
	pub const POW:isize=15; 
	pub const NUMBER:isize=16; 
	pub const WS:isize=17;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;19] = [
		"T__0", "T__1", "FN_NAME", "LOG", "LOGBASE", "LOG10", "LN", "SIN", "COS", 
		"TAN", "ADD", "SUB", "MUL", "DIV", "POW", "NUMBER", "WS", "DIGIT", "SIGN"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;16] = [
		None, Some("'('"), Some("')'"), None, None, None, Some("'log'"), Some("'ln'"), 
		Some("'sin'"), Some("'cos'"), Some("'tan'"), Some("'+'"), Some("'-'"), 
		Some("'*'"), Some("'/'"), Some("'^'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;18]  = [
		None, None, None, Some("FN_NAME"), Some("LOG"), Some("LOGBASE"), Some("LOG10"), 
		Some("LN"), Some("SIN"), Some("COS"), Some("TAN"), Some("ADD"), Some("SUB"), 
		Some("MUL"), Some("DIV"), Some("POW"), Some("NUMBER"), Some("WS")
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
		\x13\x76\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\
		\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\
		\x04\x13\x09\x13\x04\x14\x09\x14\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\
		\x03\x04\x03\x04\x05\x04\x31\x0a\x04\x03\x05\x03\x05\x03\x05\x05\x05\x36\
		\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\
		\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\
		\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
		\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\
		\x03\x10\x03\x11\x05\x11\x5d\x0a\x11\x03\x11\x06\x11\x60\x0a\x11\x0d\x11\
		\x0e\x11\x61\x03\x11\x03\x11\x06\x11\x66\x0a\x11\x0d\x11\x0e\x11\x67\x05\
		\x11\x6a\x0a\x11\x03\x12\x06\x12\x6d\x0a\x12\x0d\x12\x0e\x12\x6e\x03\x12\
		\x03\x12\x03\x13\x03\x13\x03\x14\x03\x14\x02\x02\x15\x03\x03\x05\x04\x07\
		\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\
		\x0e\x1b\x0f\x1d\x10\x1f\x11\x21\x12\x23\x13\x25\x02\x27\x02\x03\x02\x05\
		\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x03\x02\x32\x3b\x04\x02\x2d\x2d\x2f\x2f\
		\x02\x7c\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\
		\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\
		\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\
		\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\x02\x02\x19\x03\x02\
		\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\x03\x02\x02\x02\x02\x1f\x03\x02\
		\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\x03\x02\x02\x02\x03\x29\x03\x02\
		\x02\x02\x05\x2b\x03\x02\x02\x02\x07\x30\x03\x02\x02\x02\x09\x35\x03\x02\
		\x02\x02\x0b\x37\x03\x02\x02\x02\x0d\x3e\x03\x02\x02\x02\x0f\x42\x03\x02\
		\x02\x02\x11\x45\x03\x02\x02\x02\x13\x49\x03\x02\x02\x02\x15\x4d\x03\x02\
		\x02\x02\x17\x51\x03\x02\x02\x02\x19\x53\x03\x02\x02\x02\x1b\x55\x03\x02\
		\x02\x02\x1d\x57\x03\x02\x02\x02\x1f\x59\x03\x02\x02\x02\x21\x5c\x03\x02\
		\x02\x02\x23\x6c\x03\x02\x02\x02\x25\x72\x03\x02\x02\x02\x27\x74\x03\x02\
		\x02\x02\x29\x2a\x07\x2a\x02\x02\x2a\x04\x03\x02\x02\x02\x2b\x2c\x07\x2b\
		\x02\x02\x2c\x06\x03\x02\x02\x02\x2d\x31\x05\x11\x09\x02\x2e\x31\x05\x13\
		\x0a\x02\x2f\x31\x05\x15\x0b\x02\x30\x2d\x03\x02\x02\x02\x30\x2e\x03\x02\
		\x02\x02\x30\x2f\x03\x02\x02\x02\x31\x08\x03\x02\x02\x02\x32\x36\x05\x0b\
		\x06\x02\x33\x36\x05\x0d\x07\x02\x34\x36\x05\x0f\x08\x02\x35\x32\x03\x02\
		\x02\x02\x35\x33\x03\x02\x02\x02\x35\x34\x03\x02\x02\x02\x36\x0a\x03\x02\
		\x02\x02\x37\x38\x07\x6e\x02\x02\x38\x39\x07\x71\x02\x02\x39\x3a\x07\x69\
		\x02\x02\x3a\x3b\x07\x61\x02\x02\x3b\x3c\x03\x02\x02\x02\x3c\x3d\x05\x21\
		\x11\x02\x3d\x0c\x03\x02\x02\x02\x3e\x3f\x07\x6e\x02\x02\x3f\x40\x07\x71\
		\x02\x02\x40\x41\x07\x69\x02\x02\x41\x0e\x03\x02\x02\x02\x42\x43\x07\x6e\
		\x02\x02\x43\x44\x07\x70\x02\x02\x44\x10\x03\x02\x02\x02\x45\x46\x07\x75\
		\x02\x02\x46\x47\x07\x6b\x02\x02\x47\x48\x07\x70\x02\x02\x48\x12\x03\x02\
		\x02\x02\x49\x4a\x07\x65\x02\x02\x4a\x4b\x07\x71\x02\x02\x4b\x4c\x07\x75\
		\x02\x02\x4c\x14\x03\x02\x02\x02\x4d\x4e\x07\x76\x02\x02\x4e\x4f\x07\x63\
		\x02\x02\x4f\x50\x07\x70\x02\x02\x50\x16\x03\x02\x02\x02\x51\x52\x07\x2d\
		\x02\x02\x52\x18\x03\x02\x02\x02\x53\x54\x07\x2f\x02\x02\x54\x1a\x03\x02\
		\x02\x02\x55\x56\x07\x2c\x02\x02\x56\x1c\x03\x02\x02\x02\x57\x58\x07\x31\
		\x02\x02\x58\x1e\x03\x02\x02\x02\x59\x5a\x07\x60\x02\x02\x5a\x20\x03\x02\
		\x02\x02\x5b\x5d\x05\x27\x14\x02\x5c\x5b\x03\x02\x02\x02\x5c\x5d\x03\x02\
		\x02\x02\x5d\x5f\x03\x02\x02\x02\x5e\x60\x05\x25\x13\x02\x5f\x5e\x03\x02\
		\x02\x02\x60\x61\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x61\x62\x03\x02\
		\x02\x02\x62\x69\x03\x02\x02\x02\x63\x65\x07\x30\x02\x02\x64\x66\x05\x25\
		\x13\x02\x65\x64\x03\x02\x02\x02\x66\x67\x03\x02\x02\x02\x67\x65\x03\x02\
		\x02\x02\x67\x68\x03\x02\x02\x02\x68\x6a\x03\x02\x02\x02\x69\x63\x03\x02\
		\x02\x02\x69\x6a\x03\x02\x02\x02\x6a\x22\x03\x02\x02\x02\x6b\x6d\x09\x02\
		\x02\x02\x6c\x6b\x03\x02\x02\x02\x6d\x6e\x03\x02\x02\x02\x6e\x6c\x03\x02\
		\x02\x02\x6e\x6f\x03\x02\x02\x02\x6f\x70\x03\x02\x02\x02\x70\x71\x08\x12\
		\x02\x02\x71\x24\x03\x02\x02\x02\x72\x73\x09\x03\x02\x02\x73\x26\x03\x02\
		\x02\x02\x74\x75\x09\x04\x02\x02\x75\x28\x03\x02\x02\x02\x0a\x02\x30\x35\
		\x5c\x61\x67\x69\x6e\x03\x08\x02\x02";
