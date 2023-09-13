use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn get_bunch_of_strings() -> Vec<String> {
    let files = glob::glob("fixtures/threejs_src/**/*.js").unwrap();
    let mut files = files
        .into_iter()
        .map(|p| p.unwrap().canonicalize().unwrap())
        .collect::<Vec<_>>();
    files.sort();
    let stirngs = files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect::<Vec<_>>();

    let mut ret = vec![];
    for _ in 0..10 {
        ret.extend(stirngs.clone());
    }
    ret
}

fn criterion_benchmark(c: &mut Criterion) {
    let bunch_of_strings = get_bunch_of_strings();
  
    let mut joiner = string_wizard::Joiner::new();
    bunch_of_strings.clone().into_iter().for_each(|s| {
        joiner.append_raw(s);
    });
    c.bench_function("Joiner#join", |b| b.iter(|| black_box(joiner.join())));
    c.bench_function("Vec#concat", |b| {
        b.iter(|| black_box(bunch_of_strings.concat()))
    });
    c.bench_function("manual_push", |b| {
        b.iter(|| {
            let mut output = String::new();
            bunch_of_strings.iter().for_each(|s| {
                output.push_str(s);
            });
            black_box(output)
        })
    });
    c.bench_function("manual_push_with_cap", |b| {
        b.iter(|| {
            let cap: usize = bunch_of_strings.iter().map(|s| s.len()).sum();
            let mut output = String::with_capacity(cap);
            bunch_of_strings.iter().for_each(|s| {
                output.push_str(s);
            });
            black_box(output)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
