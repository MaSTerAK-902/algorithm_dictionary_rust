use algorithm_dictionary::stable_marriage_problem;

mod algorithm_dictionary;
fn main() {
    let men_pref = vec![
        vec![1, 0, 2],
        vec![0, 1, 2],
        vec![1, 0, 2],
    ];

    let women_pref = vec![
        vec![2, 1, 0],
        vec![2, 0, 1],
        vec![1, 2, 0],
    ];

    let result = stable_marriage_problem(&men_pref, &women_pref);
    println!("Stable marriages: {:?}", result);
}