use crate::structures::serializable_struct;

serializable_struct! {
    pub struct RGB {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }
}
