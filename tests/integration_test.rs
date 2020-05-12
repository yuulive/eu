use partial_application_rs::part_app;

#[part_app]
fn foo(bar: u32, baz: u64) -> i16 {
    (bar + baz as u32) as i16
}

#[part_app]
fn concat_string<'a, 'b>(s1: &'a str, s2: &'a str) -> String {
    s1.to_string() + &s2
}

#[test]
fn one_and_two() {
    let adder = foo();
    let with_two = adder.bar(|| 1);
    let final_ = with_two.baz(|| 2);
    assert!(final_.call() == 3);
}

#[test]
fn hello_world() {
    let world = concat_string().s2(|| "World!");
    let hello = world.s1(|| "Hello, ");
    assert_eq!(hello.call(), "Hello, World!");
}

// #[allow(non_camel_case_types, non_snake_case)]
// struct concat_string___Added;
// #[allow(non_camel_case_types, non_snake_case)]
// struct concat_string___Empty;

// #[allow(non_camel_case_types, non_snake_case)]
// struct __PartialApplication__concat_string_<'a, s1, s1___FN, s2, s2___FN, BODYFN>
// where
//     s1___FN: FnOnce() -> &'a str,
//     s2___FN: FnOnce() -> &'a str,
//     BODYFN: FnOnce(&str, &str) -> String,
// {
//     s1___m: ::std::marker::PhantomData<s1>,
//     s2___m: ::std::marker::PhantomData<s2>,
//     s1: Option<s1___FN>,
//     s2: Option<s2___FN>,
//     body: BODYFN,
// }
// #[allow(non_camel_case_types, non_snake_case)]
// fn concat_string<'a, s1, s2>() -> __PartialApplication__concat_string_<
//     'a,
//     concat_string___Empty,
//     s1,
//     concat_string___Empty,
//     s2,
//     impl FnOnce(&str, &str) -> String,
// >
// where
//     s1: FnOnce() -> &'a str,
//     s2: FnOnce() -> &'a str,
// {
//     __PartialApplication__concat_string_ {
//         s1: None,
//         s2: None,
//         s1___m: ::std::marker::PhantomData,
//         s2___m: ::std::marker::PhantomData,
//         body: |s1, s2| s1.to_string() + &s2,
//     }
// }
// #[allow(non_camel_case_types, non_snake_case)]
// impl<
//         'a,
//         s1___FN: FnOnce() -> &'a str,
//         s2___FN: FnOnce() -> &'a str,
//         BODYFN: FnOnce(&str, &str) -> String,
//         s2,
//     >
//     __PartialApplication__concat_string_<'a, concat_string___Empty, s1___FN, s2, s2___FN, BODYFN>
// {
//     fn s1(
//         mut self,
//         s1: s1___FN,
//     ) -> __PartialApplication__concat_string_<'a, concat_string___Added, s1___FN, s2, s2___FN, BODYFN>
//     {
//         self.s1 = Some(s1);
//         unsafe {
//             ::std::mem::transmute_copy::<
//                 __PartialApplication__concat_string_<
//                     'a,
//                     concat_string___Empty,
//                     s1___FN,
//                     s2,
//                     s2___FN,
//                     BODYFN,
//                 >,
//                 __PartialApplication__concat_string_<
//                     'a,
//                     concat_string___Added,
//                     s1___FN,
//                     s2,
//                     s2___FN,
//                     BODYFN,
//                 >,
//             >(&self)
//         }
//     }
// }
// #[allow(non_camel_case_types, non_snake_case)]
// impl<
//         'a,
//         s1___FN: FnOnce() -> &'a str,
//         s2___FN: FnOnce() -> &'a str,
//         BODYFN: FnOnce(&str, &str) -> String,
//         s1,
//     >
//     __PartialApplication__concat_string_<'a, s1, s1___FN, concat_string___Empty, s2___FN, BODYFN>
// {
//     fn s2(
//         mut self,
//         s2: s2___FN,
//     ) -> __PartialApplication__concat_string_<'a, s1, s1___FN, concat_string___Added, s2___FN, BODYFN>
//     {
//         self.s2 = Some(s2);
//         unsafe {
//             ::std::mem::transmute_copy::<
//                 __PartialApplication__concat_string_<
//                     'a,
//                     s1,
//                     s1___FN,
//                     concat_string___Empty,
//                     s2___FN,
//                     BODYFN,
//                 >,
//                 __PartialApplication__concat_string_<
//                     'a,
//                     s1,
//                     s1___FN,
//                     concat_string___Added,
//                     s2___FN,
//                     BODYFN,
//                 >,
//             >(&self)
//         }
//     }
// }
// #[allow(non_camel_case_types, non_snake_case)]
// impl<
//         'a,
//         s1___FN: FnOnce() -> &'a str,
//         s2___FN: FnOnce() -> &'a str,
//         BODYFN: FnOnce(&str, &str) -> String,
//     >
//     __PartialApplication__concat_string_<
//         'a,
//         concat_string___Added,
//         s1___FN,
//         concat_string___Added,
//         s2___FN,
//         BODYFN,
//     >
// {
//     fn call(self) -> String {
//         (self.body)(self.s1.unwrap()(), self.s2.unwrap()())
//     }
// }
