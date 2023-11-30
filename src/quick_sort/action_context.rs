use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CreateItem {
    pub value: u32,
    pub value_uuid: Uuid,
    pub array_uuid: Uuid,
}

#[derive(Debug, Clone, Copy)]
pub struct Pivot {
    pub value_uuid: Uuid,
    pub array_uuid: Uuid,
}

#[derive(Debug, Clone, Copy)]
pub struct Recursion {
    auuid: Uuid, // ID for Array
}

#[derive(Debug, Clone, Copy)]
pub struct RecursiveItem {
    vuuid: Uuid,
}

impl CreateItem {
    pub fn new(val: u32, vuuid: Uuid, auuid: Uuid) -> Self {
        Self {
            value: val,
            value_uuid: vuuid,
            array_uuid: auuid,
        }
    }
}

impl Pivot {
    pub fn new(vuuid: Uuid, auuid: Uuid) -> Self {
        Self {
            value_uuid: vuuid,
            array_uuid: auuid,
        }
    }
}

impl Recursion {
    pub fn new(auuid: Uuid) -> Self {
        Self { auuid }
    }
}

impl RecursiveItem {
    pub fn new(vuuid: Uuid) -> Self {
        Self { vuuid }
    }
}
