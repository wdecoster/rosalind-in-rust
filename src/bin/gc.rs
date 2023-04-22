use std::io::Read;

fn main() {
    // open a file called rosalind_gc.fasta
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_gc.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut fasta = String::new();
    file.read_to_string(&mut fasta)
        .expect("Can't read file contents");
    // split the string into a vector of strings
    let fasta = fasta.split(">").collect::<Vec<&str>>();
    // create a vector to store the results
    let mut results = Vec::new();
    // iterate over the vector
    for entry in fasta {
        // split the entry into a vector of strings
        let entry = entry.split_whitespace().collect::<Vec<&str>>();
        // if the entry is not empty
        if !entry.is_empty() {
            // get the label
            let label = entry[0];
            // get the sequence
            let sequence = entry[1..].join("");
            // count the G's and C's
            let gc = sequence.matches("G").count() + sequence.matches("C").count();
            // calculate the GC content
            let gc_content = (gc as f64 / sequence.len() as f64) * 100.0;
            // add the result to the vector
            results.push((label, gc_content));
        }
    }
    // sort the results by GC content
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // print the first element of the vector
    println!("{}", results[0].0);
    println!("{}", results[0].1);
}
