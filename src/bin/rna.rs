use std::io::Read;

fn main() {
    // open a file called rna.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_rna.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut rna = String::new();
    file.read_to_string(&mut rna)
        .expect("Can't read file contents");
    // replace all T's with U's
    let rna = rna.replace("T", "U");
    println!("{}", rna);
}
