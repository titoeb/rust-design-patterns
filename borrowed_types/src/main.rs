use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn is_vowel(candidate: &char) -> bool {
    match candidate {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn three_vowels(word: &str) -> bool {
    let vowel_count = word
        .chars()
        .fold_while(0, |acc, x| {
            let acc_local = acc + is_vowel(&x) as usize;
            if acc >= 3 {
                Done(acc_local)
            } else {
                Continue(acc_local)
            }
        })
        .into_inner();
    if vowel_count >= 3 {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", three_vowels(&"aaa".to_string()));
    println!("{}", three_vowels(&"abc".to_string()));
}
