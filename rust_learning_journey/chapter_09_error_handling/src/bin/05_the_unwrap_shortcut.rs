//05_the_unwrap_shortcut
fn main() {
    let dirty_num_string = "xyz";
    let num: i32 = dirty_num_string.trim().parse::<i32>().unwrap(); /*
                                                                    thread 'main' (7352) panicked at src\bin\05_the_unwrap_shortcut.rs:5:56:
                                                                    called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
                                                                    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
                                                                    error: process didn't exit successfully: `chapter_09_error_handling\target\debug\05_the_unwrap_shortcut.exe` (exit code: 101) */
    println!("Number is ={num}");
}
