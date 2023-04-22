use std::io::Read;

fn main() {
    // open a file called rosalind_hamm.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_hamm.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents");
    // split the string into a vector of strings
    let contents = contents.split_whitespace().collect::<Vec<&str>>();
    // get the first sequence
    let sequence1 = contents[0];
    // get the second sequence
    let sequence2 = contents[1];
    // create a variable to store the hamming distance
    let mut hamming_distance = 0;
    // iterate over the sequences
    for (base1, base2) in sequence1.chars().zip(sequence2.chars()) {
        // if the bases are not equal
        if base1 != base2 {
            // increase the hamming distance
            hamming_distance += 1;
        }
    }
    // print the hamming distance
    println!("{}", hamming_distance);
}
