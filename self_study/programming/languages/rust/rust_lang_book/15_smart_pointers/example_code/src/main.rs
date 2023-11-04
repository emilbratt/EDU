pub mod box_type;
pub mod deref_trait;
pub mod drop_trait;
pub mod rc_type;
pub mod refcell_type;
pub mod rc_type_inside_refcell_type;
pub mod reference_cycle;
pub mod prevent_reference_cycle;

fn main() {
    box_type::run();
    deref_trait::run();
    drop_trait::run();
    rc_type::run();
    refcell_type::run();
    rc_type_inside_refcell_type::run();
    reference_cycle::run();
    prevent_reference_cycle::run();
}

/*
 * Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
 *
 * Ownership:
 *    Rc<T> enables multiple owners of the same data.
 *    Box<T> and RefCell<T> have single owners.
 *
 * Borrowing:
 *    Box<T> allows immutable or mutable borrows checked at compile time.
 *    Rc<T> allows only immutable borrows checked at compile time.
 *    RefCell<T> allows immutable or mutable borrows checked at runtime.
 *
 * RefCell:
 *    You can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
 *    This is because RefCell<T> allows mutable borrows checked at runtime.
 */
