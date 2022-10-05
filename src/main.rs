fn main() {
    let mut actual_player: bool = false;
    let mut tab: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    println!("\nWitaj w grze \"Tik Tak Toe\"!\n");
    'game: loop {
        if let Some(winner) = check_winner(&tab) {
            match winner {
                1..=2 => println!("[INFO] - Wygrywa gracz {}!", winner),
                _ => println!("[INFO] - Remis!")
            }
            break 'game;
        }
        display_tab(&tab);
        println!("Wybór gracza {} - ", if actual_player == false { '1' } else { '2' });
        let mut buffer: String = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut buffer) {
            panic!("{error}");
        }
        if let Ok(number) = buffer.trim().parse::<usize>() {
            if number > 0 && number < 10 {
                if tab[number - 1] == 0 {
                    tab[number - 1] = actual_player as u8 + 1;
                    actual_player = !actual_player;
                }
                else {
                    println!("[!] - Pole {} jest juz zajete przez gracza {}!", number, tab[number - 1]);
                }
            }
            else {
                println!("[!] - Nieprawidłowy numer!");
            }
        }
        else {
            println!("[!] - Nieprawidłowy numer!");
        }
    }
}

fn display_tab(tab: &[u8; 9]) {
    println!("\n\n\n\n\n");
    for x in 1..tab.len() + 1 {
        if x % 3 != 0 {
            if tab[x - 1] == 0 {
                print!("{} ", x);
            }
            else {
                match tab[x - 1] {
                    1 => print!("X "),
                    2 => print!("O "),
                    _ => {},
                }
            }
        }
        else {
            if tab[x - 1] == 0 {
                print!("{}\n", x);
            }
            else {
                match tab[x - 1] {
                    1 => print!("X\n"),
                    2 => print!("O\n"),
                    _ => {},
                }
            }
        }
    }
}
fn check_winner(tab: &[u8; 9]) -> Option<u8> {
    for x in 1..=2 {
        if tab[0] == x && tab[1] == x && tab[2] == x ||
        tab[3] == x && tab[4] == x && tab[5] == x ||
        tab[6] == x && tab[7] == x && tab[8] == x ||
        tab[0] == x && tab[3] == x && tab[6] == x ||
        tab[1] == x && tab[4] == x && tab[7] == x ||
        tab[2] == x && tab[5] == x && tab[8] == x ||
        tab[0] == x && tab[4] == x && tab[8] == x ||
        tab[2] == x && tab[4] == x && tab[6] == x {
            return Some(x)
        }
    }
    let mut draw: usize = 0;
    for x in tab {
        if *x != 0 {
            draw += 1;
        }
    }
    if draw == tab.len() {
        return Some(3)
    }
    None
}