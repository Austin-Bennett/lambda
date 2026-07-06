use crate::lexer::TokenParser;
use crate::lexer::parsers::KeywordParser;
use crate::lexer::Token;

#[test]
fn keyword_parser_matches_exact_keyword() {
    let (token, len) = KeywordParser.parse("let").unwrap();
    assert!(matches!(token, Token::Keyword(_)));
    assert_eq!(len, 3);
}

#[test]
fn keyword_parser_matches_keyword_before_boundary() {
    let (token, len) = KeywordParser.parse("let x = 3;").unwrap();
    assert!(matches!(token, Token::Keyword(_)));
    assert_eq!(len, 3);
}

#[test]
fn keyword_parser_rejects_identifier_with_keyword_prefix() {
    assert!(KeywordParser.parse("letter").is_none());
    assert!(KeywordParser.parse("returnValue").is_none());
}
