use crate::js::console;
use crate::ports::models::subjects::Player;

pub fn run(player: &Player) {
    console::log("Wellcome to Dimensions!");
    console::log("Character created!");
    show_character(player);
    console::log("Done!");
}

fn show_character(player: &Player) {
    console::log(&format!("Nickname: {}", player.nickname));
    console::log(&format!("Description: {}", player.description));
    console::log("Attributes:");

    for attr in &player.subject.attrs {
        console::log(&format!(
            "    {} [points={}, absorb={}]",
            attr.title, attr.points, attr.absorb
        ));
        console::log(&format!("        {}", attr.description));
    }
}
