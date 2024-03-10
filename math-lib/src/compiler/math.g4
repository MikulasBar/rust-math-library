grammar math;


// parser rules
expr: term ((PLUS | MINUS) term)*;
term: factor ((MULTIPLY | DIVIDE) factor)*;
factor: '(' expr ')' | NUMBER;


// lexer rules
PLUS: '+';
MINUS: '-';
MULTIPLY: '*';
DIVIDE: '/';
POWER: '^';

NUMBER: DIGIT+ ('.' DIGIT+)?;
WS: [ \t\r\n]+ -> skip;


// fragments
fragment DIGIT: [0-9];