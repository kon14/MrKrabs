extern crate rand;

use rand::Rng;

const QUOTES: [&str; 8] = [
  "Well, the way I see it, there are three possibilities: One, you stole it; two, you stole it; or three, you stole it!",
  "A 5 letter word for happiness: MONEY.",
  "If you're wasting time, you're wasting money... and that's just sick.",
  "I can think of ten good reasons to never let go of a dime, boy.",
  "A man works hard all week to keep his pants off all weekend.",
  "Get back to work all of you! I'm not running a happy factory here.",
  "If I don't make any money today I'll surely break out in a rash!",
  "Money is the ultimate source of joy.",
];

pub fn spit_facts() {
  println!("{} -Mr.Krabs\n", QUOTES[rand::thread_rng().gen_range(0..QUOTES.len().into())]);
}
