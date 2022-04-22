use crate::parser::visitors::Statement;

use self::for_desugar::ForDesugar;

mod for_desugar;

fn desugar_for_loops(stmts: &mut Vec<Statement>) {
    let mut for_desugar = ForDesugar {};
    let mut desugared: Vec<(usize, Statement)> = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        if let Some(d) = for_desugar.visit_stmt(stmt) {
            desugared.push((i, d));
        }
    }

    for (i, stmt) in desugared {
        stmts[i] = stmt;
    }
}

pub fn desugar_ast(stmts: &mut Vec<Statement>) {
    desugar_for_loops(stmts);
}
