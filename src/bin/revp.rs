use std::io::Read;

fn main() {
    // open a file called rosalind_revp.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_revp.txt")
        .expect("Can't open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Can't read file");
    // skip the first line and remove all newlines
    buf = buf.lines().skip(1).collect::<String>().replace('\n', "");
    // find every reverse palindrome in the sequence
    for i in 0..buf.len() {
        for j in 4..13 {
            if i + j > buf.len() {
                break;
            }
            let sub = &buf[i..i + j];
            if sub == rev_comp(sub) {
                println!("{} {}", i + 1, j);
            }
        }
    }
}

fn rev_comp(s: &str) -> String {
    s.chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Invalid character"),
        })
        .collect()
}
