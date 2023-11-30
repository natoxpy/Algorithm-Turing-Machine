use super::action_context::{*};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    // Enters
    EnterStage,
    CreateArray(Uuid),
    // Item UUID
    ReadItem(Uuid),
    // Array Uuid
    ReadItems(Uuid),
    CreateItem(CreateItem),
    CleanArray(Uuid),
    ExtractPivot(Pivot),
    ConcatPivot(Pivot),

    StartRecursionLeft(Recursion),
    StartRecursionRight(Recursion),

    Move(Uuid),
    OpenScope(Uuid), 
    CloseScope(Uuid),

    ItemLeft(RecursiveItem),
    RightItem(RecursiveItem),
    ReturnItem(RecursiveItem),

    ResolveRecursionLeft(Recursion),
    ResolveRecursionRight(Recursion),
}

#[derive(Debug, Clone)]
pub enum Reaction<'a> {
    Item(&'a Action),
    Items(Vec<&'a Action>),
    Silent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mapped {
    CreateArray,
    CleanedArray,
    ItemRefs,
    MovedItem,
}

#[derive(Debug, Clone)]
pub struct ActionHistory {
    actions: Vec<Action>,
    data: HashMap<Uuid, Vec<Action>>,
    mapped: HashMap<Mapped, HashMap<Uuid, (Action, usize)>>,
}

impl ActionHistory {
    pub fn new() -> Self {
        let mut mapped = HashMap::new();
        mapped.insert(Mapped::CreateArray, HashMap::new());
        mapped.insert(Mapped::CleanedArray, HashMap::new());
        mapped.insert(Mapped::ItemRefs, HashMap::new());
        mapped.insert(Mapped::MovedItem, HashMap::new());

        Self {
            actions: Vec::new(),
            data: HashMap::new(),
            mapped,
        }
    }

    pub fn push(&mut self, action: Action) -> Reaction {
        match action {
            Action::ReadItem(uuid) => {
                let (action, _) = self
                    .mapped
                    .get(&Mapped::ItemRefs)
                    .unwrap()
                    .get(&uuid)
                    .unwrap();

                return Reaction::Item(action);
            }
            Action::ReadItems(auuid) => {
                let (_, index) = self
                    .mapped
                    .get(&Mapped::CreateArray)
                    .unwrap()
                    .get(&auuid)
                    .unwrap();

                let start = index.clone();
                let mut actions_found = vec![];

                for (idx, action) in self.actions.iter().enumerate() {
                    if idx < start {
                        continue;
                    }

                    if let Action::CreateItem(c) = action {
                        if c.array_uuid == auuid {
                            actions_found.push(action);
                        }
                    }
                }

                return Reaction::Items(actions_found);
            }
            Action::CreateArray(uuid) => {
                self.mapped
                    .get_mut(&Mapped::CreateArray)
                    .unwrap()
                    .insert(uuid.clone(), (action.clone(), self.actions.len()));

                self.data.insert(uuid, Vec::new());
            }
            Action::CreateItem(c) => {
                self.data
                    .get_mut(&c.array_uuid)
                    .unwrap()
                    .push(action.clone());

                self.mapped
                    .get_mut(&Mapped::ItemRefs)
                    .unwrap()
                    .insert(c.value_uuid.clone(), (action.clone(), self.actions.len()));
            }
            Action::EnterStage => self.actions.push(action),
            Action::CleanArray(uuid) => {
                self.data.remove(&uuid);
            }
            Action::ExtractPivot(d) => {}
            Action::ConcatPivot(d) => {}
            _ => {}
        }

        self.actions.push(action);
        Reaction::Silent
    }

    fn read_items(&self) -> Vec<Uuid> {
        let items = vec![];

        items
    }
}
