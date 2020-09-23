// https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux

// Here is how I run benchmarks:
//     for F in /sys/devices/system/cpu/cpufreq/policy*/scaling_governor; do echo $F; cat $F; done
//     for F in /sys/devices/system/cpu/cpufreq/policy*/scaling_governor; do echo performance >$F; done
//
//     while true; do echo "time: $(date +%s)"; cat benches/bench.rs.tmpl | sed "s|//SHIFT_CODE|$( N=$(( 1 + $RANDOM % 1024 )); while [[ $N > 0 ]]; do N=$(( $N - 1 )); echo -n 'let x=black_box(x+1);'; done )|g" >benches/bench.rs; RUSTFLAGS="--emit=asm" cargo bench; done >bench.out
//     while true; do echo "time: $(date +%s)"; cat benches/bench.rs.tmpl | sed "s|//SHIFT_CODE|$( N=$(( 1 + $RANDOM % 1024 )); while [[ $N > 0 ]]; do N=$(( $N - 1 )); echo -n 'let x=black_box(x+1);'; done )|g" >benches/bench.rs; RUSTFLAGS="--emit=asm" cargo bench --features unsafe-vars; done >bench.out
//     cat bench.out | awk -v "now=$(date +%s)" '$1=="time:"{when=$2}  $3=="..." && $4=="bench:" {gsub(/,/, "", $5); v=$5+0; if (t[$2]=="" || v<t[$2]){t[$2]=v; w[$2]=when;}} END{for (k in t) { printf "%-40s %9d ns/iter    %5ds ago\n",k,t[k],now-w[k] }}' | sort



// ---- Results (2019-12-04 on a 2012 laptop with Intel(R) Core(TM) i7-3610QM CPU @ 2.30GHz) ----
// fasteval:
//     "(3 * (3 + 3) / 3)"
//     BTreeMap, --emit=asm:
//     ez                                             591 ns/iter      632s ago eval_ic_ref!()
//     native_1000x                                   319 ns/iter      470s ago
//     parse_compile_eval_1000x                    704383 ns/iter      789s ago
//     parse_eval_1000x                            473303 ns/iter      365s ago
//     parse_nsbubble_eval_1000x                   489977 ns/iter      272s ago
//     parser::internal_tests::spaces_1M            11629 ns/iter       50s ago
//     preparse_eval_1000x                         166261 ns/iter      184s ago
//     preparse_precompile_eval_1000x                 616 ns/iter      809s ago
//     preparse_precompile_eval_closure_1000x         617 ns/iter      463s ago
//     preparse_precompile_nsbubble_eval_1000x       9773 ns/iter      488s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             583 ns/iter     1181s ago eval_ic_ref!()
//     native_1000x                                   324 ns/iter     1651s ago
//     parse_compile_eval_1000x                    750526 ns/iter      377s ago
//     parse_eval_1000x                            480434 ns/iter      399s ago
//     parse_eval_unsafe_1000x                     482036 ns/iter     1529s ago
//     parse_nsbubble_eval_1000x                   495585 ns/iter      116s ago
//     parser::internal_tests::spaces_1M            11578 ns/iter       54s ago
//     preparse_eval_1000x                         169097 ns/iter      867s ago
//     preparse_precompile_eval_1000x                 923 ns/iter     1572s ago
//     preparse_precompile_eval_closure_1000x         923 ns/iter      905s ago
//     preparse_precompile_eval_unsafe_1000x          923 ns/iter      629s ago
//     preparse_precompile_nsbubble_eval_1000x       9770 ns/iter     1312s ago
//
//     "3 * 3 - 3 / 3"
//     BTreeMap, --emit=asm:
//     ez                                             401 ns/iter    11430s ago eval_ic_ref!()
//     native_1000x                                   318 ns/iter     5104s ago
//     parse_compile_eval_1000x                    557642 ns/iter     8997s ago
//     parse_eval_1000x                            289469 ns/iter      968s ago
//     parse_nsbubble_eval_1000x                   307711 ns/iter     5898s ago
//     parser::internal_tests::spaces_1M            11561 ns/iter     9405s ago
//     preparse_eval_1000x                          79998 ns/iter     9403s ago
//     preparse_precompile_eval_1000x                 615 ns/iter     6266s ago
//     preparse_precompile_eval_closure_1000x         615 ns/iter     6680s ago
//     preparse_precompile_nsbubble_eval_1000x       9731 ns/iter     3282s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             397 ns/iter     2473s ago eval_ic_ref!()
//     native_1000x                                   322 ns/iter     2043s ago
//     parse_compile_eval_1000x                    589180 ns/iter     2104s ago
//     parse_eval_1000x                            285362 ns/iter      947s ago
//     parse_eval_unsafe_1000x                     282690 ns/iter     1915s ago
//     parse_nsbubble_eval_1000x                   301117 ns/iter      883s ago
//     parser::internal_tests::spaces_1M            11558 ns/iter      101s ago
//     preparse_eval_1000x                          78434 ns/iter     1764s ago
//     preparse_precompile_eval_1000x                 923 ns/iter     3150s ago
//     preparse_precompile_eval_closure_1000x         920 ns/iter     1114s ago
//     preparse_precompile_eval_unsafe_1000x          923 ns/iter     3553s ago
//     preparse_precompile_nsbubble_eval_1000x       9770 ns/iter     2203s ago
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     BTreeMap, --emit=asm:
//     ez                                             422 ns/iter     1466s ago eval_ic_ref!()
//     native_1000x                                   319 ns/iter     1769s ago
//     parse_compile_eval_1000x                    436220 ns/iter     1018s ago
//     parse_eval_1000x                            323837 ns/iter      655s ago
//     parse_nsbubble_eval_1000x                   338085 ns/iter     1952s ago
//     parser::internal_tests::spaces_1M            11628 ns/iter     1204s ago
//     preparse_eval_1000x                         178083 ns/iter     2019s ago
//     preparse_precompile_eval_1000x                 618 ns/iter     1910s ago
//     preparse_precompile_eval_closure_1000x         617 ns/iter     2036s ago
//     preparse_precompile_nsbubble_eval_1000x       9773 ns/iter     1328s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             457 ns/iter      819s ago eval_ic_ref!()
//     native_1000x                                   324 ns/iter     1847s ago
//     parse_compile_eval_1000x                    448440 ns/iter     1124s ago
//     parse_eval_1000x                            320781 ns/iter      755s ago
//     parse_eval_unsafe_1000x                     320422 ns/iter      636s ago
//     parse_nsbubble_eval_1000x                   336635 ns/iter      508s ago
//     parser::internal_tests::spaces_1M            11625 ns/iter      413s ago
//     preparse_eval_1000x                         177317 ns/iter     1094s ago
//     preparse_precompile_eval_1000x                 923 ns/iter      750s ago
//     preparse_precompile_eval_closure_1000x         923 ns/iter     1456s ago
//     preparse_precompile_eval_unsafe_1000x          923 ns/iter     1662s ago
//     preparse_precompile_nsbubble_eval_1000x       9772 ns/iter     1028s ago
//
//     "x * 2"
//     BTreeMap, --emit=asm:
//     ez                                             277 ns/iter    28096s ago eval_ic_ref!()
//     native_1000x                                   679 ns/iter    29062s ago
//     parse_compile_eval_1000x                    284595 ns/iter     2736s ago
//     parse_eval_1000x                            179107 ns/iter    10798s ago
//     parse_nsbubble_eval_1000x                   260312 ns/iter     8722s ago
//     parser::internal_tests::spaces_1M            11595 ns/iter    10088s ago
//     preparse_eval_1000x                          64427 ns/iter    18578s ago
//     preparse_precompile_eval_1000x               17636 ns/iter     3662s ago
//     preparse_precompile_eval_closure_1000x       12771 ns/iter    24117s ago
//     preparse_precompile_nsbubble_eval_1000x      91065 ns/iter    26106s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             316 ns/iter     2777s ago eval_ic_ref!()
//     native_1000x                                   680 ns/iter     1623s ago
//     parse_compile_eval_1000x                    295780 ns/iter     1233s ago
//     parse_eval_1000x                            180192 ns/iter      658s ago
//     parse_eval_unsafe_1000x                     176062 ns/iter      517s ago
//     parse_nsbubble_eval_1000x                   261934 ns/iter      872s ago
//     parser::internal_tests::spaces_1M            11624 ns/iter     2486s ago
//     preparse_eval_1000x                          61570 ns/iter     2506s ago
//     preparse_precompile_eval_1000x               20450 ns/iter     1358s ago
//     preparse_precompile_eval_closure_1000x       14041 ns/iter     1282s ago
//     preparse_precompile_eval_unsafe_1000x         7939 ns/iter     1699s ago
//     preparse_precompile_nsbubble_eval_1000x      90555 ns/iter     2506s ago
//
//     "sin(x)"
//     BTreeMap, --emit=asm:
//     ez                                             357 ns/iter     1022s ago eval_ic_ref!()
//     native_1000x                                 16661 ns/iter      210s ago
//     parse_compile_eval_1000x                    288987 ns/iter      745s ago
//     parse_eval_1000x                            242037 ns/iter      759s ago
//     parse_nsbubble_eval_1000x                   323260 ns/iter     1201s ago
//     parser::internal_tests::spaces_1M            11622 ns/iter     1251s ago
//     preparse_eval_1000x                          75891 ns/iter      717s ago
//     preparse_precompile_eval_1000x               37213 ns/iter      759s ago
//     preparse_precompile_eval_closure_1000x       29987 ns/iter     1392s ago
//     preparse_precompile_nsbubble_eval_1000x     107116 ns/iter     1468s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                             380 ns/iter     3344s ago eval_ic_ref!()
//     native_1000x                                 16599 ns/iter    31069s ago
//     parse_compile_eval_1000x                    297220 ns/iter    26363s ago
//     parse_eval_1000x                            243847 ns/iter     7116s ago
//     parse_eval_unsafe_1000x                     239089 ns/iter    17857s ago
//     parse_nsbubble_eval_1000x                   323920 ns/iter    13683s ago
//     parser::internal_tests::spaces_1M            11602 ns/iter    31808s ago
//     preparse_eval_1000x                          75835 ns/iter    12617s ago
//     preparse_precompile_eval_1000x               36971 ns/iter    30510s ago
//     preparse_precompile_eval_closure_1000x       29860 ns/iter     1355s ago
//     preparse_precompile_eval_unsafe_1000x        23351 ns/iter    31685s ago
//     preparse_precompile_nsbubble_eval_1000x     105957 ns/iter     3190s ago
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     BTreeMap, --emit=asm:
//     ez                                            1407 ns/iter     1043s ago eval_ic_ref!()
//     native_1000x                                   319 ns/iter     5514s ago
//     parse_compile_eval_1000x                   2296192 ns/iter     3917s ago
//     parse_eval_1000x                           1177906 ns/iter     2428s ago
//     parse_nsbubble_eval_1000x                  1346214 ns/iter     2250s ago
//     parser::internal_tests::spaces_1M            11603 ns/iter     2576s ago
//     preparse_eval_1000x                         448529 ns/iter     1851s ago
//     preparse_precompile_eval_1000x              185588 ns/iter     2065s ago
//     preparse_precompile_eval_closure_1000x      144374 ns/iter     5135s ago
//     preparse_precompile_nsbubble_eval_1000x     342331 ns/iter     3026s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                            1456 ns/iter     1148s ago eval_ic_ref!()
//     native_1000x                                   320 ns/iter     1963s ago
//     parse_compile_eval_1000x                   2451196 ns/iter     1013s ago
//     parse_eval_1000x                           1204616 ns/iter     1148s ago
//     parse_eval_unsafe_1000x                    1189381 ns/iter      925s ago
//     parse_nsbubble_eval_1000x                  1393907 ns/iter      585s ago
//     parser::internal_tests::spaces_1M            11640 ns/iter     1257s ago
//     preparse_eval_1000x                         451742 ns/iter     1390s ago
//     preparse_precompile_eval_1000x              187067 ns/iter     1320s ago
//     preparse_precompile_eval_closure_1000x      145993 ns/iter      646s ago
//     preparse_precompile_eval_unsafe_1000x       116063 ns/iter     2332s ago
//     preparse_precompile_nsbubble_eval_1000x     348860 ns/iter      861s ago
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     BTreeMap, --emit=asm:
//     000_NAME                                      MEAN       MIN             WHEN
//     ...                                                                                with custom parser limits
//     ez                                            9539      9223 ns/iter     7304s ago normal math, indexing, slicing
//     native_1000x                                   319       318 ns/iter    26623s ago
//     parse_compile_eval_1000x                  15014684  14209543 ns/iter     4264s ago
//     parse_eval_1000x                           9531154   9098889 ns/iter    21754s ago
//     preparse_eval_1000x                        2991060   2825433 ns/iter     6808s ago
//     preparse_precompile_eval_1000x                 616       615 ns/iter    26469s ago
//     preparse_precompile_eval_closure_1000x         617       615 ns/iter    26623s ago
//     BTreeMap, --emit=asm, --features nightly:
//     ez                                            9375 ns/iter    30003s ago saturating_add, range index
//     native_1000x                                   318 ns/iter    43847s ago
//     parse_compile_eval_1000x                  14324079 ns/iter    23297s ago
//     parse_eval_1000x                           9230867 ns/iter    28870s ago
//     preparse_eval_1000x                        2857944 ns/iter    27829s ago
//     preparse_precompile_eval_1000x                 615 ns/iter    41471s ago
//     preparse_precompile_eval_closure_1000x         615 ns/iter    42477s ago
//     ez                                            9179 ns/iter     1987s ago eval_ic_ref!()
//     native_1000x                                   319 ns/iter     4215s ago
//     parse_compile_eval_1000x                  14503940 ns/iter     3584s ago
//     parse_eval_1000x                           9402799 ns/iter     4382s ago
//     parse_nsbubble_eval_1000x                  9226554 ns/iter      667s ago
//     parser::internal_tests::spaces_1M            11635 ns/iter     2790s ago
//     preparse_eval_1000x                        2874974 ns/iter     4215s ago
//     preparse_precompile_eval_1000x                 615 ns/iter     1078s ago
//     preparse_precompile_eval_closure_1000x         618 ns/iter     2790s ago
//     preparse_precompile_nsbubble_eval_1000x       9480 ns/iter     4215s ago
//     BTreeMap, --emit=asm, --features unsafe-vars:
//     ez                                            9418 ns/iter      995s ago eval_ic_ref!()
//     native_1000x                                   319 ns/iter      171s ago
//     parse_compile_eval_1000x                  15532033 ns/iter      935s ago
//     parse_eval_1000x                           9503323 ns/iter      995s ago
//     parse_eval_unsafe_1000x                    9499499 ns/iter      995s ago
//     parse_nsbubble_eval_1000x                  9629130 ns/iter     1298s ago
//     parser::internal_tests::spaces_1M            11630 ns/iter     1464s ago
//     preparse_eval_1000x                        2921451 ns/iter      995s ago
//     preparse_precompile_eval_1000x                 924 ns/iter      668s ago
//     preparse_precompile_eval_closure_1000x         924 ns/iter     1519s ago
//     preparse_precompile_eval_unsafe_1000x          923 ns/iter      579s ago
//     preparse_precompile_nsbubble_eval_1000x       9787 ns/iter     1379s ago
//
//
// python3:
//     "(3 * (3 + 3) / 3)"
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=(3 * (3 + 3) / 3)'; echo 'print(x)')  | time python3
//     7.36user 0.01system 0:07.38elapsed  -->  73.8 ns/op
//
//     "3 * 3 - 3 / 3"
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=3 * 3 - 3 / 3'; echo 'print(x)')  | time python3
//     7.20user 0.00system 0:07.21elapsed  -->  72.1 ns/op
//
//     "2 ^ 3 ^ 4"  = 2417851639229258349412352
//     user@asus:~$ ( echo 'x=[0]'; echo 'for i in range(100000000):'; echo '  x[0]=2**3**4'; echo 'print(x)')  | time python3
//     39.55user 0.00system 0:39.55elapsed  -->  395.5 ns/op
//
//     "x * 2"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=x*2'; echo 'print(_)')  | time python3
//     10.14user 0.00system 0:10.14elapsed  -->  101.4 ns/op
//
//     "sin(x)"
//     user@asus:~$ ( echo 'import math'; echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=math.sin(x)'; echo 'print(_)')  | time python3
//     19.67user 0.00system 0:19.70elapsed  -->  197 ns/op
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=(-z + (z**2 - 4*x*y)**0.5) / (2*x)'; echo 'print(_)')  | time python3
//     56.92user 0.00system 0:56.92elapsed  -->  569 ns/op
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     user@asus:~$ ( echo '_,x,y,z=[0],1,2,3'; echo 'for i in range(100000000):'; echo '  _[0]=((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))'; echo 'print(_)')  | time python3
//     7.24user 0.01system 0:07.26elapsed  -->  72.6 ns/op
//
//
// bc:
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { (3 * (3 + 3) / 3) }' | time bc >/dev/null
//     1.71user 0.32system 0:02.04elapsed  -->  2040 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { 3*3-3/3 }' | time bc >/dev/null
//     1.43user 0.22system 0:01.66elapsed  -->  1660 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { 2 ^ 3 ^ 4 }' | time bc >/dev/null = 2417851639229258349412352
//     2.33user 0.21system 0:02.55elapsed  -->  2550 ns/op
//
//     user@asus:~$ echo 'x=1; for (i=0; i<1000000; i++) { x * 2 }' | time bc >/dev/null
//     0.74user 0.27system 0:01.01elapsed  -->  1010 ns/op
//
//     user@asus:~$ echo 'x=1; for (i=0; i<1000000; i++) { s(x) }' | time bc -l >/dev/null
//     40.82user 0.40system 0:41.24elapsed  -->  41240 ns/op
//
//     user@asus:~$ echo 'x=1; y=2; z=3; for (i=0; i<1000000; i++) { (-z + sqrt(z^2 - 4*x*y)) / (2*x) }' | time bc >/dev/null
//     1.93user 0.27system 0:02.20elapsed  -->  2200 ns/op
//
//     user@asus:~$ echo 'for (i=0; i<1000000; i++) { ((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90)))) }' | time bc >/dev/null
//     10.95user 0.30system 0:11.26elapsed  -->  11260 ns/op
//
//
// caldyn:
//     "(3 * (3 + 3) / 3)", No Context
//     test ez                             ... bench:       1,191 ns/iter (+/- 315)
//     test preparse_precompile_eval_1000x ... bench:       4,193 ns/iter (+/- 217)
//
//     "(3 * (3 + 3) / 3)", Normal Context
//     test ez                             ... bench:       1,298 ns/iter (+/- 70)
//     test preparse_precompile_eval_1000x ... bench:       4,273 ns/iter (+/- 233)
//
//     "(3 * (3 + 3) / 3)", Callback Context
//     test ez                             ... bench:       1,286 ns/iter (+/- 158)
//     test preparse_precompile_eval_1000x ... bench:       4,223 ns/iter (+/- 236)
//
//     "3 * 3 - 3 / 3", Callback Context
//     test ez                             ... bench:       1,070 ns/iter (+/- 80)
//     test preparse_precompile_eval_1000x ... bench:       4,245 ns/iter (+/- 190)
//
//     "2 ^ 3 ^ 4", = 2417851639229258300000000.0, Callback Context
//     test ez                             ... bench:         867 ns/iter (+/- 75)
//     test preparse_precompile_eval_1000x ... bench:       4,182 ns/iter (+/- 238)
//
//     "x * 2", Callback Context
//     test ez                             ... bench:         607 ns/iter (+/- 61)
//     test preparse_precompile_eval_1000x ... bench:      77,540 ns/iter (+/- 12,490)
//
//     "sin(x)", Callback Context
//     test ez                             ... bench:         573 ns/iter (+/- 54)
//     test preparse_precompile_eval_1000x ... bench:      97,861 ns/iter (+/- 6,063)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)" --> -z => 0 - z
//     test ez                             ... bench:       4,440 ns/iter (+/- 618)
//     test preparse_precompile_eval_1000x ... bench:     525,066 ns/iter (+/- 64,388)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test ez                             ... bench:      24,598 ns/iter (+/- 4,140)
//     test preparse_precompile_eval_1000x ... bench:       4,418 ns/iter (+/- 429)
//
//
// tinyexpr-rs:
//     "(3 * (3 + 3) / 3)"
//     test bench_interp ... bench:       1,171 ns/iter (+/- 120)
//
//     "3 * 3 - 3 / 3"
//     test bench_interp ... bench:         895 ns/iter (+/- 50)
//
//     "2 ^ (3 ^ 4)" = 2417851639229258300000000
//     test bench_interp ... bench:         816 ns/iter (+/- 83)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test bench_interp ... bench:      38,422 ns/iter (+/- 6,510)
//
//
// tinyexpr-c:
//     "(3 * (3 + 3) / 3)"
//     te_interp  :  748 ns/iter
//     parse_compile_eval  :  762 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "3 * 3 - 3 / 3"
//     te_interp  :  615 ns/iter
//     parse_compile_eval  :  630 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "2 ^ (3 ^ 4)"  = 2417851639229258349412352.000000
//     te_interp  :  585 ns/iter
//     parse_compile_eval  :  580 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//     "x * 2"
//     parse_compile_eval  :  221 ns/iter
//     preparse_precompile_eval  :  9.4 ns/iter
//
//     "sin(x)"
//     parse_compile_eval  :  249 ns/iter
//     preparse_precompile_eval  :  21.4 ns/iter
//
//     "(-z + sqrt(z^2 - 4*x*y)) / (2*x)"
//     parse_compile_eval  :  1507 ns/iter
//     preparse_precompile_eval  :  117 ns/iter
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     te_interp  :  12,423 ns/iter
//     parse_compile_eval  :  12,222 ns/iter
//     preparse_precompile_eval  :  2.8 ns/iter
//
//
// calc:
//     "(3 * (3 + 3) / 3)"
//     test eval_1000x ... bench:   1,675,179 ns/iter (+/- 295,930)
//
//     "3 * 3 - 3 / 3"
//     test eval_1000x ... bench:   1,445,273 ns/iter (+/- 210,599)
//
//     "2 ** 3 ** 4" = 2417851639229258349412352
//     test eval_1000x ... bench:   2,275,338 ns/iter (+/- 351,933)
//
//     "x * 2"
//     test eval_1000x ... bench:     792,132 ns/iter (+/- 145,850)
//
//     "sin(x)"
//     N/A
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test eval_1000x ... bench:  26,565,727 ns/iter (+/- 3,870,655)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test eval_1000x ... bench:  44,810,253 ns/iter (+/- 5,380,532)
//
//
// meval:
//     "(3 * (3 + 3) / 3)"
//     test parse_eval    ... bench:       3,341 ns/iter (+/- 254)
//     test preparse_eval ... bench:       1,482 ns/iter (+/- 121)
//
//     "3 * 3 - 3 / 3"
//     test parse_eval    ... bench:       2,630 ns/iter (+/- 332)
//     test preparse_eval ... bench:       1,564 ns/iter (+/- 187)
//
//     "2 ^ 3 ^ 4"  = 2417851639229258300000000
//     test parse_eval    ... bench:       2,622 ns/iter (+/- 352)
//     test preparse_eval ... bench:       1,683 ns/iter (+/- 319)
//
//     "x * 2"
//     test parse_eval    ... bench:       2,289 ns/iter (+/- 344)
//     test preparse_eval ... bench:       1,484 ns/iter (+/- 80)
//
//     "sin(x)"
//     test parse_eval    ... bench:       2,476 ns/iter (+/- 323)
//     test preparse_eval ... bench:       1,521 ns/iter (+/- 166)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test parse_eval    ... bench:       5,830 ns/iter (+/- 641)
//     test preparse_eval ... bench:       1,803 ns/iter (+/- 471)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test parse_eval    ... bench:      25,371 ns/iter (+/- 8,285)
//     test preparse_eval ... bench:       2,642 ns/iter (+/- 163)
//
//
// rsc:
//     "(3 * (3 + 3) / 3)"
//     test ez            ... bench:       1,438 ns/iter (+/- 130)
//     test parse_eval    ... bench:       1,434 ns/iter (+/- 98)
//     test preparse_eval ... bench:          92 ns/iter (+/- 16)
//
//     "3 * 3 - 3 / 3"
//     test ez            ... bench:       1,291 ns/iter (+/- 150)
//     test parse_eval    ... bench:       1,330 ns/iter (+/- 464)
//     test preparse_eval ... bench:         114 ns/iter (+/- 11)
//
//     "2 ^ (3 ^ 4)"  = 2417851639229258300000000
//     test ez            ... bench:       1,283 ns/iter (+/- 141)
//     test parse_eval    ... bench:       1,306 ns/iter (+/- 113)
//     test preparse_eval ... bench:         244 ns/iter (+/- 165)
//
//     "x * 2"
//     test ez            ... N/A
//     test parse_eval    ... bench:       1,962 ns/iter (+/- 150)
//     test preparse_eval ... bench:         117 ns/iter (+/- 26)
//
//     "sin(x)"
//     test ez            ... N/A
//     test parse_eval    ... bench:       2,262 ns/iter (+/- 385)
//     test preparse_eval ... bench:         158 ns/iter (+/- 22)
//
//     "(-z + (z^2 - 4*x*y)^0.5) / (2*x)"
//     test ez            ... N/A
//     test parse_eval    ... bench:       5,808 ns/iter (+/- 499)
//     test preparse_eval ... bench:         370 ns/iter (+/- 103)
//
//     "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"
//     test ez            ... bench:      20,343 ns/iter (+/- 2,515)
//     test parse_eval    ... bench:      24,555 ns/iter (+/- 6,041)
//     test preparse_eval ... bench:       1,491 ns/iter (+/- 146)




#![feature(test)]
extern crate test;  // 'extern crate' seems to be required for this scenario: https://github.com/rust-lang/rust/issues/57288
use test::{Bencher, black_box};

use fasteval::{Parser, Compiler, Evaler, Slab, EmptyNamespace, CachedCallbackNamespace, ez_eval, eval_compiled, eval_compiled_ref};

use std::collections::BTreeMap;
use std::f64::NAN;


//fn evalcb(_:&str) -> Option<f64> { None }
fn evalcb(name:&str, args:Vec<f64>) -> Option<f64> {
    match name {
        "x" => Some(1.0),
        "y" => Some(2.0),
        "z" => Some(3.0),
        "foo" => Some(args.get(0).unwrap_or(&NAN)*10.0),
        "bar" => Some(args.get(0).unwrap_or(&NAN) + args.get(1).unwrap_or(&NAN)),
        _ => None,
    }
}

macro_rules! Namespace {
    () => {
        {
            let mut map = BTreeMap::new();
            map.insert("x".to_string(), 1.0);
            map.insert("y".to_string(), 2.0);
            map.insert("z".to_string(), 3.0);
            map
        }

        //EmptyNamespace

        //CachedCallbackNamespace::new(evalcb)

        //CachedLayeredNamespace::new(evalcb)
    }
}

macro_rules! memshift {
    () => {
        {
            let x = black_box(0);
            let x = black_box(x+1);

            //SHIFT_CODE

            black_box(x);  // Silence 'unused variable' warning.
        }
    }
}

//static EXPR : &'static str = "(3 * (3 + 3) / 3)";
static EXPR : &'static str = "3 * 3 - 3 / 3";
//static EXPR : &'static str = "2 ^ 3 ^ 4";
//static EXPR : &'static str = "x * 2";
//static EXPR : &'static str = "sin(x)";
//static EXPR : &'static str = "(-z + (z^2 - 4*x*y)^0.5) / (2*x)";
// static EXPR : &'static str = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))";

#[bench]
fn native_1000x(bencher:&mut Bencher) {
    memshift!();

    // Silence compiler warnings about unused imports:
    let _ = EmptyNamespace;  let _: CachedCallbackNamespace<f64> = CachedCallbackNamespace::new(|_,_| None);


    #[allow(dead_code)]
    fn x() -> f64 { black_box(1.0) }
    #[allow(unused_variables)]
    let (a,b,c) = (1.0f64, 3.0f64, 2.0f64);
    bencher.iter(|| {
        //let (a,b,c) = (a,b,c);  // Localize
        for _ in 0..1000 {
            //black_box(3.0 * (3.0 + 3.0) / 3.0);
            //black_box(3.0 * 3.0 - 3.0 / 3.0);
            //black_box(2.0f64.powf(3.0).powf(4.0));
            //black_box(x() * 2.0);
            //black_box(x().sin());
            //black_box( (-b + (b.powf(2.0) - 4.0*a*c).powf(0.5)) / (2.0*a) );
            black_box( ((((87.))) - 73.) + (97. + (((15. / 55. * ((31.)) + 35.))) + (15. - (9.)) - (39. / 26.) / 20. / 91. + 27. / (33. * 26. + 28. - (7.) / 10. + 66. * 6.) + 60. / 35. - ((29.) - (69.) / 44. / (92.)) / (89.) + 2. + 87. / 47. * ((2.)) * 83. / 98. * 42. / (((67.)) * ((97.))) / (34. / 89. + 77.) - 29. + 70. * (20.)) + ((((((92.))) + 23. * (98.) / (95.) + (((99.) * (41.))) + (5. + 41.) + 10.) - (36.) / (6. + 80. * 52. + (90.)))) );
        }
    });
}

#[bench]
fn ez(b:&mut Bencher) {
    memshift!();

    let mut vars=BTreeMap::new();
    vars.insert("x".to_string(),1.0);
    vars.insert("y".to_string(),2.0);
    vars.insert("z".to_string(),3.0);

    b.iter(|| {
        black_box(match ez_eval(EXPR, &mut vars) {
            Ok(f) => f,
            Err(_) => 0.0,
        });
    });
}

#[bench]
fn parse_eval_1000x(b:&mut Bencher) {
    memshift!();

    let parser = Parser::new();
    let mut slab = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                black_box(parser.parse(EXPR, &mut slab.ps)?.from(&slab.ps).eval(&slab, &mut ns)?);
            }
            Ok(())
        })();
    });
}

//// Commented until we bring CachedLayeredNamespace back.
// #[bench]
// fn parse_nsbubble_eval_1000x(b:&mut Bencher) {
//     memshift!();
//
//     let parser = Parser::new();
//     let mut slab = Slab::new();
//     let mut ns = CachedLayeredNamespace::new(evalcb);
//
//     b.iter(|| {
//         let _ = (|| -> Result<(),fasteval::Error> {
//             for _ in 0..1000 {
//                 let expr_ref = parser.parse(EXPR, &mut slab.ps)?.from(&slab.ps);
//                 let mut bub = Bubble::new(&mut ns);  bub.push();
//                 black_box( expr_ref.eval(&slab, &mut bub)? );
//             }
//             Ok(())
//         })();
//     });
// }

#[bench]
#[cfg(feature="unsafe-vars")]
fn parse_eval_unsafe_1000x(b:&mut Bencher) {
    memshift!();

    let parser = Parser::new();
    let mut slab = Slab::new();
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    let foo = 0.0;
    let bar = 0.0;
    unsafe {
        slab.ps.add_unsafe_var("x".to_string(), &x);
        slab.ps.add_unsafe_var("y".to_string(), &y);
        slab.ps.add_unsafe_var("z".to_string(), &z);
        slab.ps.add_unsafe_var("foo".to_string(), &foo);
        slab.ps.add_unsafe_var("bar".to_string(), &bar);
    }

    let mut ns = EmptyNamespace;

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                black_box(parser.parse(EXPR, &mut slab.ps)?.from(&slab.ps).eval(&slab, &mut ns)?);
            }
            Ok(())
        })();
    });
}

#[bench]
fn preparse_eval_1000x(b:&mut Bencher) {
    memshift!();

    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let expr_ref = match Parser::new().parse(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                black_box( expr_ref.eval(&slab, &mut ns)? );
            }
            Ok(())
        })();
    });
}

#[bench]
fn parse_compile_eval_1000x(b:&mut Bencher) {
    memshift!();

    let parser = Parser::new();
    let mut slab = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                let instr = parser.parse(EXPR, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                black_box(eval_compiled!(instr, &slab, &mut ns));
            }
            Ok(())
        })();
    });
}

#[bench]
fn parse_compile_eval_1000x_f32(b:&mut Bencher) {
    memshift!();

    let parser = Parser::new();
    let mut slab: Slab<f32> = Slab::new();
    let mut ns = Namespace!();

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                let instr = parser.parse(EXPR, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                black_box(eval_compiled!(instr, &slab, &mut ns));
            }
            Ok(())
        })();
    });
}

#[bench]
fn preparse_precompile_eval_1000x(b:&mut Bencher) {
    memshift!();

    let mut slab = Slab::new();
    let mut ns = Namespace!();
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            let (instr_ref, slab_ref, ns_mut) = (&instr, &slab, &mut ns);  // Localize (doesn't help much)
            for _ in 0..1000 {
                black_box( eval_compiled_ref!(instr_ref, slab_ref, ns_mut));
            }
            Ok(())
        })();
    });

    //// Produces basically the same results, proving that the --emit=asm performanace boost is not coming from this test function -- it's coming from the evaluation, and I'm not able to replicate it.
    // let _ = (|| -> Result<(),fasteval::Error> {
    //     let mut slab = Slab::new();
    //     let mut ns = Namespace!();
    //     let instr = match parse_noclear(EXPR, &mut slab.ps) {
    //         Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
    //         Err(e) => return Err(e),
    //     };
    //
    //     let start = std::time::Instant::now();
    //     for _ in 0..1_000_000 {
    //         black_box( eval_compiled_ref!(&instr, &slab, &mut ns) );
    //     }
    //     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
    //
    //     Ok(())
    // })();
}

#[bench]
fn preparse_precompile_eval_closure_1000x(b:&mut Bencher) {
    memshift!();

    let mut slab = Slab::new();
    let mut ns = evalcb;
    let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
        Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
        Err(_) => return,
    };

    b.iter(|| {
        let _ = (|| -> Result<(),fasteval::Error> {
            let (instr_ref, slab_ref, ns_mut) = (&instr, &slab, &mut ns);  // Localize (doesn't help much)
            for _ in 0..1000 {
                black_box( eval_compiled_ref!(instr_ref, slab_ref, ns_mut));
            }
            Ok(())
        })();
    });
}

//// Commented until we bring CachedLayeredNamespace back.
// #[bench]
// fn preparse_precompile_nsbubble_eval_1000x(b:&mut Bencher) {
//     memshift!();
//
//     let mut slab = Slab::new();
//     let mut ns = CachedLayeredNamespace::new(evalcb);
//     let instr = match Parser::new().parse_noclear(EXPR, &mut slab.ps) {
//         Ok(expr_i) => expr_i.from(&slab.ps).compile(&slab.ps, &mut slab.cs),
//         Err(_) => return,
//     };
//
//     b.iter(|| {
//         let _ = (|| -> Result<(),fasteval::Error> {
//             for _ in 0..1000 {
//                 let mut bub = Bubble::new(&mut ns);  bub.push();
//                 black_box( eval_compiled_ref!(&instr, &slab, &mut bub) );
//             }
//             Ok(())
//         })();
//     });
// }

#[bench]
#[cfg(feature="unsafe-vars")]
fn preparse_precompile_eval_unsafe_1000x(b:&mut Bencher) {
    memshift!();

    let mut slab = Slab::new();
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    let foo = 0.0;
    let bar = 0.0;
    unsafe {
        slab.ps.add_unsafe_var("x".to_string(), &x);
        slab.ps.add_unsafe_var("y".to_string(), &y);
        slab.ps.add_unsafe_var("z".to_string(), &z);
        slab.ps.add_unsafe_var("foo".to_string(), &foo);
        slab.ps.add_unsafe_var("bar".to_string(), &bar);
    }

    let mut ns = EmptyNamespace;
    let instr = Parser::new().parse_noclear(EXPR, &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    b.iter(|| {
        (|| -> Result<(),fasteval::Error> {
            for _ in 0..1000 {
                black_box(eval_compiled_ref!(&instr, &slab, &mut ns));
            }
            Ok(())
        })().unwrap();
    });
}

// #[bench]
// #[cfg(feature="unsafe-vars")]
// #[allow(non_snake_case)]
// fn preparse_precompile_eval_unsafe_100B(_:&mut Bencher) {
//     let _ = (|| -> Result<(),fasteval::Error> {
//         let mut slab = Slab::new();
//         let x = 1.0;
//         let y = 2.0;
//         let z = 3.0;
//         let foo = 0.0;
//         let bar = 0.0;
//         unsafe {
//             slab.ps.add_unsafe_var("x".to_string(), &x);
//             slab.ps.add_unsafe_var("y".to_string(), &y);
//             slab.ps.add_unsafe_var("z".to_string(), &z);
//             slab.ps.add_unsafe_var("foo".to_string(), &foo);
//             slab.ps.add_unsafe_var("bar".to_string(), &bar);
//         }
//
//         let mut ns = EmptyNamespace;
//         let instr = Parser::new().parse_noclear(EXPR, &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
//         eprintln!("slab: {:?}  instr: {:?}", slab, instr);
//
//         let start = std::time::Instant::now();
//         //for _ in 0..100 {
//             for _ in 0..1_000_000_000 {
//                 black_box(eval_compiled_ref!(&instr, &slab, &mut ns));
//             }
//         //}
//         eprintln!("bench time: {}", start.elapsed().as_secs_f64());
//
//         Ok(())
//     })();
// }

// #[bench]
// #[allow(non_snake_case)]
// fn preparse_compile_100M(_:&mut Bencher) {
//     let mut slab = Slab::new();
//     let expr_ref = Parser::new().parse_noclear(EXPR, &mut slab.ps).unwrap().from(&slab.ps);
//
//
//     let start = std::time::Instant::now();
//     for _ in 0..100 {
//         for _ in 0..1_000_000 {
//             slab.cs.clear();
//             black_box( expr_ref.compile(&slab.ps, &mut slab.cs) );
//         }
//     }
//     eprintln!("bench time: {}", start.elapsed().as_secs_f64());
// }

