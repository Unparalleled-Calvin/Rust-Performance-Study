pub use rand::{thread_rng, Rng};

pub use paste::paste;

pub fn random_word(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| rng.gen_range(b'A'..b'Z') as char)
        .collect()
}

#[macro_export]
macro_rules! random_gen {
    { $type:ty } => { thread_rng().gen::<$type>() };
}

#[macro_export]
macro_rules! bench_gen_batch {
    { $mod:ident, $touch_func:ident, } => {};
    { $mod:ident, $touch_func:ident, $func:ident $(, $rest:ident)*} => {
        paste! {
            #[bench]
            fn [<bench_ $mod _ $func>](b: &mut Bencher) {
                $mod::$touch_func();
                b.iter(|| std::hint::black_box($mod::$func()))
            }
        }
        bench_gen_batch! {$mod, $touch_func, $($rest),*}
    };
}

#[macro_export]
macro_rules! bench_gen {
    { $mod:ident, $index:literal}  => {
        paste! {
            #[bench]
            fn [<bench_ $mod _ $index f>](b: &mut Bencher) {
                b.iter(|| std::hint::black_box($mod::[<_ $index f>]()))
            }
            #[bench]
            fn [<bench_ $mod _ $index o>](b: &mut Bencher) {
                b.iter(|| std::hint::black_box($mod::[<_ $index o>]()))
            }
        }
    };
    { $mod:ident, $index:literal, $touch_func:ident } => {
        paste! {
            #[bench]
            fn [<bench_ $mod _ $index f>](b: &mut Bencher) {
                $mod::$touch_func();
                b.iter(|| std::hint::black_box($mod::[<_ $index f>]()))
            }
            #[bench]
            fn [<bench_ $mod _ $index o>](b: &mut Bencher) {
                $mod::$touch_func();
                b.iter(|| std::hint::black_box($mod::[<_ $index o>]()))
            }
        }
    };
}
