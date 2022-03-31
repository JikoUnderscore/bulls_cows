#![allow(non_camel_case_types)]

use std::collections::HashSet;

const DIGITS: u8 = 9;              // 1, 2, 3, 4, 5, 6, 7, 8, 9
// const DIGIT_POSISTIONS: u8 = 4; // ----
const SIZE: usize = 6561;

// const SECRET: (u8, u8, u8, u8) = (5, 9, 1, 4);

// https://www.youtube.com/watch?v=FR_71HyBytE&t=846s
fn main() {
    let secret = match handle_cla() {
        Ok(e) => { e }
        Err(_) => { return; }
    };

    let mut guesses = [Some((0, 0, 0, 0)); SIZE]; // 6561 = (DIGITS pow DIGIT_POSISTIONS) as usize
    {
        let mut n = 0;
        for h in 1..=DIGITS {
            for s in 1..=DIGITS {
                for d in 1..=DIGITS {
                    for e in 1..=DIGITS {
                        // println!("{}{}{}{}", h, s, d, e);
                        guesses[n] = Some((h, s, d, e));
                        n += 1;
                    }
                }
            }
        }
    }
    let mut answare_index = 0;
    let mut chosen_number;
    let mut luup = 0;

    let mut run = true;
    while run {
        chosen_number = choose_number_gen(&guesses);
        println!("---------------------");
        println!("chosen_number {:?}", chosen_number);
        compare_numbers(&mut run, chosen_number.unwrap(), secret, &mut guesses);
        println!();
        answare_index = found_answare(&mut run, &guesses);



        luup += 1;
    }
    println!("---------------------");
    println!("loops {:?}", luup);
    println!("SECRET {:?}", secret);
    println!("answare {:?}", guesses[answare_index]);
}


fn found_answare(run: &mut bool, guesses: &[Option<(u8, u8, u8, u8)>; SIZE]) -> usize {
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

fn compare_numbers(run: &mut bool, guess: (u8, u8, u8, u8), secret: (u8, u8, u8, u8), guesses: &mut [Option<(u8, u8, u8, u8)>; SIZE]) {
    let mut witch = [true; 15];

    let bull_cow = play_bull_cows(&guess, &secret);

    match bull_cow {
        0 => {  witch[0] = false; println!("0 0"); },
        1 => {  witch[1] = false; println!("0 1"); },
        2 => {  witch[2] = false; println!("0 2"); },
        3 => {  witch[3] = false; println!("0 3"); },
        4 => {  witch[4] = false; println!("0 4"); },
        10 => { witch[5] = false; println!("1 0"); },
        11 => { witch[6] = false; println!("1 1"); },
        12 => { witch[7] = false; println!("1 2"); },
        13 => { witch[8] = false; println!("1 3"); },
        20 => { witch[9] = false; println!("2 0"); },
        21 => { witch[10] = false; println!("2 1"); },
        22 => { witch[11] = false; println!("2 2"); },
        31 => { witch[12] = false; println!("3 1"); },
        30 => { witch[13] = false; println!("3 0"); },
        40 => { witch[14] = false; println!("4 0"); *run = false;},
        _ => {}
    }


    for og in guesses.iter_mut() {
        if og.is_some() {
            let g = og.unwrap();
            let bull_cow = play_bull_cows(&guess, &g);

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
                _ => {}
            }
        }
    }
}

fn choose_number_gen(guesses: &[Option<(u8, u8, u8, u8)>; SIZE]) -> Option<(u8, u8, u8, u8)> {
    let mut table: [[u32; 15]; SIZE] = [[0; 15]; SIZE];


    for (i, r) in guesses.iter().enumerate() {
        if let Some(r) = r {
            for l in guesses.iter().flatten() {
                let bull_cow = play_bull_cows(r, l);
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
                    _ => {}
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

    let mut unic = HashSet::new();
    for (i, guess) in guesses.iter().enumerate() {
        if let Some(gu) = guess {
            if not!(unic.contains(&table[i])) {
                unic.insert(table[i]);
                println!("index: {}={:?},  {:?}", i, gu, table[i]);
            }
        }
    }

    return guesses[numer_index];
}

fn play_bull_cows(guess: &(u8, u8, u8, u8), secret: &(u8, u8, u8, u8)) -> u8 {
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

    if guess.0 != secret.0 &&
           (guess.0 == secret.1 ||
            guess.0 == secret.2 ||
            guess.0 == secret.3) {
        cows += 1;
    }
    if guess.1 != secret.1 &&
           (guess.1 == secret.0 ||
            guess.1 == secret.2 ||
            guess.1 == secret.3) {
        cows += 1;
    }
    if guess.2 != secret.2 &&
           (guess.2 == secret.1 ||
            guess.2 == secret.0 ||
            guess.2 == secret.3) {
        cows += 1;
    }
    if guess.3 != secret.3 &&
           (guess.3 == secret.0 ||
            guess.3 == secret.1 ||
            guess.3 == secret.2) {
        cows += 1;
    }


    return bulls * 10 + cows;
}


type void = ();
#[allow(non_upper_case_globals)]
const void: () = ();


fn handle_cla() -> Result<(u8, u8, u8, u8), void> {

    let args: Vec<String> = std::env::args().collect();

    // println!("{:?}", args);
    // assert!(args.len() >= 2, "Pass a number");
    if not!(args.len() >= 2) {
        println!("Enter a foure digit number as a commadn line argumen");
        return Err(void);
    }


    // println!("args[1].len() {:?}", args[1].len());
    // assert_eq!(args[1].len(), 4, "arg should be 4 char long");
    if not!(args[1].len() == 4) {
        println!("Number should be 4 char long!");
        return Err(void);
    }
    // println!("args[1] {:?}", args[1]);

    let ss = args[1].as_bytes();
    // println!("{:?} {:?} {:?} {:?}", ss[0] as char, ss[1]as char, ss[2]as char, ss[3]as char);

    let h = (ss[0] as char).to_digit(10).unwrap();
    let s = (ss[1] as char).to_digit(10).unwrap();
    let d = (ss[2] as char).to_digit(10).unwrap();
    let e = (ss[3] as char).to_digit(10).unwrap();

    // assert!(h > 0 || s > 0 || d > 0 || e > 0, "Numbers less than zero are not an allowed");
    // assert!(h <= 9 || s <= 9 || d <= 9 || e <= 9, "Some error !");
    if not!(h > 0 || s > 0 || d > 0 || e > 0) {
        println!("Numbers should be more than zero");
        return Err(void);
    }
    if not!(h <= 9 || s <= 9 || d <= 9 || e <= 9) {
        println!("Numbers should be less than nine");
        return Err(void);
    }
    return Ok((h as u8, s as u8, d as u8, e as u8));
}

#[macro_export]
macro_rules! not {
    ($x:expr) => {!$x};
}