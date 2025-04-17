mod other; 

mod m1 {
    pub const A:i32 = 120;
    mut var = 20;
    pub fn func1(){
        println!("func1 called");
    }
}

fn main() {
    println!("Inicio do programa");

    m1::func1();
    other::func();

    m1::var = 202;
    println!("{m1::var}");
}
