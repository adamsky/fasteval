// usage:  cargo run --release --example compile

use fasteval::Compiler;
use fasteval::Evaler; // use this trait so we can call eval().
use std::collections::BTreeMap; // use this trait so we can call compile().
fn main() -> Result<(), fasteval::Error> {
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let mut map = BTreeMap::new();

    let expr_str = "sin(deg/360 * 2*pi())";
    // let expr_str = "2 + 200";
    let compiled = parser
        .parse(expr_str, &mut slab.ps)?
        .from(&slab.ps)
        .compile(&slab.ps, &mut slab.cs);
    for deg in 0..360 {
        // for deg in 0..1 {
        map.insert("deg".to_string(), deg as f64);
        // When working with compiled constant expressions, you can use the
        // eval_compiled*!() macros to save a function call:
        let val = fasteval::eval_compiled!(compiled, &slab, &mut map);
        eprintln!("sin({}°) = {}", deg, val);
    }

    Ok(())
}
