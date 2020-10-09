use ansi_term::Color;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A simple tool that creates swatches from terminal colors")]
struct Opt {
    /// Print simple, one-line swatches
    #[structopt(short, long)]
    simple: bool,

    /// Print each swatch on a new line
    #[structopt(short, long)]
    newline: bool,
}

fn create_swatch(length: usize) -> String {
    const SINGLE_SWATCH: &'static str = "█";
    SINGLE_SWATCH.repeat(length)
}
fn get_colors() -> Vec<Color> {
    vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Purple,
        Color::Cyan,
        Color::White,
    ]
}

fn print_layered_swatches(newline: bool) {
    let total = 4;
    let colors = get_colors();

    let print_color_row = |color: &Color, current: usize| {
        print!(
            "{}{}{}",
            if current == total {
                format!("  {}", color.bold().paint(create_swatch(total)))
            } else {
                color.paint(create_swatch(total + 2)).to_string()
            },
            if current == 1 {
                String::from("  ")
            } else {
                color.bold().paint(create_swatch(2)).to_string()
            },
            "  ",
        );
    };

    if newline {
        for color in &colors {
            for current in 1..=total {
                print_color_row(&color, current);
                print!("\n");
            }
            print!("\n");
        }
    } else {
        for current in 1..=total {
            for color in &colors {
                print_color_row(&color, current);
            }
            if current < total {
                print!("\n");
            }
        }
    }
}

fn print_simple_swatches(newline: bool) {
    let colors = get_colors();
    for color in colors {
        print!(
            "{}{}{}",
            color.paint(create_swatch(6)),
            color.bold().paint(create_swatch(2)),
            if newline { "\n\n" } else { "  " }
        );
    }
    // if !newline {
    //     print!("\n");
    // }
}

fn main() {
    let opt = Opt::from_args();
    if opt.simple {
        print_simple_swatches(opt.newline);
    } else {
        print_layered_swatches(opt.newline);
    }
}
