/*
* Generic types on Structs
*
 * NOTE: Passing the values to the struct will make it so the generic types
 *       become the correct primitive type when compiled.
 *       This is called "type coercion"
 *       e.g. ..forcing a type based on what the language imply it should be.
 */

fn example_same_type() {
    struct ValueXY<T> {
        x: T,
        y: T,
    }

    impl ValueXY<f32> {
        // Since these are same generic type, implement arithmetic operations is easy
        fn multiplied(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let value_x_and_y = ValueXY {
        x: 5.5,
        y: 10.0,
    };

    let multiplied = value_x_and_y.multiplied();
    println!("multiplied value is: {}", multiplied);
}

fn example_multiple_types() {
    struct ValueXY<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> ValueXY<T, U> {
        /*
        * NOTE: We have to declare <T, U> just after impl so we can use T and U in the methods.
        *
        * E.g. By declaring generic types <T, U> after impl -> Rust knows that <T, U> after ValueXY is generic.
        */

        fn get_ref_x(&self) -> &T {
            &self.x // returns a reference to ValueXY.x
        }
        
        fn get_ref_y(&self) -> &U {
            &self.y // returns a reference to ValueXY.y
        }
    }

    let value_x_and_y = ValueXY {
        x: 5.5,
        y: 10,
    };

    let x_position = value_x_and_y.get_ref_x();
    println!("x value is: {}", x_position);

    let y_position = value_x_and_y.get_ref_y();
    println!("y value is: {}", y_position);

}

fn example_multiple_types_v2() {
    /*
     * Here, we use multiple types and also mix them up depending on context.
     * This example uses the generic types X1 and Y1 for the Point struct
     * and X2 Y2 for the mixup method signature.
     */
    struct ValueXY<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> ValueXY<X1, Y1> {
        fn mixup<X2, Y2>(self, other: ValueXY<X2, Y2>) -> ValueXY<X1, Y2> {
            ValueXY {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = ValueXY { x: 5, y: 10.4 };
    let p2 = ValueXY { x: "Hello", y: 'c' };

    let mixup = p1.mixup(p2);
    println!("mixup3.x = {}, mixup.y = {}", mixup.x, mixup.y);
}

pub fn run_example() {
    example_same_type();
    example_multiple_types();
    example_multiple_types_v2();
}
