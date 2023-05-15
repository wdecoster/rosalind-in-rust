use std::io::Read;

fn main() {
    // open a file called rosalind_tree.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_tree.txt")
        .expect("Can't open file");
    // the first line is the number of nodes
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Can't read file");
    let number_of_nodes = buf.lines().next().unwrap().parse::<usize>().unwrap();
    // the rest of the lines are adjacencies
    let mut adjacencies = vec![vec![]; number_of_nodes];
    for line in buf.lines().skip(1) {
        let mut split = line.split_whitespace();
        let a = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let b = split.next().unwrap().parse::<usize>().unwrap() - 1;
        adjacencies[a].push(b);
        adjacencies[b].push(a);
    }
    // the number of edges to add to form a tree
    let number_of_edges_to_add = number_of_connected_components(&adjacencies) - 1;
    println!("{}", number_of_edges_to_add);
}

fn number_of_connected_components(adjacencies: &[Vec<usize>]) -> usize {
    let mut visited = vec![false; adjacencies.len()];
    let mut number_of_connected_components = 0;
    for i in 0..adjacencies.len() {
        if !visited[i] {
            number_of_connected_components += 1;
            dfs(i, adjacencies, &mut visited);
        }
    }
    number_of_connected_components
}

fn dfs(i: usize, adjacencies: &[Vec<usize>], visited: &mut [bool]) {
    visited[i] = true;
    for &j in &adjacencies[i] {
        if !visited[j] {
            dfs(j, adjacencies, visited);
        }
    }
}
