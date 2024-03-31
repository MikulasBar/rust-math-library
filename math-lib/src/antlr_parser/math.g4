grammar math;


// parser rules
prog: expr EOF;

expr
    : NUMBER											# number
    | PI												# pi
    | E													# e
    | expr POW expr										# power
    | expr (MUL | DIV) expr 							# multiply
    | expr (ADD | SUB) expr				    			# add
    | LOG expr '(' expr ')'								# log
    | ID '(' expr (',' expr)* ','? ')'					# function
    | ID												# var
    | '(' expr ')'										# parens
    ;

// lexer rules
LOG: 'log_';

PI: 'pi';
E: 'e';

ADD: '+';
SUB: '-';
MUL: '*';
DIV: '/';
POW: '^';

ID: ID_CHAR+;
NUMBER: DIGIT+ ('.' DIGIT+)?;
WS: [ \t\r\n]+ -> skip;

// fragments
fragment DIGIT: [0-9];
fragment ID_CHAR: [_a-zA-Z];