#![feature(test)]
extern crate test;

mod checking;
mod data_collections_inefficiency;
mod inadequate_concurrency;
mod iterator_misusing;
mod memory_cloning;
mod repeated_computation;
mod trait_implementations_untuned;

mod utils;

fn main() {
    println!("This is a repository for bench mark!");
}

#[cfg(test)]
mod benchmarks {
    use test::Bencher;

    use super::checking::{borrowing_checking, bounds_checking, encoding_checking};

    use super::memory_cloning::{
        cloned_object_used_as_immutable, continuous_data_collection_conversion,
    };

    use super::iterator_misusing::{
        allocating_in_flatmap_then_collecting, incorrect_iterator_type,
    };

    use super::data_collections_inefficiency::{
        data_collection_initialization, data_collection_reallocation,
        functionally_matched_but_suboptimal, functionally_unmatched,
    };

    use super::trait_implementations_untuned::{
        auxiliary_traits_without_overriding, generic_traits_without_specialization,
        hash_traits_auto_derived_by_the_compiler,
    };

    use super::inadequate_concurrency::{divisible_serial_computation, lock_contention};

    use super::repeated_computation::{local_level, nonlocal_level};

    use crate::{bench_gen, bench_gen_batch, utils::*};

    bench_gen! {bounds_checking, 1, touch_data}
    bench_gen! {bounds_checking, 2, touch_data}
    bench_gen! {borrowing_checking, 1}
    bench_gen! {borrowing_checking, 2}
    bench_gen! {encoding_checking, 1}
    bench_gen! {encoding_checking, 2}
    bench_gen! {cloned_object_used_as_immutable, 1, touch_n}
    bench_gen! {cloned_object_used_as_immutable, 2, touch_n}
    bench_gen! {continuous_data_collection_conversion, 1, touch_s}
    bench_gen! {continuous_data_collection_conversion, 2, touch_s}
    bench_gen! {incorrect_iterator_type, 1, touch_routes}
    bench_gen! {allocating_in_flatmap_then_collecting, 1, touch_pixels}
    bench_gen! {allocating_in_flatmap_then_collecting, 2, touch_pixels}
    bench_gen! {functionally_unmatched, 1, touch_names}
    bench_gen! {functionally_matched_but_suboptimal, 1, touch_data}
    bench_gen! {functionally_matched_but_suboptimal, 2, touch_data}
    bench_gen! {data_collection_initialization, 1, touch_blocks}
    bench_gen! {data_collection_initialization, 2}
    bench_gen! {data_collection_reallocation, 1, touch_data}
    bench_gen! {data_collection_reallocation, 2, touch_data}
    bench_gen! {hash_traits_auto_derived_by_the_compiler, 1, touch_bars}
    bench_gen! {auxiliary_traits_without_overriding, 1, touch_foo}
    bench_gen! {generic_traits_without_specialization, 1, touch_others}
    bench_gen! {divisible_serial_computation, 1, touch_m}
    bench_gen! {lock_contention, 1}
    bench_gen! {local_level, 1, touch_lookup}
    bench_gen! {nonlocal_level, 1, touch_data}
    bench_gen_batch! {functionally_matched_but_suboptimal, touch_data, f1, f2, f3, f4}
}
