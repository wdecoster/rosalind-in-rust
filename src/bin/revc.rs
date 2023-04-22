use std::io::Read;

fn main() {
    // open a file called rosalind_revc.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_revc.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut dna = String::new();
    file.read_to_string(&mut dna)
        .expect("Can't read file contents");
    // reverse the string
    let dna = dna.chars().rev().collect::<String>();
    // replace all bases with their complements
    let dna = dna.replace("A", "t");
    let dna = dna.replace("T", "a");
    let dna = dna.replace("C", "g");
    let dna = dna.replace("G", "c");
    // print the result
    println!("{}", dna.to_uppercase());
}
