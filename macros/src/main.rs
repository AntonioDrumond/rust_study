fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

macro_rules! um {
    ($a:expr) => {
        {
            println!($a);
        }
    };

    ($a:expr, $b:expr) => {
        println!("{}, {}", $b, $a);
    };
}

macro_rules! add_all{
    ( $($a:expr) , *)=>{
        {
            0
            $(+$a)*
        }
    };
}


macro_rules! input {
    ($a:ident) => {
        std::io::stdin().read_line(&mut $a).unwrap();
    };

    () => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line
        }
    };
}


fn main() {
    um!("Hello", "world!");
    println!("{}", add_all!());
    println!("{}", add_all!(2,3,4));

    let mut str1 = String::new();
    input!(str1);
    println!("{}", str1);

    let str2 = input!();
    println!("{}", str2);

    print_type(&"primeiro");
    let str3 = "segundo";
    let str4 = String::from("terceiro");
    print_type(&str3);
    print_type(&str4);
}
