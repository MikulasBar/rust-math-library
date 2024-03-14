grammar math;


// parser rules
prog: expr EOF;
expr	
		: NUMBER						# number
		| expr POW expr					# power
		| expr (MUL | DIV) expr 		# multiply
		| expr (ADD | SUB) expr			# add
		| (FN_NAME | LOG) '(' expr ')'	# function
		| '(' expr ')'					# parens		
		;



// lexer rules
FN_NAME: SIN | COS | TAN;
LOG: LOGBASE | LOG10 | LN;

LOGBASE: 'log_' NUMBER;
LOG10: 'log';
LN: 'ln';

SIN: 'sin';
COS: 'cos';
TAN: 'tan';

ADD: '+';
SUB: '-';
MUL: '*';
DIV: '/';
POW: '^';

NUMBER: SIGN? DIGIT+ ('.' DIGIT+)?;
WS: [ \t\r\n]+ -> skip;

// fragments
fragment DIGIT: [0-9];
fragment SIGN: [+-];