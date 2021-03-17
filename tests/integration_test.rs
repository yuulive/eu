use eu::part_app;

#[part_app(poly, Clone)]
pub fn foo(bar: u32, baz: u64) -> i16 {
    (bar + baz as u32) as i16
}

#[test]
fn poly_clone() {
    let adder = foo();
    let with_two = adder.bar(Box::new(|| 1));

    let final1 = with_two.clone().baz(Box::new(|| 2));
    assert_eq!(final1.call(), 3);

    let final2 = with_two.baz(Box::new(|| 3));
    assert_eq!(final2.call(), 4);
}

#[part_app(poly)]
fn adder(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}

#[test]
fn poly() {
    let a = adder().c(Box::new(|| 5));
    let b = a.a(Box::new(|| 1 + 2));
    if true {
        let c = b.b(Box::new(|| 6));
        assert_eq!(c.call(), 14);
    } else {
        assert_eq!(b.b(Box::new(|| 0)).call(), 8);
    }
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

#[part_app(value)]
fn value_add(x: u32, y: u32) -> i64 {
    (x + y) as i64
}

#[test]
fn by_value() {
    let adder = value_add().x(3);
    assert_eq!(adder.y(3).call(), 6);
}

#[part_app(value, Clone)]
fn clone_value_add(x: u32, y: u32) -> u32 {
    x + y
}

#[test]
fn clone_value_test() {
    let adder = clone_value_add();
    let with_one = adder.x(1);
    assert_eq!(with_one.clone().y(2).call(), 3);
    assert_eq!(with_one.y(3).call(), 4);
}
