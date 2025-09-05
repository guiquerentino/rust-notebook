fn main() {
    // It is possible to override a variable within a block or inside the scope
    let x: i32 = 10;
    println!("{}", x);
    let x: i64 = 40_000; // overriding "x" variable
    println!("{}", x);

    {
        //overriding within a block
        let x: char = 'A';
        println!("{}", x);
    }
}
