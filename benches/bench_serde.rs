#![feature(test)]
extern crate test; // 'extern crate' seems to be required for this scenario: https://github.com/rust-lang/rust/issues/57288
use test::{black_box, Bencher};

use fasteval::{
    eval_compiled, eval_compiled_ref, ez_eval, CachedCallbackNamespace, Compiler, EmptyNamespace,
    Evaler, Parser, Slab,
};

use std::collections::BTreeMap;
use std::f64::NAN;

//fn evalcb(_:&str) -> Option<f32> { None }
fn evalcb(name: &str, args: Vec<f64>) -> Option<f64> {
    match name {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        "foo" => Some(args.get(0).unwrap_or(&NAN) * 10.0),
        "bar" => Some(args.get(0).unwrap_or(&NAN) + args.get(1).unwrap_or(&NAN)),
        _ => None,
    }
}

macro_rules! Namespace {
    () => {{
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), 1.0);
        map.insert("y".to_string(), 2.0);
        map.insert("z".to_string(), 3.0);
        map
    }}; //EmptyNamespace

        //CachedCallbackNamespace::new(evalcb)

        //CachedLayeredNamespace::new(evalcb)};
}

macro_rules! memshift {
    () => {{
        let x = black_box(0);
        let x = black_box(x + 1);

        //SHIFT_CODE

        black_box(x); // Silence 'unused variable' warning.
    }};
}

//static EXPR : &'static str = "(3 * (3 + 3) / 3)";
// static EXPR: &'static str = "3 * 3 - 3 / 3";
//static EXPR : &'static str = "2 ^ 3 ^ 4";
static EXPR: &'static str = "fdsdasdgddsaf * 2343.344 * fdsdasdgddsaf ((23.22323 / fdsdasdgddsaf / 4.323552) + (987453.2 * asddsasfasfasadsawrasd ^ 34.34234))";
//static EXPR : &'static str = "sin(x)";
// static EXPR: &'static str = "(-z + (z^2 - 4*x*y)^0.5) / (2*x)";
// static EXPR: &'static str = "1.797643 / 2.12343423 + 2.1234343223 + fdsdasdgddsaf + dsdsdsadffdsdg";
// static EXPR : &'static str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";

#[bench]
fn serialize_compiled_instruction_1000x_bincode_f32(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f32> = Slab::new();
    // let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    bincode::serialize(&slab).unwrap();
                    bincode::serialize(&instr).unwrap();
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn serialize_compiled_instruction_1000x_bincode(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    // let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    bincode::serialize(&slab).unwrap();
                    bincode::serialize(&instr).unwrap();
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn serialize_compiled_instruction_1000x_rmp(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    // let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    rmp_serde::to_vec(&slab).unwrap();
                    rmp_serde::to_vec(&instr).unwrap();
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn parse_compile_serialize_instruction_1000x_rmp(b: &mut Bencher) {
    memshift!();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    let mut slab: Slab<f64> = Slab::new();
                    // let mut ns = Namespace!();
                    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
                        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
                        Err(e) => return Err(e),
                    };
                    rmp_serde::to_vec(&instr).unwrap();
                    rmp_serde::to_vec(&slab).unwrap();
                });
            }
            Ok(())
        })();
    });
}

#[bench]
fn deserialize_compiled_instruction_1000x_bincode_f32(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f32> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let instr_ser = bincode::serialize(&instr).unwrap();
    let slab_ser = bincode::serialize(&slab).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    bincode::deserialize::<fasteval::Instruction<f32>>(&instr_ser).unwrap();
                    bincode::deserialize::<fasteval::Slab<f32>>(&slab_ser).unwrap();
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn deserialize_compiled_instruction_1000x_bincode(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let instr_ser = bincode::serialize(&instr).unwrap();
    let slab_ser = bincode::serialize(&slab).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    bincode::deserialize::<fasteval::Instruction<f64>>(&instr_ser).unwrap();
                    bincode::deserialize::<fasteval::Slab<f64>>(&slab_ser).unwrap();
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn deserialize_compiled_instruction_1000x_rmp(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let instr_ser = rmp_serde::to_vec(&instr).unwrap();
    let slab_ser = rmp_serde::to_vec(&slab).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    rmp_serde::from_slice::<fasteval::Instruction<f64>>(&instr_ser).unwrap();
                    rmp_serde::from_slice::<fasteval::Slab<f64>>(&slab_ser).unwrap();
                });
            }
            Ok(())
        })();
    });
}

#[bench]
fn serialize_expr_1000x_bincode(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let expr = String::from(EXPR);

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box(bincode::serialize(&expr).unwrap());
            }
            Ok(())
        })();
    });
}
#[bench]
fn serialize_expr_1000x_rmp(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let expr = String::from(EXPR);

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box(rmp_serde::to_vec(&expr).unwrap());
            }
            Ok(())
        })();
    });
}
#[bench]
fn deserialize_expr_1000x_rmp(b: &mut Bencher) {
    memshift!();

    let mut slab: Slab<f64> = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };
    let serialized = rmp_serde::to_vec(&EXPR).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box(rmp_serde::from_slice::<String>(&serialized).unwrap());
            }
            Ok(())
        })();
    });
}

#[bench]
fn deserialize_expr_compile_instruction_1000x_bincode(b: &mut Bencher) {
    memshift!();

    // let mut slab = Slab::new();
    // let mut ns = Namespace!();
    // let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
    //     Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
    //     Err(_) => return,
    // };
    let serialized = bincode::serialize(&EXPR).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    let expr: String = bincode::deserialize(&serialized).unwrap();
                    let mut slab: Slab<f64> = Slab::new();
                    // let mut ns = Namespace!();
                    match Parser::new().parse(&expr, &mut slab.ps) {
                        Ok(expr_i) => {
                            expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                            ()
                        }
                        Err(_) => (),
                    }
                });
            }
            Ok(())
        })();
    });
}
#[bench]
fn deserialize_expr_compile_instruction_1000x_rmp(b: &mut Bencher) {
    memshift!();

    // let mut slab = Slab::new();
    // let mut ns = Namespace!();
    // let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
    //     Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
    //     Err(_) => return,
    // };
    let serialized = rmp_serde::to_vec(&EXPR).unwrap();

    b.iter(|| {
        let _ = (|| -> Result<(), fasteval::Error> {
            for _ in 0..1000 {
                black_box({
                    let expr: String = rmp_serde::from_slice(&serialized).unwrap();
                    let mut slab: Slab<f64> = Slab::new();
                    // let mut ns = Namespace!();
                    match Parser::new().parse(&expr, &mut slab.ps) {
                        Ok(expr_i) => {
                            expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                            ()
                        }
                        Err(_) => (),
                    }
                });
            }
            Ok(())
        })();
    });
}
