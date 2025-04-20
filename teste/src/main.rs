fn func() -> u32{
    println!("called");
    1233
}

fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
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
    
    {
        let d = 10;
        let dou = |x|x*2;
        println!("d = {d} | double = {}", dou(d));
        print_type(&dou);
        print_type(&d);
    }

    {
        let var = 20;
        let ptr = &var;
        let var = 30;
        println!("drf = {} | var = {}", *ptr, var);
        // Prints "drf = 20 | var = 30"
    }

}
