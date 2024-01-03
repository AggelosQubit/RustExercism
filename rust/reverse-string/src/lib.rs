use unicode_segmentation::UnicodeSegmentation;
//Plateform is probably only running "cargo test"
//instead of "cargo test --features grapheme"
//not sure how i can tell the plateform
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
