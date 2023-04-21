use std::io::Read;

fn main() {
    // open a file called dna.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_dna.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut dna = String::new();
    file.read_to_string(&mut dna)
        .expect("Can't read file contents");
    // count the bases and print the results
    let mut a = 0;
    let mut c = 0;
    let mut g = 0;
    let mut t = 0;
    for base in dna.chars() {
        match base {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            // ignore new line character
            '\n' => (),
            '\r' => (),
            _ => println!("{} is not a valid base", base),
        }
    }
    println!("{} {} {} {}", a, c, g, t);
}
