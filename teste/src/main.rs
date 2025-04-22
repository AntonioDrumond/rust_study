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

    let d = func();
    println!("\nd = {}", d);

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
        println!("drf = {} | var = {}\n", *ptr, var);
        // Prints "drf = 20 | var = 30"
    }

    {
        const CON:i32 = 48;
        println!("{CON}\n");
        // const CON:i32 = 38; // Doesnt work, cant shadow consts
    }

}
