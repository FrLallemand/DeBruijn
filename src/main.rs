use std::env;
use std::io::Write;
use std::ops::Deref;

type Edge = (usize,usize,usize, isize, usize);
type Graph = Vec<Edge>;

pub fn new_de_bruijn_graph(neighborhood: usize, nb_states: usize) -> Graph {
    let nb_nodes = nb_states.pow((neighborhood-1) as u32);
    let mut graph = vec![(0, 0, 0, -1, 0) ;(nb_nodes * nb_states)];
    for i in 0..nb_nodes {
        for j in 0..nb_states {
            let index = i*nb_states + j;
            let opposed = index % nb_nodes * nb_states + j;
            graph[index] = (i, index % nb_nodes, j, -1, opposed);
        }
    }
    graph
}

pub fn render_to<W: Write>(graph: &Graph, base: usize, output: &mut W) {
    write!(output, "digraph deBruijn {{").unwrap();
    for edge in graph {
        write!(output, "\t{} -> {} [label={}]", to_str_with_base(edge.0, base), to_str_with_base(edge.1, base), edge.3).unwrap();
        //write!(output, "\t{} -> {} [label={}]", edge.0, edge.1, edge.3).unwrap();
    }
    write!(output, "}}").unwrap();
}

pub fn order_maximise_cycle(graph: &mut Graph, nb_states: usize){
    let mut edges_in_cycle: usize = 0;
    let mut next: isize = 0;
    //println!("{:?}", graph);
    for i in 1..graph.len()+1{
        find_cycle_at_size(graph, &mut next, &mut edges_in_cycle, i, nb_states);
    }
    //println!("{:?}", graph);
    //graph
}

fn find_cycle_at_size(graph: &mut Graph, next: &mut isize, edges_in_cycle: &mut usize, edges_to_add:  usize, nb_states: usize) -> bool{
    let mut success = false;
    for i in 0..graph.len(){
        if graph[i].3 == -1 {
            graph[i].3 = *next;
            *next+=1;
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
    let mut nb_edges_in_cycle = 0;
    let reachability = reachability(graph, nb_states);
    for (index, edge) in graph.iter().enumerate() {
        if(edge.3 > -1 && reachability[index]){
            nb_edges_in_cycle+=1;
        }
    }
    nb_edges_in_cycle
}

fn reachability(graph: &mut Graph, nb_states: usize) -> Vec<bool>{
    let mut reachability = vec![false ;graph.len()];
    for (index, edge) in graph.iter().enumerate() {
        if edge.3 > -1 {
            reachability[index] = true;
        }
    }

    //println!("{:?}", reachability);
    let mut changed = true;
    while(changed){
        changed = false;
        for (index, edge) in graph.iter().enumerate() {
            //if !reachability[index] {
                //println!("{:?}", reachability[index]);
                for (j, edge_test) in graph.iter().enumerate() {
                    if(edge_test.0 == edge.1) && (edge_test.1 == edge.0) {
                        if(reachability[index]) && (reachability[j]){

                            reachability[index] = true;
                            changed = true;
                            break;
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


fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Must have at least 2 arguments : neighborhood and number of states.");
    }
    let neighborhood = args[1].parse::<usize>().expect("Unable to parse first argument ");
    let nb_states = args[2].parse::<usize>().expect("Unable to parse first argument ");

    /*
    let mut graph = new_de_bruijn_graph(neighborhood, nb_states);
    order_maximise_cycle(&mut graph, nb_states);
    println!("{:?}",  graph);


    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    render_to(&graph, nb_states, &mut f);
*/
    let mut graph = vec![(0, 0, 0, -1, 0), (0, 1, 0, -1, 0), (1, 0, 0, -1, 0), (1, 1, 0, -1, 0)];
    println!("{:?}", reachability(&mut graph, 2));

    /*
    use std::fs::File;
    let mut f = File::create("test.dot").unwrap();
    render_to(&graph, nb_states, &mut f);
    */
    //graph.order_maximise_cycle();
}
