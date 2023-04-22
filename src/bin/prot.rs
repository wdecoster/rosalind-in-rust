use std::io::Read;

fn main() {
    // open a file called rosalind_prot.txt
    let mut file = std::fs::File::open("/home/wdecoster/rosalind-input/rosalind_prot.txt")
        .expect("Can't open file");
    // read the file contents into a string
    let mut rna = String::new();
    file.read_to_string(&mut rna)
        .expect("Can't read file contents");
    // translate the rna string into a protein string
    let mut protein = String::new();
    for codon in rna.as_bytes().chunks(3) {
        let codon = std::str::from_utf8(codon).unwrap();
        match codon {
            "UUU" | "UUC" => protein.push('F'),
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => protein.push('L'),
            "AUU" | "AUC" | "AUA" => protein.push('I'),
            "AUG" => protein.push('M'),
            "GUU" | "GUC" | "GUA" | "GUG" => protein.push('V'),
            "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => protein.push('S'),
            "CCU" | "CCC" | "CCA" | "CCG" => protein.push('P'),
            "ACU" | "ACC" | "ACA" | "ACG" => protein.push('T'),
            "GCU" | "GCC" | "GCA" | "GCG" => protein.push('A'),
            "UAU" | "UAC" => protein.push('Y'),
            "UAA" | "UAG" | "UGA" => break,
            "CAU" | "CAC" => protein.push('H'),
            "CAA" | "CAG" => protein.push('Q'),
            "AAU" | "AAC" => protein.push('N'),
            "AAA" | "AAG" => protein.push('K'),
            "GAU" | "GAC" => protein.push('D'),
            "GAA" | "GAG" => protein.push('E'),
            "UGU" | "UGC" => protein.push('C'),
            "UGG" => protein.push('W'),
            "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => protein.push('R'),
            "GGU" | "GGC" | "GGA" | "GGG" => protein.push('G'),
            _ => println!("{} is not a valid codon", codon),
        }
    }
    println!("{}", protein);
}
