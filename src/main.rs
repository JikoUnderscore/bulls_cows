#![allow(non_camel_case_types)]

use std::num::NonZeroU32;

const SIZE: usize = 6561; // 9^4 (for all digit combinations with no repeats

#[derive(Debug, Clone, Copy)]
pub struct GuessCode32(NonZeroU32);
const _: () = assert!(size_of::<Option<GuessCode32>>() == size_of::<u32>());

impl GuessCode32 {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Option<Self> {
        let val = u32::from_be_bytes([a, b, c, d]);
        return match NonZeroU32::new(val) {
            Some(x) => Some(Self(x)),
            None => None,
        };
    }

    pub fn new_none_zero(a: u8, b: u8, c: u8, d: u8) -> Self {
        let val = u32::from_be_bytes([a, b, c, d]);
        return GuessCode32(NonZeroU32::new(val).expect("to be non zero"));
    }

    pub fn get(self) -> [u8; 4] {
        self.0.get().to_be_bytes()
    }
}

fn choose_number(guesses: &[Option<GuessCode32>]) -> GuessCode32 {
    let mut best_guess = *guesses.iter().flatten().next().unwrap();
    let mut score = u32::MAX;

    for r in guesses.iter().flatten() {
        let mut distribution = [0u32; 15];
        for l in guesses.iter().flatten() {
            let bull_cow = play_bull_cows(*r, *l);
            match bull_cow {
                00 => distribution[0] += 1,
                01 => distribution[1] += 1,
                02 => distribution[2] += 1,
                03 => distribution[3] += 1,
                04 => distribution[4] += 1,
                10 => distribution[5] += 1,
                11 => distribution[6] += 1,
                12 => distribution[7] += 1,
                13 => distribution[8] += 1,
                20 => distribution[9] += 1,
                21 => distribution[10] += 1,
                22 => distribution[11] += 1,
                31 => distribution[12] += 1,
                30 => distribution[13] += 1,
                40 => distribution[14] += 1,
                _ => unsafe {
                    std::hint::unreachable_unchecked();
                },
            }
        }

        let worst_case = *distribution.iter().max().unwrap();
        if worst_case < score {
            score = worst_case;
            best_guess = *r;
        }
    }
    println!("[choose_number] Picked {:?} with worst-case score {}", best_guess.get(), score);
    return best_guess;
}

// https://www.youtube.com/watch?v=FR_71HyBytE&t=846s
fn main() {
    let secret = match handle_cla() {
        Ok(e) => e,
        Err(_) => {
            GuessCode32::new_none_zero(5, 8, 7, 6) /* return; */
        }
    };

    static mut ARRAY: [Option<GuessCode32>; SIZE] = make_array();
    let guesses = unsafe { &mut (*(&raw mut ARRAY)) };

    let mut answare_index = 0;
    let mut chosen_number;
    let mut luup = 1;

    let mut run = true;
    while run {
        println!("---------------------");
        chosen_number = choose_number(guesses);
        println!("guess № {}", luup);
        compare_numbers(&mut run, chosen_number, secret, guesses);
        answare_index = found_answare(&mut run, guesses);

        luup += 1;
    }
    println!("---------------------");
    println!("chosen_number {:?}", secret.get());
    println!("guess № {:?}", luup);
    println!("answare {:?}", guesses[answare_index].unwrap().get());
}

const fn make_array() -> [Option<GuessCode32>; SIZE] {
    const DIGITS: u8 = 9;
    let mut arr: [Option<GuessCode32>; SIZE] = [const { None }; SIZE];
    let mut n = 0;

    let mut h = 1;
    let mut s;
    let mut d;
    let mut e;
    while h <= DIGITS {
        s = 1;
        while s <= DIGITS {
            d = 1;
            while d <= DIGITS {
                e = 1;
                while e <= DIGITS {
                    arr[n] = GuessCode32::new(h, s, d, e);
                    n += 1;

                    e += 1;
                }
                d += 1;
            }
            s += 1;
        }
        h += 1;
    }

    return arr;
}

fn found_answare(run: &mut bool, guesses: &[Option<GuessCode32>]) -> usize {
    let mut n = 0;
    let mut index = 0;
    for (i, guess) in guesses.iter().enumerate() {
        if guess.is_some() {
            n += 1;
            index = i;
            if n > 1 {
                return i;
            }
        }
    }

    if n == 1 {
        *run = false;
    }
    return index;
}

#[rustfmt::skip]
fn compare_numbers(run: &mut bool, guess: GuessCode32, secret: GuessCode32, guesses: &mut [Option<GuessCode32>]) {
    let mut witch = [true; 15];

    let bull_cow = play_bull_cows(guess, secret);
    match bull_cow {
        00 => witch[0] = false,
        01 => witch[1] = false,
        02 => witch[2] = false,
        03 => witch[3] = false,
        04 => witch[4] = false,
        10 => witch[5] = false,
        11 => witch[6] = false,
        12 => witch[7] = false,
        13 => witch[8] = false,
        20 => witch[9] = false,
        21 => witch[10] = false,
        22 => witch[11] = false,
        31 => witch[12] = false,
        30 => witch[13] = false,
        40 => { witch[14] = false; *run = false; }
        _ => unsafe{
                    std::hint::unreachable_unchecked();
                }
    }
    println!("{:02}", bull_cow,);


    for og in guesses.iter_mut() {
        if let Some(g) = og {
            let bull_cow = play_bull_cows(guess, *g);

            match bull_cow {
                00 => if witch[0]  {*og = None},
                01 => if witch[1]  {*og = None},
                02 => if witch[2]  {*og = None},
                03 => if witch[3]  {*og = None},
                04 => if witch[4]  {*og = None},
                10 => if witch[5]  {*og = None},
                11 => if witch[6]  {*og = None},
                12 => if witch[7]  {*og = None},
                13 => if witch[8]  {*og = None},
                20 => if witch[9]  {*og = None},
                21 => if witch[10] {*og = None},
                22 => if witch[11] {*og = None},
                31 => if witch[12] {*og = None},
                30 => if witch[13] {*og = None},
                40 => if witch[14] {*og = None}, 
                _ => unsafe{
                    std::hint::unreachable_unchecked();
                }
            }
        }
    }
}

fn play_bull_cows(guess: GuessCode32, secret: GuessCode32) -> u8 {
    let guess = guess.get();
    let secret = secret.get();

    let mut bulls = 0_u8;
    let mut cows = 0;

    if guess[0] == secret[0] {
        bulls += 1;
    }
    if guess[1] == secret[1] {
        bulls += 1;
    }
    if guess[2] == secret[2] {
        bulls += 1;
    }
    if guess[3] == secret[3] {
        bulls += 1;
    }

    if guess[0] != secret[0] && (guess[0] == secret[1] || guess[0] == secret[2] || guess[0] == secret[3]) {
        cows += 1;
    }
    if guess[1] != secret[1] && (guess[1] == secret[0] || guess[1] == secret[2] || guess[1] == secret[3]) {
        cows += 1;
    }
    if guess[2] != secret[2] && (guess[2] == secret[1] || guess[2] == secret[0] || guess[2] == secret[3]) {
        cows += 1;
    }
    if guess[3] != secret[3] && (guess[3] == secret[0] || guess[3] == secret[1] || guess[3] == secret[2]) {
        cows += 1;
    }

    return bulls * 10 + cows;
}

fn handle_cla() -> Result<GuessCode32, ()> {
    let args: Vec<String> = std::env::args().collect();

    if not!(args.len() >= 2) {
        eprintln!("Enter a foure digit number as a commadn line argumen");
        return Err(());
    }

    if not!(args[1].len() == 4) {
        eprintln!("Number should be 4 char long!");
        return Err(());
    }

    let ss = args[1].as_bytes();

    let h = (ss[0] as char).to_digit(10).expect("to be a numebr");
    let s = (ss[1] as char).to_digit(10).expect("to be a numebr");
    let d = (ss[2] as char).to_digit(10).expect("to be a numebr");
    let e = (ss[3] as char).to_digit(10).expect("to be a numebr");

    if not! {h > 0 || s > 0 || d > 0 || e > 0} {
        eprintln!("Numbers should be more than zero");
        return Err(());
    }
    if not!(h <= 9 || s <= 9 || d <= 9 || e <= 9) {
        eprintln!("Numbers should be less than nine");
        return Err(());
    }
    return Ok(GuessCode32::new_none_zero(h as u8, s as u8, d as u8, e as u8));
}

#[macro_export]
macro_rules! not {
    ($x:expr) => {
        !$x
    };
}
