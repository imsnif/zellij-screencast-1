fn main() {
    println!("Hello world!");
}

fn why_would_i_want_this() {
    for i in 0..10000 {
        let timestamp = std::time::Instant::now();
        if i % 7 == 0 {
            println!("error! - {:?}", timestamp);
        } else {
            println!("all good! - {:?}", timestamp);
        }
    }
}

#[test]
fn reality_makes_sense() {
    assert_eq!(1, 1, "Oh no!")
}
