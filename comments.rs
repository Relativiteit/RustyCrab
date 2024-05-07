fn main() {
    // simple comment
    // lib docs comment
    println!("This file is for comments");
    // lib docs enclosing?
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
