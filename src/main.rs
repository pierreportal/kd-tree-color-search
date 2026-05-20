mod nearest;
mod shared;
mod tree;
use nearest::get_color_name_match;
use shared::*;
use std::io;
use tree::KdTree;

fn main() -> io::Result<()> {
    let tree = KdTree::new().load_tree("colors.kdtree");
    let color = Rgb {
        r: 251,
        g: 84,
        b: 43,
    };

    let best = get_color_name_match(&color, tree);
    println!("{}", best);
    Ok(())
}
