fn pass(s: String) -> String {
    s
}

fn pri(s: &String){ // Need to specify borrow here
    println!("{s}");
}


fn main() {
    println!("Inicio do programa\n");

    {
        let a = String::from("faz o L");
        let b = a;

        // println!("{a}"); // a is NOT accessible bc b borrowed it
        println!("{b}\n");
    }


    {
        let a = String::from("renato");
        let b = pass(a);
        // pri(&a);

        // println!("{a}"); // Doesnt work either, function got ownership and gave to b
        println!("{b}\n");
    }

}
