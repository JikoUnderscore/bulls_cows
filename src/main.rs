#![allow(non_camel_case_types)]

use std::{collections::HashSet, num::NonZeroU32};

const SIZE: usize = 6561;

#[derive(Debug, Clone)]
pub struct NonZeroNums32(NonZeroU32);
const _: () = assert!(size_of::<Option<NonZeroNums32>>() == size_of::<u32>());

impl NonZeroNums32 {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Option<Self> {
        let val = u32::from_be_bytes([a, b, c, d]);
        return match NonZeroU32::new(val) {
            Some(x) => Some(Self(x)),
            None => None,
        };
    }

    pub fn new_none_zero(a: u8, b: u8, c: u8, d: u8) -> Self {
        let val = u32::from_be_bytes([a, b, c, d]);
        return NonZeroNums32(NonZeroU32::new(val).expect("to be non zero"));
    }

    pub fn get(self) -> (u8, u8, u8, u8) {
        self.0.get().to_be_bytes().into()
    }
}

struct ChoseNum {
    data: HashSet<[u32; 15]>,
}

impl ChoseNum {
    pub fn new() -> Self {
        Self { data: HashSet::with_capacity(15) } //TODO: find a better cap
    }

    fn choose_number(&mut self, guesses: &[Option<NonZeroNums32>]) -> NonZeroNums32 {
        let mut table: [[u32; 15]; SIZE] = [[0; 15]; SIZE];

        for (i, r) in guesses.iter().enumerate() {
            if let Some(r) = r {
                for l in guesses.iter().flatten() {
                    let bull_cow = play_bull_cows(r.clone(), l.clone());
                    match bull_cow {
                        0 => table[i][0] += 1,
                        1 => table[i][1] += 1,
                        2 => table[i][2] += 1,
                        3 => table[i][3] += 1,
                        4 => table[i][4] += 1,
                        10 => table[i][5] += 1,
                        11 => table[i][6] += 1,
                        12 => table[i][7] += 1,
                        13 => table[i][8] += 1,
                        20 => table[i][9] += 1,
                        21 => table[i][10] += 1,
                        22 => table[i][11] += 1,
                        31 => table[i][12] += 1,
                        30 => table[i][13] += 1,
                        40 => table[i][14] += 1,
                        _ => unsafe {
                            std::hint::unreachable_unchecked();
                        },
                    }
                }
            }
        }

        let mut m = u32::MAX;
        let mut numer_index = 0;
        for (i, a) in table.iter().enumerate() {
            let new_max = *a.iter().max().unwrap();
            if guesses[i].is_some() && (new_max < m) {
                m = new_max;
                numer_index = i;
            }
        }

        self.data.clear();
        for (i, guess) in guesses.iter().enumerate() {
            if let Some(gu) = guess {
                if not!(self.data.contains(&table[i])) {
                    self.data.insert(table[i]);
                    println!("index: {}={:?},  {:?}", i, gu.clone().get(), table[i]);
                }
            }
        }
        return guesses[numer_index].clone().expect("to be non zero");
    }
}

// https://www.youtube.com/watch?v=FR_71HyBytE&t=846s
fn main() {
    let secret = match handle_cla() {
        Ok(e) => e,
        Err(_) => {
            NonZeroNums32::new_none_zero(5, 8, 7, 6) /* return; */
        }
    };

    static mut ARRAY: [Option<NonZeroNums32>; SIZE] = make_array();
    let guesses = unsafe { &mut (*(&raw mut ARRAY)) };
    let mut genn = ChoseNum::new();

    let mut answare_index = 0;
    let mut chosen_number;
    let mut luup = 0;

    let mut run = true;
    while run {
        println!("---------------------");
        chosen_number = genn.choose_number(guesses);
        println!("\nchosen_number {:?}", chosen_number.clone().get());
        compare_numbers(&mut run, chosen_number, secret.clone(), guesses);
        println!();
        answare_index = found_answare(&mut run, guesses);

        luup += 1;
    }
    println!("---------------------");
    println!("loops {:?}", luup);
    println!("SECRET {:?}", secret.get());
    println!("answare {:?}", guesses[answare_index].clone().unwrap().get());
}

const fn make_array() -> [Option<NonZeroNums32>; SIZE] {
    const DIGITS: u8 = 9;
    let mut arr: [Option<NonZeroNums32>; SIZE] = [const { None }; SIZE];
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
                    arr[n] = NonZeroNums32::new(h, s, d, e);
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

fn found_answare(run: &mut bool, guesses: &[Option<NonZeroNums32>]) -> usize {
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

fn compare_numbers(run: &mut bool, guess: NonZeroNums32, secret: NonZeroNums32, guesses: &mut [Option<NonZeroNums32>]) {
    let mut witch = [true; 15];

    let bull_cow = play_bull_cows(guess.clone(), secret);
    println!("{:02}", bull_cow);
    match bull_cow {
        0 => witch[0] = false,
        1 => witch[1] = false,
        2 => witch[2] = false,
        3 => witch[3] = false,
        4 => witch[4] = false,
        10 => witch[5] = false,
        11 => witch[6] = false,
        12 => witch[7] = false,
        13 => witch[8] = false,
        20 => witch[9] = false,
        21 => witch[10] = false,
        22 => witch[11] = false,
        31 => witch[12] = false,
        30 => witch[13] = false,
        40 => {
            witch[14] = false;
            *run = false;
        }
        _ => unsafe {
            std::hint::unreachable_unchecked();
        },
    }

    for og in guesses.iter_mut() {
        if let Some(g) = og {
            let bull_cow = play_bull_cows(guess.clone(), g.clone());

            match bull_cow {
                0 if witch[0] => *og = None,
                1 if witch[1] => *og = None,
                2 if witch[2] => *og = None,
                3 if witch[3] => *og = None,
                4 if witch[4] => *og = None,
                10 if witch[5] => *og = None,
                11 if witch[6] => *og = None,
                12 if witch[7] => *og = None,
                13 if witch[8] => *og = None,
                20 if witch[9] => *og = None,
                21 if witch[10] => *og = None,
                22 if witch[11] => *og = None,
                31 if witch[12] => *og = None,
                30 if witch[13] => *og = None,
                40 if witch[14] => *og = None,
                _ => unsafe {
                    std::hint::unreachable_unchecked();
                },
            }
        }
    }
}

fn play_bull_cows(guess: NonZeroNums32, secret: NonZeroNums32) -> u8 {
    let guess = guess.get();
    let secret = secret.get();

    let mut bulls = 0;
    let mut cows = 0;

    if guess.0 == secret.0 {
        bulls += 1;
    }
    if guess.1 == secret.1 {
        bulls += 1;
    }
    if guess.2 == secret.2 {
        bulls += 1;
    }
    if guess.3 == secret.3 {
        bulls += 1;
    }

    if guess.0 != secret.0 && (guess.0 == secret.1 || guess.0 == secret.2 || guess.0 == secret.3) {
        cows += 1;
    }
    if guess.1 != secret.1 && (guess.1 == secret.0 || guess.1 == secret.2 || guess.1 == secret.3) {
        cows += 1;
    }
    if guess.2 != secret.2 && (guess.2 == secret.1 || guess.2 == secret.0 || guess.2 == secret.3) {
        cows += 1;
    }
    if guess.3 != secret.3 && (guess.3 == secret.0 || guess.3 == secret.1 || guess.3 == secret.2) {
        cows += 1;
    }

    return bulls * 10 + cows;
}

fn handle_cla() -> Result<NonZeroNums32, ()> {
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
    return Ok(NonZeroNums32::new_none_zero(h as u8, s as u8, d as u8, e as u8));
}

#[macro_export]
macro_rules! not {
    ($x:expr) => {
        !$x
    };
}
