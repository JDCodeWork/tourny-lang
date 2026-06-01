#[macro_export]
macro_rules! bytecode {
    ($($x:expr),* $(,)?) => {
        vec![$($x as u8),*]
    };
}
