use crate::serde::packet_struct;

packet_struct! {
    /// Report the invasion progress.
    ///
    /// Direction: Server -> Client.
    pub struct InvasionProgress {
        const TAG = 78;

        pub progress: i32,
        pub max_progress: i32,
        pub icon: i8,
        pub wave: i8,
    }
}
