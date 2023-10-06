// This attempt is too bad, because we have plenty of allocations on every sneeze
// Anyway, this works but slower than one thread due to infinite allocations)

use std::mem::MaybeUninit;
use std::sync::mpsc::channel;
use std::thread::scope;

const THRESHOLD: usize = 64;
const THRDS_COUNT: usize = 32;

pub fn map_reduce<'a, T, R, F>(data: Vec<T>, func: F) -> Vec<R>
where
    T: Clone + Send,
    R: Send,
    F: FnOnce(T) -> R + Send + Copy,
{
    if data.len() > THRESHOLD {
        let mut res = Vec::new();
        let data_length = data.len();
        res.resize_with(data_length, MaybeUninit::<R>::uninit);

        let (tx, rx) = channel::<(usize, R)>();
        let chunks: Vec<Vec<(usize, T)>> = range_compute_chunks(data);
        scope(|s| {
            for chunk in chunks {
                let sender = tx.clone();
                s.spawn(move || {
                    for (i, item) in chunk {
                        sender.send((i, func(item))).unwrap();
                    }
                });
            }
        });

        for _ in 0..data_length {
            if let Ok((i, r)) = rx.recv() {
                res[i] = MaybeUninit::new(r);
            }
        }

        // Here can be some problems if Rust is configured not to propagate panic
        // through threads to main and some items can be uninited. Of course i can use
        // just R: Default to make res.resize_with(data_length, R::default) but it is not
        // too good for use experience. Also i can just push values from sender to some container
        // and then sort it by usize index but there is no benefit from this operation
        // because we have O(n) in memory space and the difficulty of sorting algorithm
        unsafe { res.into_iter().map(|item| item.assume_init()).collect() }
    } else {
        data.into_iter().map(|item| func(item)).collect()
    }
}

fn range_compute_chunks<T: Clone>(data: Vec<T>) -> Vec<Vec<(usize, T)>> {
    let tasks_per_thrd = data.len() / THRDS_COUNT;

    data.into_iter()
        .enumerate()
        .collect::<Vec<(usize, T)>>()
        .chunks(tasks_per_thrd)
        .map(|rng| rng.to_vec())
        .collect()
}
