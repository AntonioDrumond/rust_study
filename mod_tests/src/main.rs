mod other; // including external file

mod m1 {
    pub const _A:i32 = 120;
    const _B:i32 = 30;

    pub fn func1(){
        println!("func1 called");
    }

    pub fn get_b() -> i32{
        _B
    }
}

mod m3 {
    use super::m1::*;

    pub fn f1(){
        println!("A = {_A}");
    }

    /* // Wont work bc _B is private
    pub fn f2(){
        println!("B = {_B}");
    }
    */
}

fn main() {
    println!("Inicio do programa");

    m1::func1();

    other::func();

    other::m2::func();

    println!("\nTrying to get not public constant");
    let a = m1::get_b();
    println!("a = {a}");

    println!("\nSuper tests");
    m3::f1();
}
