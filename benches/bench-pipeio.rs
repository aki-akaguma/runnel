use criterion::{criterion_group, criterion_main, Criterion};
use runnel::medium::pipeio::*;
use runnel::*;
use std::io::BufRead;
use std::io::Write;

fn process_one(cnt: usize) -> () {
    let (sout1, sin1) = pipe(1);
    let (sout2, sin2) = pipe(1);
    let (sout3, sin3) = pipe(1);
    let (sout4, sin4) = pipe(1);
    //
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pout(sout1).build();
    let handler1 = std::thread::spawn(move || {
        let mut i=0;
        loop {
            i += 1;
            if i > cnt {
                break;
            }
            let line = i.to_string().repeat(200);
            sioe.pout().lock().write_fmt(format_args!("{}\n", line)).unwrap();
        }
        sioe.pout().lock().flush().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pin(sin1).pout(sout2).build();
    let handler2 = std::thread::spawn(move || {
        for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pout().lock().write_fmt(format_args!("{}\n", line)).unwrap();
        }
        sioe.pout().lock().flush().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pin(sin2).pout(sout3).build();
    let handler3 = std::thread::spawn(move || {
        for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pout().lock().write_fmt(format_args!("{}\n", line)).unwrap();
        }
        sioe.pout().lock().flush().unwrap();
    });
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pin(sin3).pout(sout4).build();
    let handler4 = std::thread::spawn(move || {
        for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
            // nothing todo
            sioe.pout().lock().write_fmt(format_args!("{}\n", line)).unwrap();
        }
        sioe.pout().lock().flush().unwrap();
    });
    //
    #[rustfmt::skip]
    let sioe = RunnelIoeBuilder::new().pin(sin4).build();
    for _line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
        // nothing todo
    }
    let _ = handler1.join();
    let _ = handler2.join();
    let _ = handler3.join();
    let _ = handler4.join();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pipeio::", |b| {
        b.iter(|| {
            let _r = process_one(criterion::black_box(8*4*1024));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
//
