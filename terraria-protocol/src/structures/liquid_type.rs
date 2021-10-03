use crate::serde::serializable_enum;

serializable_enum! {
    pub enum LiquidType: u8 {
        None = 0,
        Water = 1,
        Lava = 2,
        Honey = 3,
    }
}
