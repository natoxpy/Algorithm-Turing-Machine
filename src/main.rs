// use std::{
//     sync::{Arc, Mutex},
//     time::SystemTime,
// };
// #[derive(Debug)]
// enum StateAction {
//     Read,
//     Write,
// }
// #[derive(Debug)]
// struct ActionRecord {
//     action: StateAction,
//     time: SystemTime,
// }
// fn access_record(track: Arc<Mutex<Vec<ActionRecord>>>, action: StateAction) {
//     let mut mutex = track.lock().unwrap();
//     mutex.push(ActionRecord {
//         action: action,
//         time: SystemTime::now(),
//     });
// }
// fn quick_sort(l: Vec<i32>, track: Arc<Mutex<Vec<ActionRecord>>>) -> Vec<i32> {
//     if l.len() <= 1 {
//         return l;
//     };
//     access_record(track.clone(), StateAction::Read);
//     let mut ml = l.clone();
//     access_record(track.clone(), StateAction::Read);
//     let pivot = ml.pop().unwrap();
//     let mut left = Vec::new();
//     let mut right = Vec::new();
//     for n in ml {
//         access_record(track.clone(), StateAction::Read);
//         if n > pivot {
//             access_record(track.clone(), StateAction::Write);
//             right.push(n);
//         } else {
//             access_record(track.clone(), StateAction::Write);
//             left.push(n)
//         }
//     }
//     access_record(track.clone(), StateAction::Write);
//     return vec![
//         quick_sort(left, track.clone()),
//         vec![pivot],
//         quick_sort(right, track.clone()),
//     ]
//     .concat();
// }
// fn get_reads(track: Arc<Mutex<Vec<ActionRecord>>>) -> u32 {
//     let ltrack = track.lock().unwrap();
//     let mut reads = 0;
//     for t in ltrack.iter() {
//         if let StateAction::Read = t.action {
//             reads += 1;
//         }
//     }
//     reads
// }
// fn get_writes(track: Arc<Mutex<Vec<ActionRecord>>>) -> u32 {
//     let ltrack = track.lock().unwrap();
//     let mut writes = 0;
//     for t in ltrack.iter() {
//         if let StateAction::Write = t.action {
//             writes += 1;
//         }
//     }
//     writes
// }

use algo_bench::quick_sort::sort;

fn main() {
    let qs = sort(vec![1, 10, 3, 5, 4, 7, 8, 9, 6]);

    println!("{:#?}", qs);

    // let list = vec![5, 3, 2, 4];
    // let track: Arc<Mutex<Vec<ActionRecord>>> = Arc::new(Mutex::new(vec![]));
    // println!("{:#?}", list);

    // let sorted = quick_sort(list, track.clone());

    // let reads = get_reads(track.clone());
    // let writes = get_writes(track.clone());

    // println!("Reads: {:?}", reads);
    // println!("Writes: {:?}", writes);

    // println!("{:#?}", sorted);
}
