use game::{*};

fn main() {
    let mut actual_player: bool = false;
    let mut tab: [Field; 9] = [Field::FREE; 9];
    println!("Witaj w grze \"Tik Tak Toe\"!\n");
    'game: loop {
        if let Some(result) = check_winner(&tab) {
            match result {
                GameResult::WIN(player) => println!("[INFO] - Wygrywa gracz {}!", player.as_u8()),
                GameResult::DRAW => println!(),
            }
            break 'game;
        }
        display_tab(&tab);
        println!("Wybór gracza {}", if !actual_player { '1' } else { '2' });
        let mut buffer: String = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buffer) {
            panic!("{}", error)
        }
        if let Ok(number) = buffer.trim().parse::<usize>() {
            if number > 0 && number < 10 {
                if tab[number - 1] == Field::FREE {
                    tab[number - 1] = Field::OCCUPIED(if !actual_player { Player::FIRST } else { Player::SECOND });
                    actual_player = !actual_player
                }
                else {
                    println!("[!] - Pole {} jest już zajęte przez gracza {}!", number, if !actual_player { Player::FIRST as u8 as char } else { Player::SECOND as u8 as char })
                }
            }
            else {
                println!("[!] - Nieprawidłowy numer!")
            }
        }
        else {
            println!("[!] - Nieprawidłowy numer!")
        }
        //break 'game;
    }
}

mod game {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Field {
        OCCUPIED(Player),
        FREE,
    }
    #[derive(Clone, Copy, PartialEq)]
    pub enum Player {
        FIRST = 'X' as isize,
        SECOND = 'O' as isize,
    }
    impl Player {
        pub fn get_player_from_u8(number: u8) -> Option<Self> {
            match number {
                1 => Some(Self::FIRST),
                2 => Some(Self::SECOND),
                _ => None,
            }
        }
        pub fn as_u8(&self) -> u8 {
            match *self {
                Self::FIRST => 1,
                Self::SECOND => 2,
            }
        }
    }
    pub enum GameResult {
        WIN(Player),
        DRAW,
    }
    pub fn display_tab(tab: &[Field; 9]) {
        for x in 1..tab.len() + 1 {
            if x % 3 != 0 {
                print!("{} ", match tab[x - 1] {
                    Field::OCCUPIED(player) => player as u8 as char,
                    Field::FREE => format!("{}", x).chars().next().unwrap(),
                });
            }
            else {
                print!("{}\n", match tab[x - 1] {
                    Field::OCCUPIED(player) => player as u8 as char,
                    Field::FREE => format!("{}", x).chars().next().unwrap(),
                });
            }
        }
    }
    pub fn check_winner(tab: &[Field; 9]) -> Option<GameResult> {
        for x in 1..=2 {
            let mut player: Player = Player::FIRST;
            if let Some(loop_player) = Player::get_player_from_u8(x) {
                player = loop_player;
            }
            if tab[0] == Field::OCCUPIED(player) && tab[1] == Field::OCCUPIED(player) && tab[2] == Field::OCCUPIED(player) ||
            tab[3] == Field::OCCUPIED(player) && tab[4] == Field::OCCUPIED(player) && tab[5] == Field::OCCUPIED(player) ||
            tab[6] == Field::OCCUPIED(player) && tab[7] == Field::OCCUPIED(player) && tab[8] == Field::OCCUPIED(player) ||
            tab[0] == Field::OCCUPIED(player) && tab[3] == Field::OCCUPIED(player) && tab[6] == Field::OCCUPIED(player) ||
            tab[1] == Field::OCCUPIED(player) && tab[4] == Field::OCCUPIED(player) && tab[7] == Field::OCCUPIED(player) ||
            tab[2] == Field::OCCUPIED(player) && tab[5] == Field::OCCUPIED(player) && tab[8] == Field::OCCUPIED(player) ||
            tab[0] == Field::OCCUPIED(player) && tab[4] == Field::OCCUPIED(player) && tab[8] == Field::OCCUPIED(player) ||
            tab[2] == Field::OCCUPIED(player) && tab[4] == Field::OCCUPIED(player) && tab[6] == Field::OCCUPIED(player) {
                return Some(GameResult::WIN(player))
            }
        }
        let mut draw: u8 = 0;
        for x in 0..tab.len() {
            if tab[x] != Field::FREE {
                draw += 1
            }
        }
        if draw as usize > tab.len() - 1 {
            return Some(GameResult::DRAW)
        }
        None
    }
}