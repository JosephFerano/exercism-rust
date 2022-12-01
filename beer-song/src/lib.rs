pub fn verse(n: u32) -> String {
    match n {
        n if n == 0 =>
            format!("No more bottles of beer on the wall, no more bottles of beer.\
                    \nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        n if n == 1 =>
            format!("1 bottle of beer on the wall, 1 bottle of beer.\
                    \nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        n => {
            let m = n - 1;
            let s = if m == 1 { "" } else { "s" };
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\
                    \nTake one down and pass it around, {m} bottle{s} of beer on the wall.\n")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut v: Vec<String> = Vec::new();
    for i in (end..start+1).rev() {
        v.push(verse(i));
    }
    v.join("\n")
}
