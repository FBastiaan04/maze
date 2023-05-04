use std::collections::{HashMap, VecDeque};
use std::io::stdout;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{SetForegroundColor, ResetColor, Color};
use crossterm::terminal::ClearType;
use crossterm::{event, terminal, execute, cursor};
use rand::Rng;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        execute!(stdout(), ResetColor).expect("Could not reset color");
        execute!(stdout(), SetCursorStyle::DefaultUserShape).expect("Could not reset cursor shape");
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn print_screen(screen: &HashMap<i32, u8>, symbols: [char; 16], width: i32, height: i32) {
    for y in 0..height {
        print!("{}\r\n", (0..width).map(|x| symbols[*screen.get(&(y * width + x)).unwrap_or(&0) as usize]).collect::<String>());
    }
}
fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    let mut args: VecDeque<String> = std::env::args().collect();
    let mut rng = rand::thread_rng();
    args.pop_front();
    let width = args.pop_front().unwrap_or("10".to_string()).parse().unwrap_or(10);
    let height = args.pop_front().unwrap_or("10".to_string()).parse().unwrap_or(10);
    let area = width * height;
    // Wilson's algorithm
    let symbols = [
        ' ', // 0000
        '║', // 0001
        '╚', // 0010
        '║', // 0011
        '╚', // 0100
        '╔', // 0101
        '╚', // 0110
        '╠', // 0111
        '╝', // 1000
        '╗', // 1001
        '╝', // 1010
        '╣', // 1011
        '═', // 1100
        '╦', // 1101
        '╩', // 1110
        '╬', // 1111
    ];
    let mut screen: HashMap<i32, u8> = HashMap::new();
    if let Some(code) = args.pop_front() {
        let alphabet = HashMap::from([
            ('0', 0),
            ('1', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('a', 10),
            ('b', 11),
            ('c', 12),
            ('d', 13),
            ('e', 14),
            ('f', 15),
        ]);
        code.chars().enumerate().for_each(|(i, c)| {screen.insert(i as i32, *(alphabet.get(&c)).unwrap());});
    } else {
        let options = [-1, 1, -width, width];
        let first_point = rng.gen_range(0..area);
        screen.insert(first_point, 0);
        while screen.len() < area.try_into().unwrap() {
            let mut path: Vec<i32> = vec![];
            let mut last_dir = None;
            let spots_left: Vec<_> = (0..area).filter(|x| !screen.contains_key(x)).collect();
            let mut next = spots_left[rng.gen_range(0..(spots_left.len()))]; // pick a random starting point not in the maze
            while !screen.contains_key(&next) {
                let mut options_left = vec![];
                let last_dir_or_zero = last_dir.unwrap_or(0);
                if next % width > 0 && -last_dir_or_zero != options[0] {
                    options_left.push(options[0])
                }
                if next % width < width - 1 && -last_dir_or_zero != options[1] {
                    options_left.push(options[1])
                }
                if next >= width && -last_dir_or_zero != options[2] {
                    options_left.push(options[2])
                }
                if next < area - width && -last_dir_or_zero != options[3] {
                    options_left.push(options[3])
                }
                let dir = options_left[rng.gen_range(0..(options_left.len()))];
                screen.insert(next.clone(), 
                    ((dir == options[0] || -last_dir_or_zero == options[0]) as u8) << 3 |
                    ((dir == options[1] || -last_dir_or_zero == options[1]) as u8) << 2 |
                    ((dir == options[2] || -last_dir_or_zero == options[2]) as u8) << 1 |
                    ((dir == options[3] || -last_dir_or_zero == options[3]) as u8) << 0
                );
                path.push(next);
                next += dir; // take a step in the direction
                if let Some(index) = path.iter().position(|&x| x == next) { // This removes any loops
                    for step in path[index..].iter() {
                        screen.remove(step);
                    }
                    path.truncate(index + 1);
                    if path.len() > 1 {
                        last_dir = Some(next - path[path.len() - 2]);
                    } else {
                        last_dir = None;
                    }
                } else {
                    last_dir = Some(dir);
                }
            }
            let current_symbol = screen.get(&next).unwrap();
            if *current_symbol != 0b1111 {
                screen.insert(next, *current_symbol | ( 1 << 3 - options.iter().position(|&x| x == -last_dir.unwrap()).unwrap()));
            }
        }
    }
    // print_screen(&screen, symbols, width, height);
    // print_screen(&screen, symbols_2, width, height);
    // Play the maze
    let mut pos = 0;
    
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::SetCursorStyle::SteadyBlock)?;
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    print_screen(&screen, symbols, width, height);
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    /* add the following */
    while pos != area - 1 {
        if let Event::Key(event) = event::read()? {
            execute!(stdout(), SetForegroundColor(Color::DarkMagenta))?;
            print!("{}", symbols[*screen.get(&pos).unwrap() as usize]);
            match event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: event::KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    .. 
                } => break,
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: event::KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    ..
                } => {
                    if pos % width > 0 && screen.get(&pos).unwrap() & 0b1000 != 0 {
                        pos -= 1;
                    }
                },
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: event::KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    ..
                } => {
                    if pos % width < width - 1 && screen.get(&pos).unwrap() & 0b0100 != 0 {
                        pos += 1;
                    }
                },
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: event::KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    ..
                } => {
                    if pos >= width && screen.get(&pos).unwrap() & 0b0010 != 0 {
                        pos -= width;
                    }
                },
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    ..
                } => {
                    if pos < area - width && screen.get(&pos).unwrap() & 0b0001 != 0 {
                        pos += width;
                    }
                },
                _ => {
                    //todo
                }
            }
            execute!(stdout(), cursor::MoveTo((pos % width).try_into().unwrap(), (pos / width).try_into().unwrap()))?
        };
    }
    if pos == area - 1 {
        execute!(stdout(), SetForegroundColor(Color::DarkMagenta))?;
        print!("{}", symbols[*screen.get(&pos).unwrap() as usize]);
        execute!(stdout(), cursor::MoveTo(0, (height + 1).try_into().unwrap()))?;
        println!("Congrats!");
        let alphabet = "0123456789abcdef".as_bytes();
        println!("{}", (0..area).map(|i| alphabet[*screen.get(&i).unwrap() as usize] as char).collect::<String>());
    }
    Ok(())
}
