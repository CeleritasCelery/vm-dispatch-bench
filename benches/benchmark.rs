use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_match_vm(c: &mut Criterion) {
    let all = matches::get_values();
    c.bench_function("match_vm", |b| b.iter(|| matches::run_vm(black_box(&all))));
}

fn bench_fn_vm(c: &mut Criterion) {
    let all = fnptr::get_values();
    c.bench_function("fn_vm", |b| b.iter(|| fnptr::run_vm(black_box(&all))));
}

criterion_group!(benches, bench_match_vm, bench_fn_vm);
criterion_main!(benches);

mod fnptr {
    type Func = fn(&mut State);

    pub(crate) struct State<'a> {
        count: usize,
        index: usize,
        functions: &'a [Func],
    }

    impl State<'_> {
        #[inline(always)]
        fn next_func(&mut self) {
            let next = self.functions[self.index];
            self.index += 1;
            next(self);
        }
    }

    fn func1(s: &mut State) {
        s.count += 1;
        s.next_func();
    }

    fn func2(s: &mut State) {
        s.count += 2;
        s.next_func();
    }

    fn func3(s: &mut State) {
        s.count += 3;
        s.next_func();
    }

    fn func4(s: &mut State) {
        s.count += 4;
        s.next_func();
    }

    fn func_return(_: &mut State) {
        // do nothing
    }

    #[rustfmt::skip]
    pub(crate) fn get_values() -> Vec<Func> {
        vec![
            func1, func2, func3, func4,
             func1, func2, func3, func4,
             func1, func2, func3, func4,
             func1, func2, func3, func4,
             func1, func2, func3, func4,
             func_return]
    }

    pub(crate) fn run_vm(functions: &[Func]) -> usize {
        let mut s = State {
            count: 0,
            index: 0,
            functions,
        };
        s.next_func();
        s.count
    }
}

mod matches {
    struct State {
        count: usize,
    }

    fn func1(s: &mut State) {
        s.count += 1;
    }

    fn func2(s: &mut State) {
        s.count += 2;
    }

    fn func3(s: &mut State) {
        s.count += 3;
    }

    fn func4(s: &mut State) {
        s.count += 4;
    }

    fn func_return() {
        // do nothing
    }

    pub(crate) enum Ops {
        Func1,
        Func2,
        Func3,
        Func4,
        FuncReturn,
    }

    #[rustfmt::skip]
    pub(crate) fn get_values() -> Vec<Ops> {
        vec![Ops::Func1, Ops::Func2, Ops::Func3, Ops::Func4,
             Ops::Func1, Ops::Func2, Ops::Func3, Ops::Func4,
             Ops::Func1, Ops::Func2, Ops::Func3, Ops::Func4,
             Ops::Func1, Ops::Func2, Ops::Func3, Ops::Func4,
             Ops::Func1, Ops::Func2, Ops::Func3, Ops::Func4,
             Ops::FuncReturn]
    }

    pub(crate) fn run_vm(vm: &[Ops]) -> usize {
        let s = &mut State { count: 0 };
        for op in vm {
            match op {
                Ops::Func1 => func1(s),
                Ops::Func2 => func2(s),
                Ops::Func3 => func3(s),
                Ops::Func4 => func4(s),
                Ops::FuncReturn => func_return(),
            }
        }
        s.count
    }
}
