//01_ownership_transfer_tracker
fn main() {
    let s1 = String::from("Farary");
    let s2 = s1;
    println!("s2 has car:{s2}");
    //println!("={s1}");if i use this line below error will appear
    /*error[E0382]: borrow of moved value: `s1`
     --> src\bin\01_move_semantics.rs:7:17
      |
    4 |     let s1=String::from("Farary");
      |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    5 |     let s2=s1;
      |            -- value moved here
    6 |     println!("s2 has car:{s2}");
    7 |     println!("={s1}");
      |                 ^^ value borrowed here after move
      |
    help: consider cloning the value if the performance cost is acceptable
      |
    5 |     let s2=s1.clone();
      |              ++++++++

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `chapter_04_understanding_ownership` (bin "01_move_semantics") due to 1 previous error */
}
