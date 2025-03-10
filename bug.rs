fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;     // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is 6

    // This will cause a compile-time error!
    // We cannot borrow x immutably (z) while it's already mutably borrowed (y)
    println!("x = {}", *z); 
}