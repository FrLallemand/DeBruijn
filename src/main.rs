use std::env;
use std::io::Write;

struct DeBruijn {
    edges: Vec<(usize,usize,usize)>,
    nb_states: usize
}

impl DeBruijn {
    pub fn new(neighborhood: usize, nb_states: usize) -> DeBruijn {
        let nb_nodes = nb_states.pow((neighborhood-1) as u32);
        let mut edges = vec![(0, 0, 0) ;(nb_nodes * nb_states)];
        for i in 0..nb_nodes {
            for j in 0..nb_states {
                edges[i*nb_states + j] = (i, (i*nb_states + j) % nb_nodes, j);
            }
        }

        DeBruijn {edges: edges, nb_states: nb_states}
    }

    pub fn render_to<W: Write>(&self, output: &mut W) {
        write!(output, "digraph DeBruijn {{").unwrap();
        for edge in &self.edges {
            write!(output, "\t{} -> {} [label={}]", to_str_with_base(edge.0, self.nb_states), to_str_with_base(edge.1, self.nb_states), edge.2).unwrap();
        }
        write!(output, "}}").unwrap();
    }

}

fn to_str_with_base(mut num: usize, base: usize) -> String {
    let chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut s = String::new();
    while num != 0 {
        s.push(chars[num % base]);
        num /= base;
    }
    while s.len() < base-1 { s.push('0'); }
    s.chars().rev().collect()
}


fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Must have at least 2 arguments : neighborhood and number of states.");
    }
    let neighborhood = args[1].parse::<usize>().expect("Unable to parse first argument ");
    let nb_states = args[2].parse::<usize>().expect("Unable to parse first argument ");

    let graph = DeBruijn::new(neighborhood, nb_states);
    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    graph.render_to(&mut f)
}
