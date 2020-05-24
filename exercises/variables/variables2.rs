// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn main() {
    // https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md
    // the default int type (eg: 10) is i32
    // let x = 10i32;
    let x : f64 = 1.0f64;

    // turns out rust will automatically convert for equality '=='
    // if it's possibl to do so w/o any loss of information
    // so the x==10 causes the 10 to be converted (afaik)
    // though if we chose inconvertible types for both (to avoid the autoconversion) we get a compiler error
    // let x = 10u8;
    // if x == 10i8 {
    if x == 10f64 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
