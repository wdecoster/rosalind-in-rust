use std::io::Read;

fn main() {
    // open a file called rosalind_lexf.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_lexf.txt")
        .expect("Can't open file");
    // the first line is the alphabet
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Can't read file");
    let alphabet = buf
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    // the second line is the length of the permutation
    let length = buf.lines().nth(1).unwrap().parse::<usize>().unwrap();
    // the permutations
    let mut permutations = vec![vec![]];
    // iterate over the length
    for _ in 0..length {
        // create a new vector to store the permutations
        let mut new_permutations = Vec::new();
        // iterate over the permutations
        for permutation in permutations {
            // iterate over the alphabet
            for letter in &alphabet {
                // create a new permutation
                let mut new_permutation = permutation.clone();
                // add the letter to the permutation
                new_permutation.push(letter);
                // add the new permutation to the vector
                new_permutations.push(new_permutation);
            }
        }
        // update the permutations
        permutations = new_permutations;
    }
    // print the permutations
    for permutation in permutations {
        println!(
            "{}",
            permutation
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
