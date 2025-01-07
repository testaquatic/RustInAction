use lib_renderhex::{
    draw_svg::{convert, generate_svg},
    parse_args::{parse, CmdArgs},
};

fn main() {
    let args = CmdArgs::get();
    let operations = parse(&args.input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(&args.save_to, &document).expect("Failed to save svg file.");
}
