use crate::scenarios::base::creators::*;
use crate::ports::models::subjects::Player;

use std::io;

fn create_character() -> Result<Player, io::Error> {
    let mut nickname = String::new();
    println!("Inform your nickname: ");
    io::stdin().read_line(&mut nickname)?;

    let mut description = String::new();
    println!("Inform one description (can be empty): ");
    io::stdin().read_line(&mut description)?;

    return Ok(Player::create_player(nickname, description));
}

fn show_character(player: &Player) {
    print!("Nickname: {}", player.nickname);
    print!("Description: {}", player.description);
    println!("Attributes:");

    for attr in &player.subject.attrs {
        println!("    {} [points={}, absorb={}]", attr.title, attr.points, attr.absorb);
        println!("        {}", attr.description);
    }
}

pub fn run() -> Result<(), io::Error> {
    println!("Wellcome to Dimensions!");
    let player = create_character()?;
    println!("Character created! Press any key to show your informations...");

    let mut any = String::new();
    io::stdin().read_line(&mut any)?;

    show_character(&player);
    println!("Done! press any key to quit...");
    io::stdin().read_line(&mut any)?;
    Ok(())
}
