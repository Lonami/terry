#[macro_export]
macro_rules! bitflags {
    (
        $struct_name:ident[$size:expr] {
            $( $bit_index:expr => $getter:ident & $setter:ident ),*
        }
    ) => {
        // TODO better debug showing flag names
        #[derive(Debug)]
        pub struct $struct_name {
            value: [u8; $size]
        }

        impl $struct_name {
            /// The actual value as a sequence of bytes of these flags.
            pub fn value(&self) -> [u8; $size] {
                self.value
            }

            $(
                pub fn $getter(&mut self) -> bool {
                    self.value[$bit_index / 8] & (1 << ($bit_index % 8)) != 0
                }

                pub fn $setter(&mut self, value: bool) {
                    if value {
                        self.value[$bit_index / 8] |= (1 << ($bit_index % 8));
                    } else {
                        self.value[$bit_index / 8] &= 0xff - (1 << ($bit_index % 8));
                    }
                }
            )*
        }
    }
}
