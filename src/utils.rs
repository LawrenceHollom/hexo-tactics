use std::time::Duration;

pub fn split_list(text: &str) -> Vec<&str> {
    let mut depth = 0;
    let mut last_index = 0;
    let mut args = vec![];
    for (i, c) in text.as_bytes().iter().enumerate() {
        if *c == '(' as u8 {
            depth += 1;
        } else if *c == ')' as u8 {
            depth -= 1;
            if depth < 0 {
                panic!("The parens depth is < 0!")
            }
        } else if *c == ',' as u8 && depth == 0 {
            args.push(&text[last_index..i]);
            last_index = i + 1;
        }
    }
    args.push(&text[last_index..text.len()]);
    args
}

/**
 * e.g. given "func(arg, param, g(a, b))", will return
 * ("func", ["arg", " param", " g(a, b)"]).
 */
pub fn parse_function_like(text: &str) -> (&str, Vec<&str>) {
    match text.split_once('(') {
        Some((func, args_string)) => {
            match args_string.rsplit_once(')') {
                Some((args_string, _other)) => (func, split_list(args_string)),
                None => (text, vec![]),
            }
        }
        None => (text, vec![]),
    }
}

pub fn pretty_format_time(dur: Duration) -> String {
    let nanos = dur.as_nanos();
    let mics = nanos / 1000;
    let millis = mics / 1000;
    let secs = millis / 1000;
    if nanos < 1000 {
        format!("{} ns", nanos)
    } else if mics < 10 {
        format!("{}.{:0>2} μs", mics, (nanos % 1000) / 10)
    } else if mics < 100 {
        format!("{}.{} μs", mics, (nanos % 1000) / 100)
    } else if mics < 1000 {
        format!("{} μs", mics)
    } else if millis < 10 {
        format!("{}.{:0>2} ms", millis, (mics % 1000) / 10)
    } else if millis < 100 {
        format!("{}.{} ms", millis, (mics % 1000) / 100)
    } else if millis < 1000 {
        format!("{} ms", millis)
    } else {
        format!("{}.{:0>2} s", secs, (millis % 1000) / 10)
    } 
}