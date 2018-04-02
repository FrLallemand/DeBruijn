use std::env;
use std::io::Write;
type Edge = (usize,usize,usize, isize);
type Graph = Vec<Edge>;

pub fn new_de_bruijn_graph(neighborhood: usize, nb_states: usize) -> Graph {
    let nb_nodes = nb_states.pow((neighborhood-1) as u32);
    let mut graph = vec![(0, 0, 0, -1) ;(nb_nodes * nb_states)];
    for i in 0..nb_nodes {
        for j in 0..nb_states {
            let index = i*nb_states + j;
            graph[index] = (i, index % nb_nodes, j, -1);
        }
    }
    graph
}

pub fn render_to<W: Write>(graph: &Graph, base: usize, output: &mut W) {
    write!(output, "digraph deBruijn {{").unwrap();
    for edge in graph {
        //write!(output, "\t{} -> {} [label={}]", to_str_with_base(edge.0, base), to_str_with_base(edge.1, base), edge.3).unwrap();
        write!(output, "\t{} -> {} [label={}]", edge.0, edge.1, edge.3).unwrap();
    }
    write!(output, "}}").unwrap();
}

pub fn order_maximise_cycle(graph: &mut Graph, nb_states: usize){
    let mut edges_in_cycle: usize = 0;
    let mut next: isize = 0;
    for i in 1..(graph.len()){
    find_cycle_at_size(graph, &mut next, &mut edges_in_cycle, i, nb_states);

    }
    //println!("{:?}", graph);

}

fn find_cycle_at_size(graph: &mut Graph, next: &mut isize, edges_in_cycle: &mut usize, edges_to_add:  usize, nb_states: usize) -> bool{
    let mut success = false;
    for i in 0..graph.len(){
        if graph[i].3 == -1 {
            graph[i].3 = *next;
            *next+=1;
            if *edges_in_cycle == 8 && edges_to_add == 2{
                println!("{:?} {:?} {:?}", graph[i], i,  edges_to_add);
            }

            if edges_to_add > 1 {

                let next_success = find_cycle_at_size(graph, next, edges_in_cycle, edges_to_add-1, nb_states);
                if !next_success {
                    graph[i].3 = -1;
                    *next-=1;
                }
           } else {
                let new_edges_in_cycles = nb_edges_in_cycle(graph, nb_states);

                if new_edges_in_cycles > *edges_in_cycle {
                    *edges_in_cycle = new_edges_in_cycles;
                    success = true;
                } else {
                    graph[i].3 = -1;
                    *next-=1;
                }
            }
        }
    }
    success
}

fn nb_edges_in_cycle(graph: &mut Graph, nb_states: usize) -> usize{
    let mut total = 0;
    let reachabilities = reachability(graph);
    for i in 0..reachabilities.len(){
        for j in 0..reachabilities.len(){
            println!("{:?} {:?}", i*nb_states + j, graph.len());

            if i*nb_states + j < graph.len() && graph[i*nb_states + j].3 > -1 && reachabilities[j][i]{
                total+=1;
            }
        }
    }
    /*
    if graph[0].3 == 0 && graph[2].3 == -1 && graph[1].3 == 1{
        //println!("{:?} {:?}", graph, total);
        //println!("{:?}", reachabilities);
        let size = graph[graph.len()-1].0 +1;
        let mut r = vec![vec![false ;size] ; size];

        for edge in graph.iter_mut() {
            if edge.3 > -1 {
                r[edge.0][edge.1] = true;
            }
        }
        //println!("{:?}", r);

    }*/
    total
}

fn reachability(graph: &mut Graph) -> Vec<Vec<bool>>{
    //let size = graph[graph.len()-1].0 +1;
    let size = graph[graph.len()-1].0 +1;
    let mut reachability = vec![vec![false ;size] ; size];

    for edge in graph.iter_mut() {
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
                    for (index, edge) in graph.iter().enumerate() {
                        for (index2, edge2) in graph.iter().enumerate() {
                            if((edge.0 == i) && (edge.1 == edge2.0) && (edge2.1 == j)) &&
                                (reachability[i][edge.1] && reachability[edge.1][j]) {
                                    reachability[i][j] = true;
                                    changed = true;
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

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Must have at least 2 arguments : neighborhood and number of states.");
    }
    let neighborhood = args[1].parse::<usize>().expect("Unable to parse first argument ");
    let nb_states = args[2].parse::<usize>().expect("Unable to parse first argument ");

    let mut graph = new_de_bruijn_graph(neighborhood, nb_states);

    //println!("{:?}",  reachability(&mut graph));
    order_maximise_cycle(&mut graph, nb_states);
    //println!("{:?}",  graph);

    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    render_to(&graph, nb_states, &mut f);

   // let mut graph = new_de_bruijn_graph(neighborhood, nb_states);
   // reachability(&mut graph);


    /*
    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    render_to(&graph, nb_states, &mut f);
    */
    //graph.order_maximise_cycle();
}
