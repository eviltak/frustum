
use pos::*;

use syntax;
use syntax_pos;

pub fn span_from_ast_span(span: &syntax_pos::Span, sess: &syntax::parse::ParseSess) -> Span {
    let code_map = sess.codemap();

    Span {
        start: Position::from_u32(span.lo.0),
        end: Position::from_u32(span.hi.0),
    }
}
