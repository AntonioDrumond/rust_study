use std::fmt;
use std::error::Error;
use std::collections::HashSet;
//use std::collections::HashMap;


//#[derive(default, debug)]
#[allow(dead_code)]
struct Graph{
    pub n:i32,
    pub last_vert:i32,
    pub directed:bool,
    pub arr:Vec<HashSet<i32>>,
}

#[derive(Debug)]
enum GraphError {
    InvalidVertices,
    ExistingEdge,
    NonExistingEdge,
}

impl Error for GraphError {}
impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphError::InvalidVertices => write!(f, "Specified vertices out of bounds."),
            GraphError::ExistingEdge => write!(f, "Specified edge already exists."),
            GraphError::NonExistingEdge => write!(f, "Specified edge does not exist."),
        }
    }
}

#[allow(dead_code)]
impl Graph{

    fn simple(n:i32) -> Graph {
        Graph {n: n, last_vert: 0, directed: false, arr: vec![HashSet::with_capacity({n-1} as usize); n as usize]}
    }

    fn directed(n:i32, directed: bool) -> Graph{
        Graph {n: n, last_vert: 0, directed: directed, arr: vec![HashSet::with_capacity({n-1} as usize); n as usize]}
    }
    
    fn add_vert(&mut self) -> bool {
        if self.last_vert < self.n {
            self.last_vert += 1;
            return true;
        }
        false
    }

    fn all_verts(&mut self){
        while self.add_vert() {}
    }

    fn vert_neighbors(&self, vert:i32) -> Option<&HashSet<i32>> {
        if vert > self.last_vert {
            None
        }
        else {
            Some(&self.arr[vert as usize])
        }
    }

    fn add_edge(&mut self, vert1:i32, vert2:i32) -> Result<(), GraphError> {
        if vert1 > self.last_vert || vert2 > self.last_vert {
            Err(GraphError::InvalidVertices)
        } else if self.arr[vert1 as usize].contains(&vert2) {
            Err(GraphError::ExistingEdge)
        } else {
            self.arr[vert1 as usize].insert(vert2);
            if self.directed {
                self.arr[vert2 as usize].insert(vert1);
            }
            Ok(())
        }
    }

    fn check_edge(&self, vert1:i32, vert2:i32) -> bool {
        if vert1 <= self.last_vert && vert2 <= self.last_vert {
            if !self.directed {
                self.arr[vert1 as usize].contains(&vert2) && self.arr[vert2 as usize].contains(&vert1)
            }
            else { self.arr[vert1 as usize].contains(&vert2) }
        }
        else { false }
    }

    fn remove_edge(&mut self, vert1:i32, vert2:i32) -> Result<(), GraphError> {
        if self.check_edge(vert1, vert2) {
            self.arr[vert1 as usize].remove(&vert2);
            if self.directed { self.arr[vert2 as usize].remove(&vert1); }
            Ok(())
        }
        else { Err(GraphError::NonExistingEdge) }
    }

    fn print_csacademy(&self) {
        for i in 0..self.last_vert {
            println!("{}", i);
        }
        for i in 0..self.last_vert {
            for neighbor in &self.arr[i as usize] {
                println!("{} {}", i, neighbor);
            }
        }
    }

}


fn main() {
    let mut g1 = Graph::simple(5);
    g1.all_verts();

    g1.add_edge(1, 2).expect("1");
    g1.add_edge(2, 3).expect("2");
    g1.add_edge(1, 4).expect("3");
    g1.add_edge(2, 4).expect("4");
    g1.add_edge(0, 4).expect("4");

    g1.print_csacademy();
}
