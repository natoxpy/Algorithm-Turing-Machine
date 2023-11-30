

// use std::{
//     collections::HashMap,
//     sync::{Arc, Mutex},
// };
// use uuid::Uuid;

// #[derive(Debug, Clone)]
// pub enum DataActionContext {
//     PivotPop,
// }

// #[derive(Debug, Clone)]
// pub struct ArrayContext {
//     data: HashMap<Uuid, Item>,
//     runtime_data: HashMap<Uuid, Item>,
//     rtd_content: Vec<Uuid>,
//     runtime_data_context: Vec<(Uuid, DataActionContext)>,
// }

// impl From<Vec<u32>> for ArrayContext {
//     fn from(arr: Vec<u32>) -> Self {
//         // Total data every allocated for this array
//         let mut hmap: HashMap<Uuid, Item> = HashMap::new();

//         // Runtime Data Content
//         let mut rtdc = vec![];

//         for ar in arr {
//             let uuid = Uuid::new_v4();
//             rtdc.push(uuid);
//             hmap.insert(uuid, Item::new(ar, ItemContext::Pivot(ItemPopContext {})));
//         }

//         Self {
//             data: hmap.clone(),
//             runtime_data: hmap.clone(),
//             rtd_content: rtdc.clone(),
//             runtime_data_context: Vec::new(),
//         }
//     }
// }

// impl ArrayContext {
//     pub fn set_silent_data(&mut self, arr: Vec<u32>) {
//         for ar in arr {
//             let uuid = Uuid::new_v4();
//             let item = Item::new(ar, ItemContext::Silent);

//             self.runtime_data.insert(uuid, item.clone());
//             self.rtd_content.push(uuid);
//             self.data.insert(uuid, item.clone());
//         }
//     }

//     pub fn add_context(&mut self, cfor: Uuid, c: DataActionContext) {
//         self.runtime_data_context.push((cfor, c));
//     }

//     pub fn pivot_pop(&mut self) -> Item {
//         let popped_uuid = self.rtd_content.pop().unwrap();
//         self.add_context(popped_uuid.clone(), DataActionContext::PivotPop);
//         return *self.runtime_data.get(&popped_uuid).unwrap();
//     }

//     pub fn get_iter(&self) -> std::slice::Iter<Uuid> {
//         self.rtd_content.iter()
//     }

//     pub fn get_value(&self, uuid: &Uuid) -> &Item {
//         self.runtime_data.get(uuid).unwrap()
//     }

//     pub fn append(&mut self, value: u32, context: ItemContext) {
//         let uuid = Uuid::new_v4();
//         self.data.insert(uuid, Item::new(value, context));
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub struct ItemPopContext {}

// #[derive(Debug, Clone, Copy)]
// pub struct ItemLeftContext {}
// #[derive(Debug, Clone, Copy)]
// pub struct ItemRightContext {}

// #[derive(Debug, Clone, Copy)]
// pub enum ItemContext {
//     Pivot(ItemPopContext),
//     Silent,
//     Left(ItemLeftContext),
//     RightContext(ItemRightContext),
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Item {
//     pub context: ItemContext,
//     pub value: u32,
// }

// impl Item {
//     pub fn new(value: u32, ctx: ItemContext) -> Self {
//         Self {
//             context: ctx,
//             value,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum CreateArrayContext {
//     Main,
//     Left,
//     Right,
// }

// pub struct SortType {
//     pub arrays: Arc<Mutex<HashMap<Uuid, Arc<Mutex<ArrayContext>>>>>,
// }

// impl SortType {
//     pub fn new() -> Self {
//         Self {
//             arrays: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     pub fn create_array(&mut self, context: CreateArrayContext) -> Arc<Mutex<ArrayContext>> {
//         let mut arrays = self.arrays.lock().unwrap();

//         let r = Arc::new(Mutex::new(ArrayContext::from(vec![])));
//         arrays.insert(Uuid::new_v4(), r.clone());

//         return r.clone();
//     }
// }
