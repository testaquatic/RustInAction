fn main() {
    let search_tem = "picture";
    let quote = "\
Every face, every shop, bedroom windows, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    quote
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(search_tem))
        .for_each(|(line_num, line)| println!("{}: {}", line_num + 1, line));
}
