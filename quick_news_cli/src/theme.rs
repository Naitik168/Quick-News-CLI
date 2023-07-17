use crossterm::style::Color::*;
use termimad::*;

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r: 28,
        g: 28,
        b: 28,
    });
 
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');

    skin.inline_code.set_fg(Rgb {
        r: 255,
        g: 255,
        b: 0,
    });
    skin.italic.set_fg(Rgb {
        r: 0,
        g: 100,
        b: 255,
    });

    skin
}