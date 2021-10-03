use crate::serde::serializable_struct;

serializable_struct! {
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }
}
