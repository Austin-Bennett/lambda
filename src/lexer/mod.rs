/*
How the lexer works:

[[ TOKENS ]]
token types:
EXPRESSIONS: operators like + - * / ** (power operator) (bitwise operators), boolean operators, as well as variables, numeric literals, string literals, etc
KEYWORDS: if, while, for, etc
FEATURES: {, }, [, ], (, ), etc

Some tokens are several types, like () and [] groups

*/
pub mod token;
pub mod literal;
pub mod token_parsers;
pub mod tokenizer;