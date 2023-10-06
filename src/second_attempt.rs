// This attempt is bad too, because there is also a big allocation in the end of function
// which takes a lot of time due to impossibility of moving value from raw pointer.
// But this works too)

use std::cell::UnsafeCell;
use std::sync::Arc;
struct SyncWrapper<R>(UnsafeCell<Vec<R>>);
unsafe impl<R> Sync for SyncWrapper<R> {}

use crossbeam::scope;

const THRESHOLD: usize = 64;
const THRDS_COUNT: usize = 32;

pub fn map_reduce<T, R, F>(data: Vec<T>, func: F) -> Vec<R>
where
    T: Clone + Send + Sync,
    R: Send + Default + Clone,
    F: FnOnce(T) -> R + Send + Copy,
{
    if data.len() > THRESHOLD {
        let chunk_size = (data.len() + THRDS_COUNT - 1) / THRDS_COUNT;

        let res = scope(|s| {
            let res = Arc::new(SyncWrapper(UnsafeCell::new(vec![R::default(); data.len()])));

            for i in 0..THRDS_COUNT {
                let start = i * chunk_size;
                let end = std::cmp::min(start + chunk_size, data.len());
                let chunk = &data[start..end];
                let res = Arc::clone(&res);

                s.spawn(move |_| {
                    for (j, item) in chunk.iter().enumerate() {
                        unsafe { (*res.0.get())[start + j] = func(item.clone()) };
                    }
                });
            }
            res
        });
        unsafe { (*Arc::into_inner(res.unwrap()).unwrap().0.get()).clone() }
    } else {
        data.into_iter().map(|item| func(item)).collect()
    }
}

// use std::cell::UnsafeCell;
// use std::mem::MaybeUninit;
// use std::sync::Arc;

// struct SyncWrapper<R>(UnsafeCell<Vec<R>>);
// unsafe impl<R> Sync for SyncWrapper<R> {}

// use crossbeam::scope;

// const THRESHOLD: usize = 32;
// const THRDS_COUNT: usize = 10;

// pub fn map_reduce<T, R, F>(data: Vec<T>, func: F) -> Vec<R>
// where
//     T: Clone + Send + Sync,
//     R: Send,
//     F: FnOnce(T) -> R + Send + Copy,
// {
//     if data.len() > THRESHOLD {
//         let res = scope(|s| {
//             let chunk_size = (data.len() + THRDS_COUNT - 1) / THRDS_COUNT;
//             let mut inner = Vec::with_capacity(data.len());
//             inner.resize_with(data.len(), MaybeUninit::<R>::uninit);

//             let shared = Arc::new(SyncWrapper(UnsafeCell::new(inner)));

//             for i in 0..THRDS_COUNT {
//                 let start = i * chunk_size;
//                 let end = std::cmp::min(start + chunk_size, data.len());
//                 let chunk = &data[start..end];
//                 let shared = Arc::clone(&shared);

//                 s.spawn(move |_| {
//                     for (j, item) in chunk.iter().enumerate() {
//                         unsafe {
//                             (*shared.0.get())[start + j] = MaybeUninit::new(func(item.clone()))
//                         };
//                     }
//                 });
//             }

//             shared
//         });

//         unsafe {
//             let res = std::ptr::read::<Vec<MaybeUninit<R>>>(
//                 Arc::into_inner(res.unwrap()).unwrap().0.get_mut(),
//             );
//             res.into_iter().map(|item| item.assume_init()).collect()
//         }
//     } else {
//         data.into_iter().map(|item| func(item)).collect()
//     }
// }
