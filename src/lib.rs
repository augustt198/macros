#![feature(macro_rules)]

// returns a random boolean when used with empty parameters
// if supplied a block/expression, the block/expression is
// called if a random boolean is true.
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

// repeat a block/expression a certain amount of times
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

// repeat a block/expression twice
macro_rules! twice(
    ($($body:expr),*) => ({
        $($body; $body;)*
    });   
)

// HashMap literal
macro_rules! hash(
    ($($key:expr => $val:expr),*) => ({
        let mut _temp = std::collections::HashMap::new();
        $(_temp.insert($key, $val);)*
        _temp
    });

    ($key_type:ty , $val_type:ty) => (
        std::collections::HashMap::<$key_type, $val_type>::new()
    )
)

macro_rules! t(
    ($condition:expr ? $ifbranch:expr : $elsebranch:expr) => (
        if $condition {
            $ifbranch
        } else {
            $elsebranch
        }
    )
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

#[test]
fn hash_type_test() {
    let hash1 = std::collections::HashMap::<String, String>::new();
    let hash2 = hash! { String, String };
    assert!(hash1 == hash2);
}

#[test]
fn ternary_true_expr_test() {
    let num = t! { true ? 1i : 0i };
    assert!(num == 1);
}

#[test]
fn ternary_false_expr_test() {
    let num = t! { false ? 1i : 0i };
    assert!(num == 0);
}
