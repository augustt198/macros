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

#[test]
fn test_repeat() {
    let mut count = 0u;

    repeat! { (count += 1) 100 times }

    assert!(count == 100)
}

#[test]
fn twice_test() {
    let mut count = 0u;

    twice! { count += 1 }

    assert!(count == 2)
}

#[test]
fn hash_test() {
    let mut hash1 = std::collections::HashMap::new();
    hash1.insert("hello", "world");

    let hash2 = hash! {"hello" => "world"};

    assert!(hash1 == hash2);
}
