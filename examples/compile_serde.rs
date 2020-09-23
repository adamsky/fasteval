use std::collections::BTreeMap; // use this trait so we can call compile().

// use serde_cbor;
use fasteval::Evaler; // use this trait so we can call eval().
use fasteval::{Compiler, Instruction};

fn main() -> Result<(), fasteval::Error> {
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let mut map = BTreeMap::new();

    // compile an expression to an `Instruction` struct
    let expr_str = "sin(deg/360 * 2*pi())";
    let compiled = parser
        .parse(expr_str, &mut slab.ps)?
        .from(&slab.ps)
        .compile(&slab.ps, &mut slab.cs);

    // serialize and deserialize compiled instruction
    let serialized = serde_cbor::ser::to_vec_packed(&compiled).unwrap();
    println!("{:?}", serialized);
    let instr: Instruction = serde_cbor::from_slice(&serialized).unwrap();

    // eval the instruction as usual
    for deg in 0..360 {
        map.insert("deg".to_string(), deg as f64);
        // When working with compiled constant expressions, you can use the
        // eval_compiled*!() macros to save a function call:
        let val = fasteval::eval_compiled!(instr, &slab, &mut map);
        eprintln!("sin({}°) = {}", deg, val);
    }

    Ok(())
}
