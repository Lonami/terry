use crate::serde::serializable_struct;

serializable_struct! {
    pub struct Chest {
        pub index: u16,
        pub x: i16,
        pub y: i16,
        pub name: String,
    }
}
