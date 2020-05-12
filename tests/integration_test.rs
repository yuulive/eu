use partial_application_rs::part_app;

// #[part_app(poly,Clone)]
// fn foo(bar: u32, baz: u64) -> i16 {
//     (bar + baz as u32) as i16
// }

#[test]
fn one_and_two() {
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

// Example code
#[allow(non_camel_case_types, non_snake_case)]
struct foo___Added;
#[allow(non_camel_case_types, non_snake_case)]
struct foo___Empty;
#[allow(non_camel_case_types, non_snake_case)]
struct __PartialApplication__foo_<bar, baz, BODYFN>
where
    BODYFN: Fn(u32, u64) -> i16,
{
    bar___m: ::std::marker::PhantomData<bar>,
    baz___m: ::std::marker::PhantomData<baz>,
    bar: Option<::std::sync::Arc<dyn Fn() -> u32>>,
    baz: Option<::std::sync::Arc<dyn Fn() -> u64>>,
    body: ::std::sync::Arc<BODYFN>,
}

impl<bar, baz, BODYFN> ::std::clone::Clone for __PartialApplication__foo_<bar, baz, BODYFN>
where
    BODYFN: Fn(u32, u64) -> i16,
{
    fn clone(&self) -> Self {
        Self {
            bar___m: ::std::marker::PhantomData,
            baz___m: ::std::marker::PhantomData,
            bar: self.bar.clone(),
            baz: self.baz.clone(),
            body: self.body.clone(),
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
fn foo() -> __PartialApplication__foo_<foo___Empty, foo___Empty, impl Fn(u32, u64) -> i16> {
    __PartialApplication__foo_ {
        bar: None,
        baz: None,
        bar___m: ::std::marker::PhantomData,
        baz___m: ::std::marker::PhantomData,
        body: ::std::sync::Arc::new(|bar, baz| (bar + baz as u32) as i16),
    }
}
#[allow(non_camel_case_types, non_snake_case)]
impl<BODYFN: Fn(u32, u64) -> i16, baz> __PartialApplication__foo_<foo___Empty, baz, BODYFN> {
    fn bar(
        mut self,
        bar: Box<dyn Fn() -> u32>,
    ) -> __PartialApplication__foo_<foo___Added, baz, BODYFN> {
        self.bar = Some(::std::sync::Arc::from(bar));
        unsafe {
            ::std::mem::transmute::<
                __PartialApplication__foo_<foo___Empty, baz, BODYFN>,
                __PartialApplication__foo_<foo___Added, baz, BODYFN>,
            >(self)
        }
    }
}
#[allow(non_camel_case_types, non_snake_case)]
impl<BODYFN: Fn(u32, u64) -> i16, bar> __PartialApplication__foo_<bar, foo___Empty, BODYFN> {
    fn baz(
        mut self,
        baz: Box<dyn Fn() -> u64>,
    ) -> __PartialApplication__foo_<bar, foo___Added, BODYFN> {
        self.baz = Some(::std::sync::Arc::from(baz));
        unsafe {
            ::std::mem::transmute::<
                __PartialApplication__foo_<bar, foo___Empty, BODYFN>,
                __PartialApplication__foo_<bar, foo___Added, BODYFN>,
            >(self)
        }
    }
}
#[allow(non_camel_case_types, non_snake_case)]
impl<BODYFN: Fn(u32, u64) -> i16> __PartialApplication__foo_<foo___Added, foo___Added, BODYFN> {
    fn call(self) -> i16 {
        (self.body)(self.bar.unwrap()(), self.baz.unwrap()())
    }
}
// Bad code
