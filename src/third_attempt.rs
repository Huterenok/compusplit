// After a few hours of thinking i came to this solution which is the most optimized
// without any unnecessary allocations and utils. Here i decided to use itertools
// to make chunks from iterator instead vector

use itertools::Itertools;
use std::thread::scope;

const THRDS_COUNT: usize = 32;
const THRESHOLD: usize = 64;

pub fn map_reduce<T, R, F>(data: Vec<T>, func: F) -> Vec<R>
where
    T: Send,
    F: Fn(T) -> R + Copy + Send,
    R: Send,
{
    if data.len() > THRESHOLD {
        scope(move |s| {
            let mut handles = Vec::with_capacity(THRDS_COUNT);
            let len = data.len();
            for chunk in data.into_iter().chunks(len / THRDS_COUNT).into_iter() {
                let chunk: Vec<T> = chunk.collect();
                handles.push(s.spawn(move || chunk.into_iter().map(func).collect::<Vec<R>>()));
            }
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .flatten()
                .collect()
        })
    } else {
        data.into_iter().map(func).collect()
    }
}
