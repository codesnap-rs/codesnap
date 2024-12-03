use rand::{thread_rng, Rng};
use std::io::{stdout, BufWriter};

const WORDS: [&'static str; 8] = [
    "Share the code snapshot with your friends!",
    "Today is a great day to share CodeSnap!",
    "CodeSnap is the best way to share code snippets!",
    "The CodeSnap is different from that one in VSCode extension market!",
    "Rustaceans are the best!",
    "Do you enjoy CodeSnap? Let's star it on GitHub!",
    "Enjoy CodeSnap? also try Silicon!",
    "Enjoy CodeSnap? also try Freeze!",
];

pub fn say() {
    let random_num = thread_rng().gen_range(0..8);
    let random_words = WORDS[random_num];
    let writer = BufWriter::new(stdout());
    let _ = ferris_says::say(random_words, random_words.len(), writer);
}
