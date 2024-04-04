pub mod thread_join;
pub mod move_closure;
pub mod message_passing;
pub mod shared_state;

fn main() {
    println!("\n\nthread_join.rs\n");
    thread_join::run();

    println!("\n\nmove_closure.rs\n");
    move_closure::run();

    println!("\n\nmessage_passing.rs\n");
    message_passing::run();

    println!("\n\nshared_state.rs\n");
    shared_state::run();
}
