use std::io::Read;

fn main() {
    // open a file called rosalind_lcsm.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_lcsm.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents");
    // split the string into a vector of strings
    let contents = contents.split(">").collect::<Vec<&str>>();
    // create a vector to store the sequences
    let mut sequences = Vec::new();
    // iterate over the contents
    for sequence in contents {
        // if the sequence is not empty
        if sequence != "" {
            // split the sequence into a vector of strings
            let sequence = sequence.split_whitespace().collect::<Vec<&str>>();
            // get the sequence
            let sequence = sequence[1..].join("");
            // add the sequence to the vector
            sequences.push(sequence);
        }
    }
    // sort the vector of sequences by length
    sequences.sort_by(|a, b| a.len().cmp(&b.len()));
    // find the longest common substring of the sequences
    let mut longest_common_substring = String::new();
    // iterate over the first sequence
    for i in 0..sequences[0].len() {
        // iterate over the second sequence
        for j in i..sequences[0].len() {
            // get the substring
            let substring = &sequences[0][i..j];
            // check if the substring is in all the sequences
            if sequences.iter().all(|s| s.contains(substring)) {
                // if the substring is longer than the longest common substring
                if substring.len() > longest_common_substring.len() {
                    // set the longest common substring to the substring
                    longest_common_substring = substring.to_string();
                }
            }
        }
    }
    // print the longest common substring
    println!("{}", longest_common_substring);
}
