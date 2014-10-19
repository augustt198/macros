#![feature(macro_rules)]
macro_rules! maybe(
    () => (
        std::rand::random::<bool>();
    );

    ($body:expr) => (
        if std::rand::random() { $body }
    );

    ($body:block) => (
        if std::rand::random() $body
    );    
)

macro_rules! repeat(
    ($($body:expr $times:expr times),*) => ({
        $(
            let mut _x = 0u;
            while _x < $times {
                $body ;
                _x += 1;
            }
        )*
    });
)

macro_rules! twice(
    ($($body:expr),*) => ({
        $($body; $body;)*
    });   
)

macro_rules! hash(
    ($($key:expr => $val:expr),*) => ({
        let mut _temp = std::collections::HashMap::new();
        $(_temp.insert($key, $val);)*
        _temp
    });
)
