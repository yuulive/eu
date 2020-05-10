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
stuct PartialApplyAdd<X,Y> {
	x: Option<FnOnce() -> u32>,
	y: Option<FnOnce() -> u32>,
	body: FnOnce(x: u32, y: u32) -> i64,
	ph_x: PantomData::X(),
	ph_y: PantomData::Y(),
}

}
fn add() -> PartialApplyAdd<(),()>{
	// create new struct
}

impl PartialApplyAdd<X,()>{
	fn y(self, y: FnOnce() -> u32) -> PartialApplyAdd<X,bool>{
		// fill and transmute, returning self
	}
}

impl PartialApplyAdd<(),Y>{
	fn x(self, x: FnOnce() -> u32) -> PartialApplyAdd<bool,()>{
		// fill and transmute, returning self
	}
}

impl std::FnOnce<() -> i64> for PartialApplyAdd<bool,bool>{
	fn call(self, y: FnOnce() -> u32) -> PartialApplyAdd<X,bool>{
		self.body(x.unwrap()(), y.unwrap()())
	}
}

// This should provide compile time checking for partial application
// Then something like

main() {
	let add_with_y = add().y(2);
	let add_with_both = add_with_y.x(1);
	println!("add: {}",add_with_both());
}

// would be possible
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

## Ideas
Have the add variable function call the full application if complete. This
should be doable by adding

``` rust
impl PartialApplyAdd<(),bool>{
	fn x(self, x: FnOnce() -> u32) -> i64{
		self.x::<(),Y>()()		
	}
}
```
