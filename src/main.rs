use algorithm_dictionary::parse_formula;
use algorithm_dictionary::count_isomers;

mod algorithm_dictionary;
fn main() {
    let formula = "C8H32C4fea938c732jc643j48d93";
    let elements = parse_formula(formula).unwrap();
    let isomer_count = count_isomers(&elements).unwrap();

    println!("{} has {} isomers.", formula, isomer_count);
}