//02_pig_latin_convertor
use std::io::Write;
fn main() {
    print!("Enter word: ");
    std::io::stdout().flush().expect("Flushing error!");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).expect("Input error!");
    let word = word.trim();
    let first_letter = word.chars().next().expect("Error!");
    if first_letter == 'a'
        || first_letter == 'e'
        || first_letter == 'i'
        || first_letter == 'o'
        || first_letter == 'u'
    {
        let pig_latin = format!("{}-hey", word);
        println!("Pig latin={}", pig_latin);
    } else {
        let pig_latin = format!(
            "{}-{}ay",
            word.chars().skip(1).collect::<String>(),
            first_letter
        );
        println!("Pig latin={}", pig_latin);
    }
}
