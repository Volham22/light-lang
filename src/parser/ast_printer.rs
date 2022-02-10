use super::visitors::Visitor;

struct AstPrinter;

impl Visitor<()> for AstPrinter {
    fn visit_expression(&mut self, expr: super::visitors::Expression) -> () {
        todo!()
    }
}
