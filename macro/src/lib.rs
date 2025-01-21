extern crate cairo_lang_macro;
extern crate cairo_lang_parser;
extern crate cairo_lang_syntax;

use cairo_lang_macro::{attribute_macro, quote, ProcMacroResult, TokenStream};
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::with_db::SyntaxNodeWithDb;

#[attribute_macro]
pub fn some(_args: TokenStream, _token_stream: TokenStream) -> ProcMacroResult {
    let db_val = SimpleParserDatabase::default();
    let db = &db_val;
    let code = r#"
              #[derive(Drop)]
              struct Rectangle {
                  width: u64,
                  height: u64,
              }
              #[derive(Drop, PartialEq)]
              struct Square {
                  side_length: u64,
              }
              impl RectangleIntoSquare of TryInto<Rectangle, Square> {
                  fn try_into(self: Rectangle) -> Option<Square> {
                      if self.height == self.width {
                          Option::Some(Square { side_length: self.height })
                      } else {
                          Option::None
                      }
                  }
              }
              fn main() {
                let rectangle = Rectangle { width: 8, height: 8 };
                let result: Square = rectangle.try_into().unwrap();
                let expected = Square { side_length: 8 };
                assert!(
                    result == expected,
                    "Rectangle with equal width and height should be convertible to a square."
                );
                let rectangle = Rectangle { width: 5, height: 8 };
                let result: Option<Square> = rectangle.try_into();
                assert!(
                    result.is_none(),
                    "Rectangle with different width and height should not be convertible to a square."
                );
              }
          "#;
    let syntax_node = db.parse_virtual(code).unwrap();
    let syntax_node_with_db = SyntaxNodeWithDb::new(&syntax_node, db);
    let tokens = quote! {
      #syntax_node_with_db
      trait Circle {
        fn print() -> ();
      }
      impl CircleImpl of Circle {
        fn print() -> () {
          println!("This is a circle!");
        }
      }
    };
    ProcMacroResult::new(tokens)
}
