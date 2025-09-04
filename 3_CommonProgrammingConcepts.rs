/*
1. Basic Syntax Overview

Functions: Defined with fn keyword, followed by the function name and parentheses.

Main function: fn main() is the entry point of every Rust program.

Macros: Notice the println! macro. Macros in Rust end with ! and can take variable number of arguments.

Comments:
// for single-line comments
/* ... */ for multi-line comments
*/

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!"); //println! is a macro that prints formatted strings. Macros end with ! and don’t behave exactly like functions.
    
    // =============================
    //  VARIABLES & MUTABILITY
    // =============================
    // - Variables are immutable by default in Rust
    // - Use `let mut` to make them mutable
    // - Variables have a specific type (can be inferred or declared)

    let x = 5; //immutable
    println!("The value of x is: {x}");
    //x = 6; //cannot assign twice to immutable variable
    println!("The value of x is: {x}");

    let mut y = 5; //muatble
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

/*  =============================
     CONSTANTS IN RUST
    =============================
    - Declared using `const`, NOT `let`
    - Always immutable — cannot use `mut`
    - Type annotation is required
    - Must be assigned a value that can be evaluated at compile time
    - Naming convention: ALL_CAPS_WITH_UNDERSCORES
    - Can be declared in any scope (including global scope) */
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);
    // The following would be invalid:
    // const MUTABLE_CONST: i32 = 5;
    // MUTABLE_CONST = 10; // ❌ Error: cannot change a constant
    // Also invalid:
    // const RUNTIME_VALUE: i32 = get_runtime_value(); // ❌ Error: must be const evaluable

    // =============================
    // 🕶️ SHADOWING IN RUST
    // =============================
    // - You can declare a variable with the same name multiple times using `let`
    // - The new declaration "shadows" the previous one (overrides it in scope)
    // - Shadowing creates a NEW variable, not mutating the existing one
    // - Unlike `mut`, shadowing can also change the type of the variable

    // Step 1: Declare x with initial value
    let x = 5;
    println!("Initial x: {x}"); // Output: 5

    // Step 2: Shadow x with a new value (x + 1)
    let x = x + 1;
    println!("After adding 1, x: {x}"); // Output: 6

    // Step 3: Shadow x again in an inner scope (x * 2)
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // Output: 12
    }

    // Step 4: x in outer scope remains unchanged from inner scope
    println!("The value of x is: {x}"); // Output: 6

    // =============================
    // 🔁 SHADOWING VS MUTABILITY
    // =============================
    // - With `mut`, you can't change the type of a variable
    // - With shadowing, you can change both value AND type

    // Shadowing Example: Changing type
    let spaces = "   ";              // type: &str
    let spaces = spaces.len();       // type: usize
    println!("Number of spaces: {spaces}"); // Output: 3

    // ❌ Invalid with `mut` (compile-time error)
    // let mut spaces = "   ";
    // spaces = spaces.len(); // Error: cannot assign usize to &str

    // =============================
    // 🔢 SCALAR TYPES
    // =============================
    // Scalar types represent a single value.
    // Rust has 4 primary scalar types:
    // - Integers
    // - Floating-point numbers
    // - Booleans
    // - Characters

    // -----------------------------
    // 🧮 1. Integer Types
    // -----------------------------
    // i8, i16, i32, i64, i128 → signed
    // u8, u16, u32, u64, u128 → unsigned
    // isize, usize → depends on system architecture (32/64-bit)

    let a: i32 = -100;
    let b: u8 = 255;
    let hex = 0xff;          // Hexadecimal
    let bin = 0b1010_1010;   // Binary
    let byte = b'A';         // Byte (u8 only)

    println!("Integers: {}, {}, {}, {}, {}", a, b, hex, bin, byte);

    // Integer overflow (debug mode panics; release mode wraps)
    // let overflow: u8 = 256; // ⚠️ This will panic in debug mode (e.g., 256u8 becomes 0)

    // -----------------------------
    // 🌊 2. Floating-point Types
    // -----------------------------
    // f32 and f64 (default is f64)
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
    println!("Floats: {}, {}", x, y);

    // -----------------------------
    // ➕ 3. Numeric Operations
    // -----------------------------
    let sum = 5 + 10;              // Addition
    let difference = 95.5 - 4.3;   // Subtraction
    let product = 4 * 30;          // Multiplication
    let quotient = 56.7 / 32.2;    // Division
    let truncated = -5 / 3;        // Integer division → -1
    let remainder = 43 % 5;        // Remainder

    println!(
        "Ops → Sum: {}, Diff: {}, Prod: {}, Quot: {}, Trunc: {}, Rem: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // -----------------------------
    // ✅ 4. Boolean Type
    // -----------------------------
    let t = true;
    let f: bool = false;
    println!("Booleans: t={}, f={}", t, f);

    // -----------------------------
    // 🔤 5. Character Type
    // -----------------------------
    let c = 'z';
    let z: char = 'ℤ'; // Unicode char
    let emoji = '😻';
    println!("Chars: {}, {}, {}", c, z, emoji);

    // =============================
    // 🧱 COMPOUND TYPES
    // =============================
    // Group multiple values into one type.
    // - Tuples
    // - Arrays

    // -----------------------------
    // 📦 1. Tuples
    // -----------------------------
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("Tuple destructured: x={}, y={}, z={}", x, y, z);

    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "Tuple indexed: {}, {}, {}",
        five_hundred, six_point_four, one
    );

    // Unit tuple
    let unit = ();
    println!("Unit tuple: {:?}", unit);

    // -----------------------------
    // 🧮 2. Arrays
    // -----------------------------
    // Arrays have:
    // - Fixed length
    // - Elements of the same type

    let a = [1, 2, 3, 4, 5]; // type: [i32; 5]
    println!("Array a: {:?}", a);

    // Accessing array elements
    let first = a[0];
    let second = a[1];
    println!("Array access: first={}, second={}", first, second);

    // Explicit type annotation: You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    let b: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array b: {:?}", b);

    // Repeating values
    let repeated = [3; 5]; // same as [3, 3, 3, 3, 3]
    println!("Repeated array: {:?}", repeated);

    // -----------------------------
    // ❌ Accessing invalid index (causes panic)
    // -----------------------------
    // Uncommenting the following lines will cause a runtime panic
    // let out_of_bounds = a[10];
    // println!("This will panic: {}", out_of_bounds);

    //Array Indexing:
    //✅ Rust checks bounds at runtime
    //❌ Accessing out-of-bounds index causes panic, not memory corruption


    // -----------------------------
    // 🧮 Functions
    // -----------------------------

    // 🟢 Calling a basic function
    another_function();

    // 🟢 Calling a function with a parameter
    another_with_param(5);

    // 🟢 Calling a function with multiple parameters
    print_labeled_measurement(10, 'm');

    // 🟢 Using block expressions to assign values
    let y = {
        let x = 3;
        x + 1 // ← expression with no semicolon returns value
    };
    println!("The value of y is: {y}");

    // 🟢 Calling a function that returns a value
    let value = five();
    println!("Function returned: {value}");

    // 🟢 Function with parameter and return value
    let result = plus_one(9);
    println!("plus_one(9) returned: {result}");

    //➕ Expressions return values, 🧱 Statements do not return values

    // -----------------------------
    // 🧮 IF expressions
    // -----------------------------
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // The condition must be a bool.
    // Blocks (code inside {}) are expressions.

    let number = 3;
    if number { //❌ This won’t work: Rust doesn’t automatically convert numbers to booleans.
        println!("number was three");
    }

    // -----------------------------
    // 🔀 Using else if for Multiple Conditions
    // -----------------------------
    let number = 6;
    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }
    // Rust runs only the first matching condition.
    // Too many else ifs? Use match (covered in Chapter 6).

    // -----------------------------
    // 📦 Using if in let Statements
    // -----------------------------
    let condition = true;
    let number = if condition { 5 } else { 6 }; //⚠️ Both branches must return the same type.
    println!("The value of number is: {number}");
    
    let number = if condition { 5 } else { "six" }; //❌ This will cause an error:

    // -----------------------------
    // 🔁 Loops in Rust
    // -----------------------------
    //Rust has 3 kinds of loops:
    //1. Infinite loop
    loop {
    println!("again!"); //Use ctrl-c to stop it manually.
    }

    //2. Breaking out of a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Break and return value, ✅ break can return a value from a loop
        }
    };
    println!("Result is: {result}");

    //❌ return exits the entire function

    //3. Nested Loops with Labels
    let mut count = 0;

    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Break inner loop
            }
            if count == 2 {
                break 'outer; // Break outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    //Labels use 'label_name:
    // Use with break or continue

    //🔄 while Loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    //Repeats while the condition is true, Similar to other languages


    //📚 Looping Through Collections:
    //🧱 Using while with index
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    //⚠️ Error-prone: Risk of index out-of-bounds, Manually update index

    //Using for Loop (Recommended)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    //👍 Safe and clean, 🚫 No risk of going past array bounds, 💡 Compiler can optimize better

    //🎯 Countdown with for and rev()
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    //1..4 = 1 to 3 (range is exclusive at the end), rev() reverses the range
    




}




/* fn get_runtime_value() -> i32 {
    42
} */

// ===========================
// 🔧 Basic function definition
// ===========================
fn another_function() {
    println!("Another function.");
}

// ===========================
// 📥 Function with 1 parameter
// ===========================
fn another_with_param(x: i32) {
    println!("The value of x is: {x}");
}

// ===========================
// 📥📥 Function with 2 parameters
// ===========================
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// ===========================
// 🔁 Function with return value
// No semicolon at end → treated as expression
// ===========================
fn five() -> i32 {
    5 // ← This is an expression, returns 5
}

// ===========================
// 🔁 Function with input & return
// ===========================
fn plus_one(x: i32) -> i32 {
    //x + 1; // ❌ This becomes a statement, so nothing is returned!
    x + 1 // ← No semicolon → returns x + 1
}

// ===========================
// 🛑 INVALID: Uncommenting this will not compile!
// Demonstrates statement vs expression issue
// ===========================
/*
fn plus_one_invalid(x: i32) -> i32 {
    x + 1; // ← semicolon makes it a statement → returns ()
}
*/