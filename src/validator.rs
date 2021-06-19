use crate::token::Token;

pub fn validate(first: &Token, second: &Token) -> bool {
    if matches!(second, Token::WhiteSpace {..}) {
        return true;
    }
    return match first {
        Token::WhiteSpace { .. } | Token::Open { .. } => match second {
            Token::Open { .. } | Token::Number { .. } => true,
            it if it.is_unary() || it.is_constant() => true,
            _ => false,
        },
        Token::Close { .. } | Token::Number { .. } => match second {
            Token::Close { .. } => true,
            it if it.is_binary() => true,
            _ => false,
        },
        it if it.is_binary() => match second {
            Token::Open { .. } | Token::Number { .. } => true,
            it if it.is_unary() || it.is_constant() => true,
            _ => false,
        },
        it if it.is_unary() && matches!(second, Token::Open {..}) => true,
        it if it.is_constant() => match second {
            Token::Close { .. } => true,
            it if it.is_binary() => true,
            _ => false,
        },
        _ => false
    };
}