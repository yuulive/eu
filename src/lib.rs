#[cfg(test)]
mod tests {
    use crate::add1;
    #[test]
    fn it_works() {
        let a = add1();
        let with_three = a.x(|| 3);
        let and_two = with_three.y(|| 2);
        assert_eq!(and_two.call(), 5);
    }
}

use std::marker::PhantomData;
use std::ops::FnOnce;

struct Added;
struct Empty;

struct PartialAdd1<X, XFN, Y, YFN, BODYFN>
where
    XFN: FnOnce() -> u32,
    YFN: FnOnce() -> u32,
    BODYFN: FnOnce(u32, u32) -> u32,
{
    x_m: PhantomData<X>,
    x: Option<XFN>,
    y_m: PhantomData<Y>,
    y: Option<YFN>,
    body: BODYFN,
}

fn add1<X, Y>() -> PartialAdd1<Empty, X, Empty, Y, impl FnOnce(u32, u32) -> u32>
where
    X: FnOnce() -> u32,
    Y: FnOnce() -> u32,
{
    PartialAdd1 {
        x: None,
        x_m: PhantomData,
        y: None,
        y_m: PhantomData,
        body: |x, y| x + y,
    }
}

impl<XFN: FnOnce() -> u32, YFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> u32, Y>
    PartialAdd1<Empty, XFN, Y, YFN, BODYFN>
{
    fn x(mut self, x: XFN) -> PartialAdd1<Added, XFN, Y, YFN, BODYFN> {
        self.x = Some(x);
        unsafe {
            // maybe should cast with a raw pointer conversion instead
            // this might not be optimized out
            std::mem::transmute_copy::<
                PartialAdd1<Empty, XFN, Y, YFN, BODYFN>,
                PartialAdd1<Added, XFN, Y, YFN, BODYFN>,
            >(&self)
        }
    }
}

impl<XFN: FnOnce() -> u32, YFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> u32, X>
    PartialAdd1<X, XFN, Empty, YFN, BODYFN>
{
    fn y(mut self, y: YFN) -> PartialAdd1<X, XFN, Added, YFN, BODYFN> {
        self.y = Some(y);
        unsafe {
            // maybe should cast with a raw pointer conversion instead
            // this might not be optimized out
            std::mem::transmute_copy::<
                PartialAdd1<X, XFN, Empty, YFN, BODYFN>,
                PartialAdd1<X, XFN, Added, YFN, BODYFN>,
            >(&self)
        }
    }
}

impl<XFN: FnOnce() -> u32, YFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> u32>
    PartialAdd1<Added, XFN, Added, YFN, BODYFN>
{
    fn call(self) -> u32 {
        (self.body)(self.x.unwrap()(), self.y.unwrap()())
    }
}
// struct PartialApplyAdd<X, Y, Body, X_func, Y_func>
// where
//     Body: Fn(u32, u32) -> i64,
//     X_func: Fn() -> u32,
//     Y_func: Fn() -> u32,
// {
//     x: Option<X_func>,
//     y: Option<Y_func>,
//     body: Body,
//     x_m: PhantomData<X>,
//     y_m: PhantomData<Y>,
// }

// fn add() -> PartialApplyAdd<(), (), dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//     PartialApplyAdd {
//         x: None,
//         y: None,
//         body: |x, y| x + y,
//         x_m: PhantomData,
//         y_m: PhantomData,
//     }
// }

// impl<X> PartialApplyAdd<X, (), dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//     fn y(
//         mut self,
//         y: dyn FnOnce() -> u32,
//     ) -> PartialApplyAdd<X, bool, dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//         self.y = Some(y);
//         unsafe { std::mem::transmute::<PartialApplyAdd<X, ()>, PartialApplyAdd<X, bool>>(self) }
//     }
// }

// impl<Y> PartialApplyAdd<(), Y, dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//     fn x(
//         self,
//         x: dyn FnOnce() -> u32,
//     ) -> PartialApplyAdd<bool, (), dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//         self.x = Some(x);
//         unsafe { std::mem::transmute::<PartialApplyAdd<(), Y>, PartialApplyAdd<bool, Y>>(self) }
//     }
// }

// impl PartialApplyAdd<bool, bool, dyn Fn(u32, u32) -> i64, dyn Fn() -> u32, dyn Fn() -> u32> {
//     fn call(self) -> i64 {
//         self.body(self.x.unwrap()(), self.y.unwrap()())
//     }
// }

// // This should provide compile time checking for partial application
// // Then something like}
