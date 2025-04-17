fn func() -> u32{
    println!("called");
    1233
}

fn func2() {
    println!("func2 called");
    fn internal() ->i32{
        println!("internal called");
        2 + 3
    }
    println!("internal result is {}", internal());
}

fn main() {
    println!("Inicio do programa");
    let a = 10;
    { // block and scope test
        println!("a = {a}");
        let a = 20;
        println!("a = {a}");
    }
    println!("a = {a}");

    // Lambda func 
    let c = {
        let t1 = 20;
        let t2 = 3;
        t1 * t2
    };
    println!("\nc = {}", c);

    let d = func();
    println!("\nd = {}", d);

    // Patterns test
    let (x, y) = (15, 32);
    println!("x = {x} | y = {y}");

    // Function scoping test
    func2();
    // let l = internal(); // Wont work, function is out of scope

}
