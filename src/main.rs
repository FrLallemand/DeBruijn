use std::env;
use std::io::Write;
type Edge = (usize,usize,usize, isize);
//sortant et entrant
type Node = Vec<usize>;
struct Graph {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
    neighborhood: usize,
    nb_states: usize,
    nb_nodes: usize
}

impl Graph {
    pub fn new(neighborhood: usize, nb_states: usize) -> Graph {
        let nb_nodes = nb_states.pow((neighborhood-1) as u32);
        let mut edges = vec![(0, 0, 0, -1) ;(nb_nodes * nb_states)];
        let mut nodes = vec![Vec::with_capacity(nb_states); nb_nodes];
        for i in 0..nb_nodes {
            for j in 0..nb_states {
                let index = i*nb_states + j;
                edges[index] = (i, index % nb_nodes, j, -1);
                nodes[i].push(index);
            }
        }

        Graph{edges, nodes, neighborhood, nb_states, nb_nodes}
    }

    pub fn render_to<W: Write>(&self, output: &mut W) {
        write!(output, "digraph deBruijn {{").unwrap();
        for edge in &self.edges {
            //write!(output, "\t{} -> {} [label={}]", to_str_with_base(edge.0, base), to_str_with_base(edge.1, base), edge.3).unwrap();
            write!(output, "\t{} -> {} [label={}]", edge.0, edge.1, edge.3).unwrap();
        }
        write!(output, "}}").unwrap();
    }


    fn reachability(&mut self) -> Vec<Vec<bool>>{
        let size = self.nb_nodes;
        let mut reachability = vec![vec![false; size]; size];

        for edge in &self.edges{
            if edge.3 > -1 {
                reachability[edge.0][edge.1] = true;
            }
        }
        let mut changed = true;
        while(changed){
            changed = false;
            for i in 0..size {
                for j in 0..size {
                    if !reachability[i][j] {
                        for (index, edge) in self.edges.iter().enumerate() {
                            let mut found_path = false;
                            let mut found_opposite = false;
                            for (index2, edge2) in self.edges.iter().enumerate() {
                                if((edge.0 == i) && (edge.1 == edge2.0) && (edge2.1 == j)) &&
                                    (reachability[i][edge.1] && reachability[edge.1][j]) {
                                        reachability[i][j] = true;
                                        changed = true;
                                        found_path = true;
                                        break;
                                    }
                            }
                        }
                    }
                }
            }
        }
        reachability
    }

    fn nb_edges_in_cycle(&mut self) -> usize{
        let mut total = 0;
        let reachabilities = self.reachability();
        for i in 0..reachabilities.len(){
            for j in 0..reachabilities.len(){
                //println!("{:?}", );
                // j peut atteindre i
                if reachabilities[j][i]{
                    // on prend le noeud d'index i et on regarde ses arêtes sortantes
                    //println!("{:?}", self.nodes[i]);

                    for index in self.nodes[i].iter() {
                        if self.edges[*index].1==j && self.edges[*index].3 > -1 {
                            //println!("{:?} {:?}", edge, graph[reachabilities[j][i].1 as usize]);
                            total +=1;
                        }
                    }
                }
            }
        }
        total
    }

    pub fn order_maximise_cycle(&mut self){
        let mut edges_in_cycle: usize = 0;
        let mut next: isize = 0;

        for i in 1..self.edges.len()+1{
            self.find_cycle_at_size(&mut next, &mut edges_in_cycle, i);
        }

       // self.find_cycle_at_size(&mut next, &mut edges_in_cycle, 1);
        //self.find_cycle_at_size(&mut next, &mut edges_in_cycle, 2);
        //self.find_cycle_at_size(&mut next, &mut edges_in_cycle, 3);
        //self.find_cycle_at_size(&mut next, &mut edges_in_cycle, 4);

        println!("{:?}", self.edges);

    }


    fn find_cycle_at_size(&mut self, next: &mut isize, edges_in_cycle: &mut usize, edges_to_add:  usize) -> bool{
        let mut success = false;

        // pour toutes les arêtes qui ne sont pas définies
        for i in 0..self.edges.len(){
            if self.edges[i].3 == -1 {
                self.edges[i].3 = *next;
                *next+=1;
                if edges_to_add > 1 {
                    let next_success = self.find_cycle_at_size(next, edges_in_cycle, edges_to_add-1);
                    if !next_success {
                        self.edges[i].3 = -1;
                        *next-=1;
                    }
                } else {
                    let new_edges_in_cycles = self.nb_edges_in_cycle();
                    if new_edges_in_cycles > *edges_in_cycle {
                        *edges_in_cycle = new_edges_in_cycles;
                        success = true;
                    } else {
                        self.edges[i].3 = -1;
                        *next-=1;
                    }
                }
            } else if edges_to_add > 1 {
                success = true;
            }
        }
        success
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

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_edges_in_cycle() {
        // A graph with a neighborhood of 2 and 3 states, all edges are in cycle
        let mut graph = vec![(0, 0, 0, -1), (0, 1, 1, -1), (0, 2, 2, -1), (1, 0, 0, -1), (1, 1, 1, -1), (1, 2, 2, -1), (2, 0, 0, -1), (2, 1, 1, -1), (2, 2, 2, -1)];
        assert_eq!(nb_edges_in_cycle(&mut graph), 9);

        // A graph where no edges are in cycles
        graph = vec![(1, 0, 1, -1)];
        assert_eq!(nb_edges_in_cycle(&mut graph), 0);

        // A graph were some edges are in a cycle
        let mut graph = vec![(0, 0, 0, -1), (0, 1, 1, -1), (1, 0, 0, -1), (1, 1, 1, -1), (1, 2, 2, -1), (2, 2, 2, -1)];
        assert_eq!(nb_edges_in_cycle(&mut graph), 5);
    }

    #[test]
    fn test_reachability() {
        let mut graph = vec![(0, 0, 0, -1), (0, 1, 1, -1), (1, 0, 0, -1), (1, 1, 1, -1)];
        assert_eq!(reachability(&mut graph), vec![[true, true], [true, true]]);

        // A graph with a neighborhood of 3 and 3 states, all edges are in reachable
        graph = vec![(0, 0, 0, -1), (0, 1, 1, -1), (0, 2, 2, -1), (1, 3, 0, -1), (1, 4, 1, -1), (1, 5, 2, -1), (2, 6, 0, -1), (2, 7, 1, -1), (2, 8, 2, -1), (3, 0, 0, -1), (3, 1, 1, -1), (3, 2, 2, -1), (4, 3, 0, -1), (4, 4, 1, -1), (4, 5, 2, -1), (5, 6, 0, -1), (5, 7, 1, -1), (5, 8, 2, -1), (6, 0, 0, -1), (6, 1, 1, -1), (6, 2, 2, -1), (7, 3, 0, -1), (7, 4, 1, -1), (7, 5, 2, -1), (8, 6, 0, -1), (8, 7, 1, -1), (8, 8, 2, -1)];
        let size = graph[graph.len()-1].0 +1;

        assert_eq!(reachability(&mut graph), vec![vec![true; size]; size]);

        // Same as before, but one edge changes: (2, 8, 2, -1) becomes (2, 4, 2, -1)
        graph = vec![(0, 0, 0, -1), (0, 1, 1, -1), (0, 2, 2, -1), (1, 3, 0, -1), (1, 4, 1, -1), (1, 5, 2, -1), (2, 6, 0, -1), (2, 7, 1, -1), (2, 4, 2, -1), (3, 0, 0, -1), (3, 1, 1, -1), (3, 2, 2, -1), (4, 3, 0, -1), (4, 4, 1, -1), (4, 5, 2, -1), (5, 6, 0, -1), (5, 7, 1, -1), (5, 8, 2, -1), (6, 0, 0, -1), (6, 1, 1, -1), (6, 2, 2, -1), (7, 3, 0, -1), (7, 4, 1, -1), (7, 5, 2, -1), (8, 6, 0, -1), (8, 7, 1, -1), (8, 8, 2, -1)];
        let mut test = vec![vec![true; size]; size];
        test[0][8] = false;
        test[2][8] = false;
        test[3][8] = false;
        test[6][8] = false;

        assert_eq!(reachability(&mut graph), test);
    }

}
 */

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Must have at least 2 arguments : neighborhood and number of states.");
    }
    let neighborhood = args[1].parse::<usize>().expect("Unable to parse first argument ");
    let nb_states = args[2].parse::<usize>().expect("Unable to parse first argument ");

    // let mut graph = new_de_bruijn_graph(neighborhood, nb_states);
    let mut graph = Graph::new(neighborhood, nb_states);
    //println!("{:?}",  graph.edges);
    graph.order_maximise_cycle();
    //println!("{:?}",  graph.nb_edges_in_cycle());

    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    graph.render_to(&mut f);

}
