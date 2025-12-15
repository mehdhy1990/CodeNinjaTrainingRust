fn dna_strand(dna: &str) -> String {
    // let mut scores:HashMap<char,char> = HashMap::new();
    // scores.insert('A','T');
    // scores.insert('T','A');
    // scores.insert('C','G');
    // scores.insert('G','C');
    // dna.chars().map(|x| {
    //     if(scores.contains_key(&x)) {
    //         return scores[&x];
    //     }
    //    return  x;
    // }).collect::<String>()
    dna.chars()
        .map(|x| match x {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            s => s,
        })
        .collect::<String>()
}
