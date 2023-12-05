pub mod thread_join;
pub mod move_closure;

fn main() {
    thread_join::run();
    move_closure::run();
}
