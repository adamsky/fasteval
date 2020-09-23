use std::collections::BTreeMap;

use fasteval::Evaler;
use fasteval::{Compiler, Instruction, Slab};

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

    println!("slab: {:?}", slab);
    println!("instruction: {:?}", compiled);

    // serialize both the compiled instruction and the slab
    let slab_ser = serde_cbor::ser::to_vec_packed(&slab).unwrap();
    let instr_ser = serde_cbor::ser::to_vec_packed(&compiled).unwrap();
    println!("compiled slab: {:?}", slab_ser);
    println!("compiled instruction: {:?}", instr_ser);

    // deserialize
    let slab: Slab = serde_cbor::from_slice(&slab_ser).unwrap();
    let compiled: Instruction = serde_cbor::from_slice(&instr_ser).unwrap();

    // eval the instruction as usual
    for deg in 0..12 {
        map.insert("deg".to_string(), deg as f64);
        // When working with compiled constant expressions, you can use the
        // eval_compiled*!() macros to save a function call:
        let val = fasteval::eval_compiled!(compiled, &slab, &mut map);
        eprintln!("sin({}°) = {}", deg, val);
    }

    Ok(())
}
