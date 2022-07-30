pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// a size of 3 should draw this
/// +---+---+---+
/// |   |   |   |
/// +---+---+---+
/// |   |   |   |
/// +---+---+---+
/// |   |   |   |
/// +---+---+---+

pub fn draw_board(size: usize) -> Result<(), &'static str>{
    match size {
        size if size > 0 => {
            // content box
            // vert
            draw_index(size);
            for row in 0..size {
                // hor
                // border
                draw_corner(size);
                // content
                if row < 10 {
                    print!("{}  ", row);
                } else {
                    print!("{} ", row);
                }
                draw_middle(size);
            }
            // border
            draw_corner(size);
            draw_index(size);
            Ok(())
        },
        size if size > usize::MAX => Err("oh hell nah"),
        _ => Err("oh hell nah")
    }
}
fn draw_corner(size: usize) {
    let edge_x = String::from("---");
    let corner = String::from("+");
    print!("   ");
    for _col in 0..size {
        print!("{}", corner);
        print!("{}", edge_x);
    }
    print!("{}", corner);
    println!();
}
fn draw_middle(size: usize) {
    let edge_y = String::from("|");
    let content = String::from("   ");
    for _col in 0..size {
        print!("{}", edge_y);
        print!("{}", content);
    }
    print!("{}", edge_y);
    println!();
}
fn draw_index(size: usize) {
    print!("   ");
    for row in 0..size {
        if row < 10 {
            print!("  {} ", row);
        } else {
            print!(" {} ", row);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw() {
        draw_board(6).expect("can't draw the board");
    }
}
