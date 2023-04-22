use std::io::Read;

fn main() {
    // open a file called rosalind_subs.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_subs.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents");
    // split the string into a vector of strings
    let contents = contents.split_whitespace().collect::<Vec<&str>>();
    // get the sequence
    let sequence = contents[0];
    // get the substring
    let substring = contents[1];
    // create a vector to store the results
    let mut results = Vec::new();
    // iterate over the sequence
    for (i, base) in sequence.chars().enumerate() {
        // if the base is equal to the first base of the substring
        if base == substring.chars().nth(0).unwrap() {
            // get the substring of the sequence
            // but the subsequence should not exceed the length of the sequence
            let subsequence = &sequence[i..std::cmp::min(i + substring.len(), sequence.len())];
            // if the substring is equal to the substring
            if subsequence == substring {
                // add the index to the vector
                results.push(i + 1);
            }
        }
    }
    // print the results
    for i in results {
        print!("{} ", i);
    }
    print!("\n");
}
