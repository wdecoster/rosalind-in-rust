fn main() {
    let length = 5;
    let mut permutations = (1..=length).collect::<Vec<usize>>();
    let mut indices = vec![0; length];
    let mut i = 0;
    let mut number_of_permutations = 1;
    let mut combinations = vec![permutations
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")];
    while i < length {
        if indices[i] < i {
            if i % 2 == 0 {
                permutations.swap(0, i);
            } else {
                permutations.swap(indices[i], i);
            }
            combinations.push(
                permutations
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            number_of_permutations += 1;
            indices[i] += 1;
            i = 0;
        } else {
            indices[i] = 0;
            i += 1;
        }
    }
    println!("{}", number_of_permutations);
    println!("{}", combinations.join("\n"));
}
