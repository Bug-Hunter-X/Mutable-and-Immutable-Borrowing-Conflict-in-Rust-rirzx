fn main() {
    let mut x = 5;
    {
        let y = &mut x; // Mutable borrow
        *y += 1;       // Modify x
    } // Mutable borrow ends here

    let z = &x;     // Immutable borrow is now safe
    println!("x = {}", *z); // Output: x = 6
} 