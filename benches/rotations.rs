use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use rust_rotations::*;
// use pprof::criterion::{Output, PProfProfiler};

// use std::time::Duration;
// use std::ptr;
use std::cmp;

fn seq<const count: usize>(size: usize) -> Vec<[usize; count]> {
    let mut v = vec![[0; count]; size];
    for i in 0..size {
        v[i] = [i + 1; count];
    }
    v
}

fn test<T>(
    rotate: unsafe fn(left: usize, mid: *mut T, right: usize),
    left: usize,
    p: *mut T,
    right: usize,
) {
    unsafe { rotate(left, p, right) }
}

fn buf_test<T>(
    rotate: unsafe fn(left: usize, mid: *mut T, right: usize, buffer: &mut [T]),
    left: usize,
    p: *mut T,
    right: usize,
    buffer: &mut [T],
) {
    unsafe { rotate(left, p, right, buffer) }
}

enum Rotation {
    Direct,
    Aux,
    NaiveAux,
    Bridge,
    Contrev,
    ContrevB,
    Piston,
    GM,
    GMRec,
    Helix,
    Drill,
    Grail,
    Edge,
    Stable,
    Rev,
    RevB,
}

fn case<const count: usize>(
    name: &str,
    c: &mut Criterion,
    length: usize,
    ls: &[usize],
    rotations: Vec<Rotation>,
) {
    let mut group = c.benchmark_group(format!("{name}/{length}/{count}"));
    //    group.throughput(Throughput::Elements(length as u64));

    let mut buffer = Vec::<[usize; count]>::with_capacity(length);
    let mut v = seq::<count>(length);

    for l in ls {
        let mid = unsafe {
            let p = &v[..].as_mut_ptr().add(l.clone());
            p.clone()
        };

        let r = length - l;

        use Rotation::*;

        for rotation in &rotations {
            match rotation {
                Direct => {
                    group.bench_with_input(BenchmarkId::new("Direct", l), l, |b, _| {
                        b.iter(|| test(ptr_direct_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Contrev => {
                    group.bench_with_input(BenchmarkId::new("Contrev", l), l, |b, _| {
                        b.iter(|| test(ptr_contrev_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                ContrevB => {
                    group.bench_with_input(BenchmarkId::new("ContrevB", l), l, |b, _| {
                        b.iter(|| {
                            test(
                                ptr_block_contrev_rotate::<[usize; count]>,
                                l.clone(),
                                mid,
                                r,
                            )
                        })
                    });
                }
                GM => {
                    group.bench_with_input(BenchmarkId::new("GM", l), l, |b, _| {
                        b.iter(|| test(ptr_griesmills_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                GMRec => {
                    group.bench_with_input(BenchmarkId::new("GM (rec)", l), l, |b, _| {
                        b.iter(|| {
                            test(
                                ptr_griesmills_rotate_rec::<[usize; count]>,
                                l.clone(),
                                mid,
                                r,
                            )
                        })
                    });
                }
                Helix => {
                    group.bench_with_input(BenchmarkId::new("Helix", l), l, |b, _| {
                        b.iter(|| test(ptr_helix_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Aux => {
                    group.bench_with_input(BenchmarkId::new("Aux", l), l, |b, _| {
                        b.iter(|| {
                            buf_test(
                                ptr_aux_rotate::<[usize; count]>,
                                l.clone(),
                                mid,
                                r,
                                buffer.as_mut_slice(),
                            )
                        })
                    });
                }
                NaiveAux => {
                    group.bench_with_input(BenchmarkId::new("Aux (naive)", l), l, |b, _| {
                        b.iter(|| {
                            buf_test(
                                ptr_naive_aux_rotate::<[usize; count]>,
                                l.clone(),
                                mid,
                                r,
                                buffer.as_mut_slice(),
                            )
                        })
                    });
                }
                Bridge => {
                    let bridge = l.abs_diff(r);

                    if cmp::min(l, &r) > &bridge {
                        group.bench_with_input(BenchmarkId::new("Bridge", l), l, |b, _| {
                            b.iter(|| {
                                buf_test(
                                    ptr_bridge_rotate::<[usize; count]>,
                                    l.clone(),
                                    mid,
                                    r,
                                    buffer.as_mut_slice(),
                                )
                            })
                        });
                    };
                }
                Rev => {
                    group.bench_with_input(BenchmarkId::new("Rev", l), l, |b, _| {
                        b.iter(|| test(ptr_reversal_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                RevB => {
                    group.bench_with_input(BenchmarkId::new("RevB", l), l, |b, _| {
                        b.iter(|| {
                            test(
                                ptr_block_reversal_rotate::<[usize; count]>,
                                l.clone(),
                                mid,
                                r,
                            )
                        })
                    });
                }
                Piston => {
                    group.bench_with_input(BenchmarkId::new("Piston", l), l, |b, _| {
                        b.iter(|| test(ptr_piston_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Grail => {
                    group.bench_with_input(BenchmarkId::new("Grail", l), l, |b, _| {
                        b.iter(|| test(ptr_grail_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Drill => {
                    group.bench_with_input(BenchmarkId::new("Drill", l), l, |b, _| {
                        b.iter(|| test(ptr_drill_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Edge => {
                    group.bench_with_input(BenchmarkId::new("Edge", l), l, |b, _| {
                        b.iter(|| test(ptr_edge_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
                Stable => {
                    group.bench_with_input(BenchmarkId::new("Stable", l), l, |b, _| {
                        b.iter(|| test(stable_ptr_rotate::<[usize; count]>, l.clone(), mid, r))
                    });
                }
            }
        }
    }
    group.finish();
}

fn case_buf<const count: usize>(c: &mut Criterion, length: usize, ls: &[usize]) {
    use Rotation::*;

    case::<count>("Buf", c, length, ls, vec![Direct, NaiveAux, Aux, Bridge]);
}

fn case_rev<const count: usize>(c: &mut Criterion, length: usize, ls: &[usize]) {
    use Rotation::*;

    case::<count>("Rev", c, length, ls, vec![Direct, Rev, RevB, Bridge, Aux]);
}

fn case_contrev<const count: usize>(c: &mut Criterion, length: usize, ls: &[usize]) {
    use Rotation::*;

    case::<count>(
        "Contrev",
        c,
        length,
        ls,
        vec![Direct, Contrev, ContrevB, Bridge, Aux],
    );
}

fn case_gm<const count: usize>(c: &mut Criterion, length: usize, ls: &[usize]) {
    use Rotation::*;

    case::<count>(
        "GM",
        c,
        length,
        ls,
        vec![Direct, GM, GMRec, Grail, Drill, Aux],
    );
}

fn case_main<const count: usize>(c: &mut Criterion, length: usize, ls: &[usize]) {
    use Rotation::*;

    case::<count>(
        "Main",
        c,
        length,
        ls,
        vec![Direct, Contrev, GM, Helix, Piston, Rev, Aux, Bridge],
    );
}

fn case_short<const count: usize>(c: &mut Criterion, length: usize) {
    let ls: Vec<usize> = (0..=length).collect();

    case_main::<count>(c, length, &ls);
}

fn bench_short(c: &mut Criterion) {
    seq_macro::seq!(i in 1..=3 {
        for l in 5..=40 {
            case_short::<i>(c, l);
        }
    });

    for l in 5..=40 {
        case_short::<5>(c, l);
    }

    for l in 5..=40 {
        case_short::<10>(c, l);
    }

    for l in 5..=40 {
        case_short::<20>(c, l);
    }

    for l in 5..=40 {
        case_short::<40>(c, l);
    }

    for l in 5..=40 {
        case_short::<80>(c, l);
    }
}

fn bench_buf(c: &mut Criterion) {
    // 1 * usize
    case_buf::<1>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_buf::<1>(c, 30, &[1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 29]);

    case_buf::<1>(c, 100, &[1, 20, 32, 35, 40, 45, 51, 60, 66, 69, 80, 90, 99]);
    case_buf::<1>(
        c,
        1000,
        &[1, 32, 200, 334, 400, 485, 516, 668, 800, 900, 969, 999],
    );
    case_buf::<1>(
        c,
        10000,
        &[
            1, 32, 2000, 3334, 4000, 4985, 5016, 6668, 8000, 9000, 9969, 9999,
        ],
    );

    // 5 * usize, possible buffer size = 6 elements of 5 * usize
    case_buf::<5>(c, 6, &[1, 2, 3, 4, 5]);

    case_buf::<5>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_buf::<5>(c, 30, &[1, 3, 6, 10, 13, 16, 18, 22, 25, 27, 29]);
    case_buf::<5>(c, 100, &[1, 6, 20, 30, 40, 48, 53, 60, 70, 80, 90, 95, 99]);
    case_buf::<5>(
        c,
        1000,
        &[1, 6, 200, 334, 400, 498, 503, 668, 800, 900, 995, 999],
    );
    case_buf::<5>(
        c,
        10000,
        &[
            1, 6, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9995, 9999,
        ],
    );
}

fn bench_contrev(c: &mut Criterion) {
    // 1 * usize
    case_contrev::<1>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_contrev::<1>(c, 30, &[1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 29]);

    case_contrev::<1>(c, 100, &[1, 20, 32, 35, 40, 45, 51, 60, 66, 69, 80, 90, 99]);
    case_contrev::<1>(
        c,
        1000,
        &[1, 32, 200, 334, 400, 485, 516, 668, 800, 900, 969, 999],
    );
    case_contrev::<1>(
        c,
        10000,
        &[
            1, 32, 2000, 3334, 4000, 4985, 5016, 6668, 8000, 9000, 9969, 9999,
        ],
    );

    // 2 * usize, possible buffer size = 16 elements of 2 * usize
    case_contrev::<2>(c, 6, &[1, 2, 3, 4, 5]);

    case_contrev::<2>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_contrev::<2>(c, 30, &[1, 3, 6, 10, 13, 16, 18, 22, 25, 27, 29]);
    case_contrev::<2>(c, 100, &[1, 16, 20, 30, 40, 48, 53, 60, 70, 80, 85, 99]);
    case_contrev::<2>(
        c,
        1000,
        &[1, 16, 200, 334, 400, 498, 503, 668, 800, 900, 985, 999],
    );
    case_contrev::<2>(
        c,
        10000,
        &[
            1, 16, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9985, 9999,
        ],
    );

    // 5 * usize, possible buffer size = 6 elements of 5 * usize
    case_contrev::<5>(c, 6, &[1, 2, 3, 4, 5]);

    case_contrev::<5>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_contrev::<5>(c, 30, &[1, 3, 6, 10, 13, 16, 18, 22, 25, 27, 29]);
    case_contrev::<5>(c, 100, &[1, 6, 20, 30, 40, 48, 53, 60, 70, 80, 90, 95, 99]);
    case_contrev::<5>(
        c,
        1000,
        &[1, 6, 200, 334, 400, 498, 503, 668, 800, 900, 995, 999],
    );
    case_contrev::<5>(
        c,
        10000,
        &[
            1, 6, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9995, 9999,
        ],
    );
}

fn bench_rev(c: &mut Criterion) {
    // 1 * usize
    case_rev::<1>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_rev::<1>(c, 30, &[2, 4, 7, 10, 13, 16, 19, 22, 25, 28]);

    case_rev::<1>(c, 100, &[2, 20, 32, 35, 40, 45, 51, 60, 66, 69, 80, 90, 98]);
    case_rev::<1>(
        c,
        1000,
        &[
            2, 32, 200, 334, 375, 400, 485, 516, 550, 668, 800, 900, 969, 998,
        ],
    );
    case_rev::<1>(
        c,
        10000,
        &[
            2, 32, 2000, 3334, 4000, 4985, 5016, 6668, 8000, 9000, 9969, 9998,
        ],
    );

    // 2 * usize, possible buffer size = 16 elements of 2 * usize
    case_rev::<2>(c, 6, &[1, 2, 3, 4, 5]);

    case_rev::<2>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_rev::<2>(c, 30, &[2, 3, 6, 10, 13, 16, 18, 22, 25, 27, 28]);
    case_rev::<2>(c, 100, &[2, 16, 20, 30, 40, 48, 53, 60, 70, 80, 85, 98]);
    case_rev::<2>(
        c,
        1000,
        &[2, 16, 200, 334, 400, 498, 503, 668, 800, 900, 985, 998],
    );
    case_rev::<2>(
        c,
        10000,
        &[
            2, 16, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9985, 9998,
        ],
    );

    // 5 * usize, possible buffer size = 6 elements of 5 * usize
    case_rev::<5>(c, 6, &[1, 2, 3, 4, 5]);

    case_rev::<5>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_rev::<5>(c, 30, &[2, 3, 6, 10, 13, 16, 18, 22, 25, 27, 28]);
    case_rev::<5>(c, 100, &[2, 6, 20, 30, 40, 48, 53, 60, 70, 80, 90, 95, 98]);
    case_rev::<5>(
        c,
        1000,
        &[2, 6, 200, 334, 400, 498, 503, 668, 800, 900, 995, 998],
    );
    case_rev::<5>(
        c,
        10000,
        &[
            2, 6, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9995, 9998,
        ],
    );
}

fn bench_gm(c: &mut Criterion) {
    // 1 * usize
    case_gm::<1>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_gm::<1>(c, 30, &[2, 4, 7, 10, 13, 16, 19, 22, 25, 28]);

    case_gm::<1>(c, 100, &[2, 20, 32, 35, 40, 45, 51, 60, 66, 69, 80, 90, 98]);
    case_gm::<1>(
        c,
        1000,
        &[2, 32, 200, 334, 400, 485, 516, 668, 800, 900, 969, 998],
    );
    case_gm::<1>(
        c,
        10000,
        &[
            2, 32, 2000, 3334, 4000, 4985, 5016, 6668, 8000, 9000, 9969, 9998,
        ],
    );

    // 2 * usize, possible buffer size = 16 elements of 2 * usize
    case_gm::<2>(c, 6, &[1, 2, 3, 4, 5]);

    case_gm::<2>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_gm::<2>(c, 30, &[2, 3, 6, 10, 13, 16, 18, 22, 25, 27, 28]);
    case_gm::<2>(c, 100, &[2, 16, 20, 30, 40, 48, 53, 60, 70, 80, 85, 98]);
    case_gm::<2>(
        c,
        1000,
        &[2, 16, 200, 334, 400, 498, 503, 668, 800, 900, 985, 998],
    );
    case_gm::<2>(
        c,
        10000,
        &[
            2, 16, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9985, 9998,
        ],
    );

    // 5 * usize, possible buffer size = 6 elements of 5 * usize
    case_gm::<5>(c, 6, &[1, 2, 3, 4, 5]);

    case_gm::<5>(c, 15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    case_gm::<5>(c, 30, &[2, 3, 6, 10, 13, 16, 18, 22, 25, 27, 28]);
    case_gm::<5>(c, 100, &[2, 6, 20, 30, 40, 48, 53, 60, 70, 80, 90, 95, 98]);
    case_gm::<5>(
        c,
        1000,
        &[2, 6, 200, 334, 400, 498, 503, 668, 800, 900, 995, 998],
    );
    case_gm::<5>(
        c,
        10000,
        &[
            2, 6, 200, 3334, 4000, 4998, 5003, 6668, 8000, 9000, 9995, 9998,
        ],
    );
}

criterion_group! {
    name = benches;

//    config = Criterion::default().sample_size(500).measurement_time(Duration::new(120, 0));
    config = Criterion::default()
             .sample_size(1000);
             // .with_profiler(
             //      PProfProfiler::new(100, Output::Flamegraph(None))
             //  );

    targets = bench_buf, bench_contrev, bench_rev, bench_gm, bench_short
}

criterion_main!(benches);
