use partial_application_rs::part_app;

#[part_app]
fn foo(bar: u32, baz: u64) -> i16 {
    (bar + baz as u32) as i16
}

#[test]
fn one_and_two() {
    let adder = foo();
    let with_two = adder.bar(|| 1);
    let final_ = with_two.baz(|| 2);
    assert!(final_.call() == 3);
}

#[part_app]
fn concat_string<'a>(s1: &'a str, s2: &'a str) -> String {
    s1.to_string() + &s2
}

#[test]
fn hello_world() {
    let world = concat_string().s2(|| "World!");
    let hello = world.s1(|| "Hello, ");
    assert_eq!(hello.call(), "Hello, World!");
}

#[part_app]
fn add(x: u32, y: u32) -> i64 {
    (x + y) as i64
}

#[test]
fn simple_fn() {
    let a = add();
    let two = a.x(|| 2);
    let number = two.y(|| 40);
    assert_eq!(number.call(), 42);
}
