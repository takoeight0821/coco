mod core_ir;
mod lexer;
mod location;
mod name;
mod token;

use std::io;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

fn main() -> io::Result<()> {
    let source = r#"
        def mult(l; α) = invoke[multAux](l; α, α)
        def multAux(l; α, β) =
            l | match {
                Nil() -> 1 | β,
                Cons(x, xs) -> switch x {
                    0 -> 0 | α,
                    _ -> invoke[multAux](xs; α, then z prim[mul](x, z; β)),
                },
            }
        def hello(;α) = prim[print]("こんにちは"; α) 
    "#;

    let mut lexer = lexer::Lexer::new("source".to_string(), source);
    while let Some(token) = lexer.next_token() {
        let mut colors = ColorGenerator::new();
        let a = colors.next();
        let report = Report::build(ReportKind::Advice, token.location.clone())
            .with_message("found token")
            .with_label(
                Label::new(token.location.clone())
                    .with_message(format!("{:?}", token.kind))
                    .with_color(a),
            )
            .finish();
        report.print(("source".to_string(), Source::from(source)))?;
    }

    Ok(())
}
