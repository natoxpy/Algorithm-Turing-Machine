pub mod action_context;
pub mod action_history;

use action_context::{CreateItem, Pivot};
use action_history::{Action, ActionHistory};
use uuid::Uuid;

use crate::quick_sort::action_history::Reaction;

pub fn sort(arr: Vec<u32>) {
    let mut history = ActionHistory::new();

    history.push(Action::EnterStage);

    let array_uuid = Uuid::new_v4();
    history.push(Action::CreateArray(array_uuid));

    for i in arr {
        history.push(Action::CreateItem(CreateItem::new(
            i,
            Uuid::new_v4(),
            array_uuid,
        )));
    }

    _sort(array_uuid, &mut history);
}

fn _sort(arr: Uuid, history: &mut ActionHistory) {
    let arr_reaction = history.push(Action::ReadItems(arr));
    let mut items = vec![];

    if let Reaction::Items(_items) = arr_reaction {
        for item in _items {
            if let Action::CreateItem(a) = item {
                items.push((a.value, a.value_uuid));
            }
        }
    } else {
        panic!("Wrong reaction in _sort!")
    }

    if items.len() <= 1 {
        return;
    }

    let pivot = items.pop().unwrap();
    history.push(Action::ExtractPivot(Pivot::new(pivot.1, arr.clone())));

    // let left = Uuid::new_v4();

    // history.push(Action::CreateArray(left));
    // // history.push(Action::StartRecursionLeft(Recursion::new(left)));
    // history.push(Action::OpenScope(left));

    // let right = Uuid::new_v4();
    // history.push(Action::CreateArray(right));
    // history.push(Action::OpenScope(right));

    let mut left_items = vec![];
    let mut right_items = vec![];

    for item in items {
        if item.0 > pivot.0 {
            left_items.push(item.1);
            // history.push(Action::RightItem(RecursiveItem::new(item.1)));
        } else {
            right_items.push(item.1);
            // history.push(Action::ItemLeft(RecursiveItem::new(item.1)));
        }
    }

    // history.push(Action::ConcatPivot(Pivot::new(pivot.1, arr.clone())));

    // history.push(Action::CloseScope(left));
    // history.push(Action::CloseScope(left));

    // history.push(Action::ResolveRecursionLeft(Recursion::new(left)));
    // history.push(Action::ResolveRecursionRight(Recursion::new(right)));

    // history.push(Action::CleanArray(left));
    // history.push(Action::CleanArray(right));

    println!("{:#?}", history);
}

// mod sorttype;
// use self::sorttype::{ArrayContext, CreateArrayContext};
// use sorttype::SortType;
// use std::sync::{Arc, Mutex};

// pub fn sort(arr: Vec<u32>) {
//     let mut sorttype = SortType::new();

//     let main_array = sorttype.create_array(CreateArrayContext::Main);
//     main_array.lock().unwrap().set_silent_data(arr);

//     _sort(main_array, &mut sorttype);
// }

// fn _sort(arr: Arc<Mutex<ArrayContext>>, stype: &mut SortType) {
//     let mut ar = arr.lock().unwrap();

//     let pivot = ar.pivot_pop();
//     let mut left = stype.create_array(CreateArrayContext::Left);
//     let mut right = stype.create_array(CreateArrayContext::Right);
//     let mut mleft = left.lock().unwrap();
//     let mut mright = right.lock().unwrap();

//     for uuid in ar.get_iter() {
//         if pivot.value > ar.get_value(uuid).value {
//             mleft.append(ar.get_value(uuid).value, sorttype::ItemContext::Left);
//         } else {

//         }
//     }
// }
