#[derive(Debug, Default)]
struct St{
    p: i32,
    pub a: i32,
}

impl St{
    fn new(x: i32, y: i32) -> St{
        St { p: x, a: y }
    }

    fn print(&self){
        println!("a = {}\np = {}", self.a, self.p);
    }

}

trait TrTest {
    const TCON:&str = "renato";
}

impl TrTest for St {}

fn main() {
    println!("Program start");

    let mut ins = St::default();
    ins.a += 32;
    ins.p += 23;
    println!("a = {} | p = {}", ins.a, ins.p);

    let inx = St::new(21, 12);
    inx.print();

    println!("\n{}\n", St::TCON);
}
