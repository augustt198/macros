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
        $(for x in range(0u, $times) { $body })*
    });

    ($body:block $times:expr times) => (
        for x in range(0u, $times) $body
    )
)

macro_rules! twice(
    ($($body:expr),*) => ({
        $($body; $body;)*
    });

    ($($body:block),*) => ({
        $($body $body)*
    });
)

macro_rules! hash(
    ($($key:expr => $val:expr),*) => ({
        let mut _temp = std::collections::HashMap::new();
        $(_temp.insert($key, $val);)*
        _temp
    });
)
