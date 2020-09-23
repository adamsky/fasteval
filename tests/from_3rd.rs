use fasteval::{eval_compiled_ref, CachedCallbackNamespace, Compiler, Error, Evaler, Parser, Slab};

use std::str::from_utf8;

fn evalns_cb<T: fasteval::Num>(name: &str, args: Vec<T>) -> Option<T> {
    match name {
        "w" => T::from(0.0),
        "x" => T::from(1.0),
        "y" => T::from(2.0),
        "y7" => T::from(2.7),
        "z" => T::from(3.0),
        "foo" => Some(args[0] * T::from(10.0).unwrap()),
        "bar" => Some(args[0] + args[1]),
        _ => None,
    }
}

fn chk_ok<T: fasteval::Num>(
    expr_str: &str,
    expect_compile_str: &str,
    expect_slab_str: &str,
    expect_eval: T,
) {
    let mut slab = Slab::new();
    let expr = Parser::new()
        .parse::<T>(expr_str, &mut slab.ps)
        .unwrap()
        .from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);

    assert_eq!(format!("{:?}", instr), expect_compile_str);
    assert_eq!(format!("{:?}", slab), expect_slab_str);

    (|| -> Result<(), Error> {
        let mut ns = CachedCallbackNamespace::new(evalns_cb);
        assert_eq!(eval_compiled_ref!(&instr, &slab, &mut ns), expect_eval);

        // Make sure Instruction eval matches normal eval:
        assert_eq!(
            eval_compiled_ref!(&instr, &slab, &mut ns),
            expr.eval(&slab, &mut ns).unwrap()
        );

        Ok(())
    })()
    .unwrap();
}

fn chk_perr<T: fasteval::Num>(expr_str: &str, expect_err: Error) {
    let mut slab = Slab::new();
    let res = Parser::new().parse::<T>(expr_str, &mut slab.ps);
    assert_eq!(res, Err(expect_err));
}

fn chk_eerr<T: fasteval::Num>(expr_str: &str, expect_err: Error) {
    let mut slab = Slab::new();
    let expr = Parser::new()
        .parse::<T>(expr_str, &mut slab.ps)
        .unwrap()
        .from(&slab.ps);
    let instr = expr.compile(&slab.ps, &mut slab.cs);
    let mut ns = CachedCallbackNamespace::new(evalns_cb);
    assert_eq!(instr.eval(&slab, &mut ns), Err(expect_err));
}

#[test]
fn meval() {
    chk_perr::<f64>("", Error::EofWhileParsing("value".to_string()));
    chk_perr::<f64>("(", Error::EofWhileParsing("value".to_string()));
    chk_perr::<f64>("0(", Error::UnparsedTokensRemaining("(".to_string()));
    chk_eerr::<f64>("e", Error::Undefined("e".to_string()));
    chk_perr::<f64>("1E", Error::ParseFloat("1E".to_string()));
    chk_perr::<f64>("1e+", Error::ParseFloat("1e+".to_string()));
    chk_perr::<f64>("()", Error::InvalidValue);
    chk_perr::<f64>("2)", Error::UnparsedTokensRemaining(")".to_string()));
    chk_perr::<f64>("2^", Error::EofWhileParsing("value".to_string()));
    chk_perr::<f64>("(((2)", Error::EofWhileParsing("parentheses".to_string()));
    chk_perr::<f64>("f(2,)", Error::InvalidValue);
    chk_perr::<f64>("f(,2)", Error::InvalidValue);

    chk_ok("round(sin (pi()) * cos(0))",
"IConst(0.0)",
"Slab{ exprs:{ 0:Expression { first: EStdFunc(EFuncPi), pairs: [] }, 1:Expression { first: EConstant(0.0), pairs: [] }, 2:Expression { first: EStdFunc(EFuncSin(ExpressionI(0))), pairs: [ExprPair(EMul, EStdFunc(EFuncCos(ExpressionI(1))))] }, 3:Expression { first: EStdFunc(EFuncRound { modulus: None, expr: ExpressionI(2) }), pairs: [] } }, vals:{}, instrs:{} }",
0.0);

    chk_ok("max(1.)",
"IConst(1.0)",
"Slab{ exprs:{ 0:Expression { first: EConstant(1.0), pairs: [] }, 1:Expression { first: EStdFunc(EFuncMax { first: ExpressionI(0), rest: [] }), pairs: [] } }, vals:{}, instrs:{} }",
1.0);

    chk_ok("max(1., 2., -1)",
"IConst(2.0)",
"Slab{ exprs:{ 0:Expression { first: EConstant(1.0), pairs: [] }, 1:Expression { first: EConstant(2.0), pairs: [] }, 2:Expression { first: EConstant(-1.0), pairs: [] }, 3:Expression { first: EStdFunc(EFuncMax { first: ExpressionI(0), rest: [ExpressionI(1), ExpressionI(2)] }), pairs: [] } }, vals:{}, instrs:{} }",
2.0);

    chk_ok("sin(1.) + cos(2.)",
"IConst(0.4253241482607541)",
"Slab{ exprs:{ 0:Expression { first: EConstant(1.0), pairs: [] }, 1:Expression { first: EConstant(2.0), pairs: [] }, 2:Expression { first: EStdFunc(EFuncSin(ExpressionI(0))), pairs: [ExprPair(EAdd, EStdFunc(EFuncCos(ExpressionI(1))))] } }, vals:{}, instrs:{} }",
(1f64).sin() + (2f64).cos());
}

#[test]
fn overflow_stack() {
    chk_perr::<f64>(
        from_utf8(&[b'('; 1]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(
        from_utf8(&[b'('; 2]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(
        from_utf8(&[b'('; 4]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(
        from_utf8(&[b'('; 8]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(
        from_utf8(&[b'('; 16]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(
        from_utf8(&[b'('; 32]).unwrap(),
        Error::EofWhileParsing("value".to_string()),
    );
    chk_perr::<f64>(from_utf8(&[b'('; 33]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 64]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 128]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 256]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 512]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 1024]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 2048]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 4096]).unwrap(), Error::TooDeep);
    chk_perr::<f64>(from_utf8(&[b'('; 8192]).unwrap(), Error::TooLong);

    // Test custom safety parse limits:
    assert_eq!(
        Parser {
            expr_len_limit: fasteval::parser::DEFAULT_EXPR_LEN_LIMIT,
            expr_depth_limit: 31
        }
        .parse::<f64>(from_utf8(&[b'('; 32]).unwrap(), &mut Slab::new().ps),
        Err(Error::TooDeep)
    );

    assert_eq!(
        Parser {
            expr_len_limit: 8,
            expr_depth_limit: fasteval::parser::DEFAULT_EXPR_DEPTH_LIMIT
        }
        .parse::<f64>(from_utf8(&[b'('; 32]).unwrap(), &mut Slab::new().ps),
        Err(Error::TooLong)
    );
}
