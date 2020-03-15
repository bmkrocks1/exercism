fn bob_phrase(n: u32) -> String {
  match n {
    0 => "no more bottles of beer".to_string(),
    1 => "1 bottle of beer".to_string(),
    _ => format!("{} bottles of beer", n),
  }
}

pub fn verse(n: u32) -> String {
  let line_1 = || -> String {
    let phrase = match n {
      0 => "No more bottles of beer".to_string(),
      _ => bob_phrase(n),
    };

    format!("{} on the wall, {}.\n", phrase, bob_phrase(n))
  };

  let line_2 = || -> String {
    let phrase = match n {
      0 => "Go to the store and buy some more",
      1 => "Take it down and pass it around",
      _ => "Take one down and pass it around",
    };

    let m = if n == 0 { 99 } else { n - 1 };
    format!("{}, {} on the wall.\n", phrase.to_string(), bob_phrase(m))
  };

  [
    line_1(), 
    line_2()
  ]
  .join("")
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(|x| verse(x))
    .collect::<Vec<String>>()
    .join("\n")
}
