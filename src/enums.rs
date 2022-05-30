// Enums are types which have a defined values

enum Movment {
    // Variants
    Up,
    Down,
    Right,
    Left
}

fn move_avatar(m: Movment) {
    // Perform action depending on info
    match m {
        Movment::Up => println!("Avatar moving up"),
        Movment::Down => println!("Avatar moving down"),
        Movment::Right => println!("Avatar moving right"),
        Movment::Left => println!("Avatar moving left"),
    }
}

pub fn run() {
    let avatar1 = Movment::Up;
    let avatar2 = Movment::Down;
    let avatar3 = Movment::Right;
    let avatar4 = Movment::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}