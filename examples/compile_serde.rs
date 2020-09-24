use std::collections::BTreeMap;

use fasteval::Evaler;
use fasteval::{Compiler, Instruction, Slab};

fn main() -> Result<(), fasteval::Error> {
    let parser = fasteval::Parser::new();
    let mut slab: Slab<f64> = fasteval::Slab::new();
    // let mut map = BTreeMap::new();

    // compile an expression to an `Instruction` struct
    // let expr_str = "sin(deg/360 * 2*pi())";
    // let expr_str = "1.797693134862315743 / 2.1233215324431233423";
    let expr_str = "x * 2.466456 / y * 4.323578 * y + z";
    // let expr_str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";
    let compiled = parser
        .parse(expr_str, &mut slab.ps)?
        .from(&slab.ps)
        .compile(&slab.ps, &mut slab.cs);

    println!("slab: {:?}", slab);
    println!("instruction: {:?}", compiled);

    // serialize both the compiled instruction and the slab
    // let expr_str_ser = rmp_serde::to_vec(&expr_str).unwrap();
    // let slab_ser = rmp_serde::to_vec(&slab).unwrap();
    // let instr_ser = rmp_serde::to_vec(&compiled).unwrap();
    let expr_str_ser = bincode::serialize(&expr_str).unwrap();
    let slab_ser = bincode::serialize(&slab).unwrap();
    let instr_ser = bincode::serialize(&compiled).unwrap();

    println!(
        "serialized slab (sized {}B): {:?}",
        slab_ser.len(),
        slab_ser
    );
    println!(
        "serialized instruction (sized {}B): {:?}",
        instr_ser.len(),
        instr_ser
    );
    println!(
        "serialized expr_str (sized {}B): {:?}",
        expr_str_ser.len(),
        expr_str_ser
    );

    // deserialize
    let slab: Slab<f64> = bincode::deserialize(&slab_ser).unwrap();
    let compiled: Instruction<f64> = bincode::deserialize(&instr_ser).unwrap();

    // // eval the instruction as usual
    // for deg in 0..12 {
    //     map.insert("deg".to_string(), deg as f64);
    //     // When working with compiled constant expressions, you can use the
    //     // eval_compiled*!() macros to save a function call:
    //     let val = fasteval::eval_compiled!(compiled, &slab, &mut map);
    //     eprintln!("sin({}°) = {}", deg, val);
    // }

    Ok(())
}
