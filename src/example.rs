#![allow(dead_code)]
use std::marker::PhantomData;
use std::ops::FnOnce;

pub struct Added;
pub struct Empty;

pub struct PartialAdd1<X, XFN, Y, YFN, BODYFN>
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

pub fn add1<X, Y>() -> PartialAdd1<Empty, X, Empty, Y, impl FnOnce(u32, u32) -> u32>
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
    pub fn x(mut self, x: XFN) -> PartialAdd1<Added, XFN, Y, YFN, BODYFN> {
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
    pub fn y(mut self, y: YFN) -> PartialAdd1<X, XFN, Added, YFN, BODYFN> {
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
    pub fn call(self) -> u32 {
        (self.body)(self.x.unwrap()(), self.y.unwrap()())
    }
}
