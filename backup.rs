use std::env;
use std::fs;
use std::thread;
use std::cmp;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color;
use rand::prelude::*;

//use console::Term;

const WIDTH: usize = 15;
const HEIGHT: usize = 7;
const NUMB: usize = 5;
static mut MAP: [&str;WIDTH*HEIGHT] = ["x";WIDTH*HEIGHT];
static mut FIELD: [bool; WIDTH*HEIGHT] = [false;WIDTH*HEIGHT];
static mut x_pos: isize = 1;
static mut y_pos: isize = 1;

fn main() {
    unsafe {
       // MAP[22] = "?";
       let bredth = WIDTH*HEIGHT;
       let mut rng = rand::thread_rng();
       for b in 0..NUMB {
           let target: usize = rng.gen::<usize>() % bredth;
           FIELD[target] = true;
       }

    }
    stdout().flush().unwrap();
    let splash = fs::read_to_string("src/splash.txt").expect("File not found");
    println!("{}{}",termion::clear::All, splash);
    //thread::sleep(Duration::from_millis(2000));
    //
    //let term = Term::stdout();
    //println!("Welcome to Walrus Minesweeper!");
    //println!("Bomb defusal takes tusks!");
    //println!("board width: {} board height: {}", WIDTH, HEIGHT);
    //println!("{}",get_board());
    //term.write_line("Welcome to Walrus Minesweeper!");
    //term.write_line("Bomb defusal takes tusks!");
    //term.write_line("board width: {} board height: {}", WIDTH, HEIGHT);
    //term.write_line("{}", get_board());
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap(); // I need to get into raw mode, but it is
                                                        //litrally breaking newlines
    //printing welcoming message, clearing the screen and going to the top left corner
    //write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1,1), termion::clear::All).unwrap();
    stdout.flush().unwrap();

    //detecting keydown events
    'game_loop: for c in stdin.keys() {
        //clearing the screen and going to top left corner
        //write!(
        //    stdout,
        //    "{}{}",
        //    termion::cursor::Goto(1,1),
        //    termion::clear::All
        //    ).unwrap();
        //println!("{}{}",termion::clear::All, splash);
        //write!(stdout, "{}{}{}{}",termion::cursor::Goto(1,1), termion::clear::All, get_board(), "f - flag\n g - dig\n h,j,k,l - left, up, down, right\n").unwrap();
        stdout.flush().unwrap();
        //writeln!(stdout, "{}", get_board());
        // i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('h') => println!("Hello world"),
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
            Key::Char('q') => { quit_game(); break},
            Key::Char('f') => {flag_map(); println!("Flag!")},
            Key::Char('g') => {dig_map(); println!("Dig!")},
            Key::Char('h') => {move_player(0); println!("Left!")},
            Key::Char('j') => {move_player(1); println!("Down!")},
            Key::Char('k') => {move_player(2); println!("Up!")},
            Key::Char('l') => {move_player(3); println!("Right!")},
            _ => (),
        }
        print_board();
        stdout.flush().unwrap();
    }

    //term.write_line("Hello World");
    //thread::sleep(Duration::from_millis(2000));
    //term.clear_line();
    //

}

fn quit_game() {
    //break 'game_loop;
}

fn flag_map() {
    // flag the x_pos y_pos
    unsafe {
        MAP[<isize as TryInto<usize>>::try_into(y_pos).unwrap()*WIDTH +
            <isize as TryInto<usize>>::try_into(x_pos).unwrap()] = "F";
    }
    
}

fn dig_map() {
    // dig the x_pos y_pos

    unsafe {
        let mut count: isize = check_bombs(x_pos, y_pos);
        println!("{}", count.to_string());
        let mut tmp: String = count.to_string();
        let mut temp: &str = tmp.as_str();
        println!("{}", temp.to_string());
        //let mut temp: &str = count.to_string().clone().as_str();
        if -1 == count {
            MAP[<isize as TryInto<usize>>::try_into(y_pos).unwrap()*WIDTH +
                <isize as TryInto<usize>>::try_into(x_pos).unwrap()] = "*";
        }
        else {
            MAP[<isize as TryInto<usize>>::try_into(y_pos).unwrap()*WIDTH +
                <isize as TryInto<usize>>::try_into(x_pos).unwrap()] = "3";// temp;
        }
        println!("{}",temp.to_string());
    }
}

fn check_bombs(x: isize, y: isize) -> isize{
    unsafe {
        let xu = <isize as TryInto<usize>>::try_into(x).unwrap();
        let yu = <isize as TryInto<usize>>::try_into(y).unwrap(); 
        if FIELD[yu * WIDTH + xu] {
            return -1;
        }
        else {
            let mut counter = 0;
            for i in cmp::max(0,x_pos-1)..cmp::min(x_pos+1,<usize as TryInto<isize>>::try_into(WIDTH).unwrap()-1) {
                for j in cmp::max(0,y_pos-1)..cmp::min(y_pos+1,<usize as TryInto<isize>>::try_into(HEIGHT).unwrap()-1) {
                    if FIELD[<isize as TryInto<usize>>::try_into(j).unwrap()*WIDTH + <isize as TryInto<usize>>::try_into(i).unwrap()]
                    {
                        counter += 1;
                    }
                }
            }
            return counter;
        }
        return 0;
    }
}
fn move_player(dir: u8) {
    
    match dir {
        0 => unsafe { x_pos = x_pos -1},
        1 => unsafe { y_pos = y_pos +1},
        2 => unsafe { y_pos = y_pos -1},
        3 => unsafe { x_pos = x_pos +1},
        _ => (),
    }
    // unsafe due to accessing a globale static mutable
    unsafe {x_pos = cmp::max(x_pos, 0);
    y_pos = cmp::max(y_pos, 0);
    x_pos = cmp::min((WIDTH-1).try_into().unwrap(), x_pos);
    y_pos = cmp::min((HEIGHT-1).try_into().unwrap(), y_pos);}
    unsafe {print!("{}, {}", x_pos, y_pos)};
}
fn get_board() -> String {
    let mut board: String = "".to_string();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            unsafe {
                board = format!("{}{}",board,  MAP[i*WIDTH + j].to_string());
            }
        }
        board = board + "\n";
    }
    return board;
}

fn print_board() {
    //println!("hello"); // println ends with a newline which causes the cursor to go down
    write!(stdout(), "{}", termion::cursor::Goto(1,1));
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            write!(stdout(),"{}", color::Bg(color::Reset));
            unsafe {
                if (i == y_pos.try_into().unwrap() && j == x_pos.try_into().unwrap()) 
                {
                    let bg = color::Bg(color::Red);
                    write!(stdout(), "{}@",bg);
                }
                else
                {
                    write!(stdout(), "{}", MAP[i*WIDTH + j].to_string());

                }
            }
        }
        write!(stdout(), "{}{}", termion::cursor::Left(WIDTH.try_into().unwrap()), termion::cursor::Down(1));
    }

}
