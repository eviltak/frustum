
use pos::*;

use syntax;
use syntax_pos::{self, Pos};

pub fn span_from_ast_span(span: &syntax_pos::Span, sess: &syntax::parse::ParseSess) -> Span {
    let code_map = sess.codemap();

    Span {
        start: Position::from_usize(code_map.bytepos_to_file_charpos(span.lo).to_usize()),
        end: Position::from_usize(code_map.bytepos_to_file_charpos(span.hi).to_usize()),
        file: code_map.span_to_filename(*span)
    }
}
