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
        // Solution would be to use a::clone()
    }

    {
        let a = String::from("renato");
        let b = pass(a);
        pri(&b);
        // println!("{a}"); // Doesnt work either, function got ownership and gave to b
        println!("{b}\n"); // This works bc the function only borrows
    }

    {
        let a = "reannn";
        let b = a;
        println!("{a} - {b}"); // Works bc the literal &str i think is a stack type
       // String does not work bc its a heap type.
    }

    {
        let mut a = 20;
        let b = &mut a;
        *b += 90;
        println!("{a}\n");
    }

    {
        let mut starship: Option<String> = None;
        match starship { // This will do nothing
            Some(ref name) => println!("{}", name),
            None => ()
        }
        println!("{:?}", starship); // This will print "None"
        starship = Some("Omaha".to_string());
        match starship { // This will print "Omaha"
            Some(ref name) => println!("{}", name),
            None => ()
        }
        println!("{:?}", starship); // This will print "Some("Omaha")"
    }
    
    {
        let val = "reciprocal";
        let ref r1 = val;
        let r2 = &val;
        assert_eq!(r1, r2);
    }

    {
        let mut a = 30;
        let b1 = &mut a;
        *b1 += 10;
        // a = *b1;
        a -= 5;
        println!("{a} - {b1}");

    }

}
