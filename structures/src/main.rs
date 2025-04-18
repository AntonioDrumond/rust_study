#[derive(Debug, Default)]
struct St{
    p: i32,
    pub a: i32,
}

mod m1{
    #[derive(Debug, Default)]
    pub struct St1{
        p: i32,
    }

    pub fn set_p(mut s: St1, x: i32){
        s.p = x;
    }

    pub fn get_p(s: St1) -> i32 {
        s.p
    }
}

fn main() {
    println!("Program start");

    let mut ins = St::default();

    ins.a += 32;

    ins.p += 23;

    println!("a = {} | p = {}", ins.a, ins.p);

    let mut oi = m1::St1::default();
    m1::set_p(oi, 23);
    println!("p = {}", m1::get_p(oi));
    
}
