use regex::Regex;

fn main() {
    let re = Regex::new("picture").expect("Failed to get new Regex");

    let quote = "\
Every face, every shop,
bedroom window, public-house and
dark square is a picture
fiverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    quote.lines().for_each(|line| {
        let contains_substring = re.find(line);
        if contains_substring.is_some() {
            println!("{}", line)
        }
    });
}
