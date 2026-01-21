


#[macro_export]
macro_rules! map {
    { $($key: expr => $val: expr),* $(,)? } => {
        { &[$(($key.into(), $val.into())),*].iter().collect() }
    };
}


