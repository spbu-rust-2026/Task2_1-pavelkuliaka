fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    let start = _text.len() - _text.trim_start().len();
    let end = start + _text.split_whitespace().next().map_or(0, str::len);
    _text.push('!');
    &_text[start..end]
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
