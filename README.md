# Macro to make functions partially applicable

## Example
``` rust
// Simple function
fn add(x: u32, y: u32) -> i64 {
	(x + y) as i64
}

fn main() {
	println!("add: {}",add(1,2));
}
```

The idea is to define a macro to turn a function into a partial application
supporting struct. This would look something like

``` rust
#[part_app]
fn add(x: u32, y: u32) -> i64 {
	(x + y) as i64
}

fn main() {
	let a = add();
	let two = a.x(|| 2);
    let number = two.y(|| 40);
    assert_eq!(number.call(), 42);
}
```

The `#[part_app]` would expand into something like this (edited for brevity). 

``` rust
struct add___Added;

struct add___Empty;

struct __PartialApplication__add_<x, x___FN, y, y___FN, BODYFN>
where
    xFN: FnOnce() -> u32,
    yFN: FnOnce() -> u32,
    BODYFN: FnOnce(u32, u32) -> i64,
{
    xm: ::std::marker::PhantomData<x>,
    ym: ::std::marker::PhantomData<y>,
    x: Option<xFN>,
    y: Option<yFN>,
    body: BODYFN,
}

fn add<x, y>(
) -> __PartialApplication__add_<addEmpty, x, addEmpty, y, impl FnOnce(u32, u32) -> i64>
where
    x: FnOnce() -> u32,
    y: FnOnce() -> u32,
{
    __PartialApplication__add_ {
        x: None,
        y: None,
        xm: ::std::marker::PhantomData,
        ym: ::std::marker::PhantomData,
        body: |x, y| (x + y) as i64,
    }
}

impl<xFN: FnOnce() -> u32, yFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64, y>
    __PartialApplication__add_<addEmpty, xFN, y, yFN, BODYFN>
{
    fn x(
        mut self,
        x: xFN,
    ) -> __PartialApplication__add_<addAdded, xFN, y, yFN, BODYFN> {
        self.x = Some(x);
        unsafe {
            ::std::mem::transmute_copy::<
                __PartialApplication__add_<addEmpty, xFN, y, yFN, BODYFN>,
                __PartialApplication__add_<addAdded, xFN, y, yFN, BODYFN>,
            >(&self)
        }
    }
}

impl<xFN: FnOnce() -> u32, yFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64, x>
    __PartialApplication__add_<x, xFN, addEmpty, yFN, BODYFN>
{
    fn y(
        mut self,
        y: yFN,
    ) -> __PartialApplication__add_<x, xFN, addAdded, yFN, BODYFN> {
        self.y = Some(y);
        unsafe {
            ::std::mem::transmute_copy::<
                __PartialApplication__add_<x, xFN, addEmpty, yFN, BODYFN>,
                __PartialApplication__add_<x, xFN, addAdded, yFN, BODYFN>,
            >(&self)
        }
    }
}

impl<xFN: FnOnce() -> u32, yFN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64>
    __PartialApplication__add_<addAdded, xFN, addAdded, yFN, BODYFN>
{
    fn call(self) -> i64 {
        (self.body)(self.x.unwrap()(), self.y.unwrap()())
    }
}
```

Importantly, this would be a zero-cost abstraction. In theory, the `Option`s
will be removed because they are not checked against. The `FnOnce` can be
optimized out (as it's just the compiler manipulating the syntax tree) so then
the struct holds no unoptimizable data. This means its size should be
effectively 0, and thus it will be optimized away.

## How it works
The macro creates a function which produces a builder pattern like struct. The
struct is parameterized by which variables are defined. Defining a variable is
only implemented for the struct if that variable is not already defined. The
final call is defined only when each variable is itself defined. A variable is
marked as defined if its place paramater is of type `Added`. It is marked as
undefined when its place is parameterized by type `Empty`. 

## Limitations
In an effort to make this as optimizable as possible, I avoid any heap
allocations. This prevents me from abstracting over types of closures. Any
instance of a PartialApplication struct can only hold one type of closure. This
also prevents copying. To avoid this, adding the attribute `poly` enables heap
allocation and thus all closures with the same trait are equally acceptable. The
attribute `Clone` enables partially constructed functions to be cloned before
they are called. `value` enables passing in values instead of structs. 
