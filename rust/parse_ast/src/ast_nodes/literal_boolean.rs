use swc_ecma_ast::Bool;

use crate::{store_literal_boolean, store_literal_boolean_flags};
use crate::convert_ast::converter::AstConverter;

impl<'a> AstConverter<'a> {
  pub(crate) fn store_literal_boolean(&mut self, literal: &Bool) {
    store_literal_boolean!(
      self,
      span => &literal.span,
      value => literal.value
    );
  }
}
