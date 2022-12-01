#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {

            let mut hm = ::std::collections::HashMap::new();
            $(
                let _ = hm.insert($key, $value);
            )*
            hm
        }
    };
}
