use partial_application_rs;
use partial_application_rs::part_app;

#[part_app]
fn foo(bar: u32, baz: u64) -> i16 {
    (bar + baz as u32) as i16
}
