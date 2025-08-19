use criterion::{criterion_group, criterion_main, Criterion};
use runnel::medium::linepipeio::*;
use runnel::*;

fn process_one(cnt: usize) -> () {
    let (sout1, sin1) = line_pipe(1);
    let (sout2, sin2) = line_pipe(1);
    let (sout3, sin3) = line_pipe(1);
    let (sout4, sin4) = line_pipe(1);
    //
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pg_out(sout1).build();
    let handler1 = std::thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            if i > cnt {
                break;
            }
            let line = i.to_string().repeat(200);
            sioe.pg_out().write_line(line).unwrap();
        }
        sioe.pg_out().flush_line().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pg_in(sin1).pg_out(sout2).build();
    let handler2 = std::thread::spawn(move || {
        for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pg_out().write_line(line).unwrap();
        }
        sioe.pg_out().flush_line().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pg_in(sin2).pg_out(sout3).build();
    let handler3 = std::thread::spawn(move || {
        for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pg_out().write_line(line).unwrap();
        }
        sioe.pg_out().flush_line().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pg_in(sin3).pg_out(sout4).build();
    let handler4 = std::thread::spawn(move || {
        for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pg_out().write_line(line).unwrap();
        }
        sioe.pg_out().flush_line().unwrap();
    });
    //
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pg_in(sin4).build();
    for _line in sioe.pg_in().lines().map(|l| l.unwrap()) {
        // nothing todo
    }
    let _ = handler1.join();
    let _ = handler2.join();
    let _ = handler3.join();
    let _ = handler4.join();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("linepipeio::", |b| {
        b.iter(|| {
            let _r = process_one(std::hint::black_box(8 * 4 * 1024));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(20000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
