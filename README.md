# Sketch for a library to turn functions in to partial application structs

## Example
``` rust
// Simple function
fn add(x: u32, y: u32) -> i64 {
	(x + y) as i64
}

main() {
	println!("add: {}",add(1,2));
}
```

The idea is to define a macro to turn a function into an partial application
supporting struct. This would look somthing like

``` rust
#[part_app]
fn add(x: u32, y: u32) -> i64 {
	(x + y) as i64
}
```

// which would expand to

``` rust
struct add___Added;

struct add___Empty;

struct __PartialApplication__add_<x, x___FN, y, y___FN, BODYFN>
where
    x___FN: FnOnce() -> u32,
    y___FN: FnOnce() -> u32,
    BODYFN: FnOnce(u32, u32) -> i64,
{
    x___m: ::std::marker::PhantomData<x>,
    y___m: ::std::marker::PhantomData<y>,
    x: Option<x___FN>,
    y: Option<y___FN>,
    body: BODYFN,
}

fn add<x, y>(
) -> __PartialApplication__add_<add___Empty, x, add___Empty, y, impl FnOnce(u32, u32) -> i64>
where
    x: FnOnce() -> u32,
    y: FnOnce() -> u32,
{
    __PartialApplication__add_ {
        x: None,
        y: None,
        x___m: ::std::marker::PhantomData,
        y___m: ::std::marker::PhantomData,
        body: |x, y| (x + y) as i64,
    }
}

impl<x___FN: FnOnce() -> u32, y___FN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64, y>
    __PartialApplication__add_<add___Empty, x___FN, y, y___FN, BODYFN>
{
    fn x(
        mut self,
        x: x___FN,
    ) -> __PartialApplication__add_<add___Added, x___FN, y, y___FN, BODYFN> {
        self.x = Some(x);
        unsafe {
            ::std::mem::transmute_copy::<
                __PartialApplication__add_<add___Empty, x___FN, y, y___FN, BODYFN>,
                __PartialApplication__add_<add___Added, x___FN, y, y___FN, BODYFN>,
            >(&self)
        }
    }
}

impl<x___FN: FnOnce() -> u32, y___FN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64, x>
    __PartialApplication__add_<x, x___FN, add___Empty, y___FN, BODYFN>
{
    fn y(
        mut self,
        y: y___FN,
    ) -> __PartialApplication__add_<x, x___FN, add___Added, y___FN, BODYFN> {
        self.y = Some(y);
        unsafe {
            ::std::mem::transmute_copy::<
                __PartialApplication__add_<x, x___FN, add___Empty, y___FN, BODYFN>,
                __PartialApplication__add_<x, x___FN, add___Added, y___FN, BODYFN>,
            >(&self)
        }
    }
}

impl<x___FN: FnOnce() -> u32, y___FN: FnOnce() -> u32, BODYFN: FnOnce(u32, u32) -> i64>
    __PartialApplication__add_<add___Added, x___FN, add___Added, y___FN, BODYFN>
{
    fn call(self) -> i64 {
        (self.body)(self.x.unwrap()(), self.y.unwrap()())
    }
}
```

Importantly, this would be a zero-cost abstraction. In theory, the `Option`s
will be removed because they are not checked against. The `FnOnce` can be
optimized out (as it's just the compiler manipulating the syntax tree) so then
the struct holds no unoptimizable data. This means it's size should be
effectively 0, and thus it will be optimized away.

## How it works
The macro creates a function which produces a builder pattern like struct. The
struct is parameterized by which variables are defined. Defining a variable is
only implemented for the struct if that variable is not already defined. The
final call is defined only when each variable is itself defined. A variable is
marked as defined if it's place paramater is of type `bool`. It is marked as
undefined when it's place is parameterized by type `()`. 

## Limitations
In an effort to make this as optimizable as possible, I avoid any heap
allocations. This prevents me from abstracting over types of closures. Any
instance of a PartialApplication struct can only hold one type of closure.
