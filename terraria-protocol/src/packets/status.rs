use crate::serde::packet_struct;
use crate::structures::NetString;

packet_struct! {
    /// Status.
    ///
    /// Direction: Server -> Client.
    pub struct Status {
        const TAG = 9;

        /// Ever-increasing status identifier
        pub id: i32,
        pub text: NetString,
        pub flags: u8,
    }
}
