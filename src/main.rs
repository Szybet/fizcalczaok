use std::{i32, process, str::FromStr};

use log::{debug, error};
use rust_decimal::{dec, Decimal};

mod helpers;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
enum oper {
    plus,
    minus,
}

impl ToString for oper {
    fn to_string(&self) -> String {
        match self {
            oper::plus => "+".to_string(),
            oper::minus => "-".to_string(),
        }
    }
}

impl oper {
    pub fn return_list() -> Vec<String> {
        vec![oper::plus.to_string(), oper::minus.to_string()]
    }

    pub fn from_str(str: &str) -> oper {
        if str == "+" {
            return oper::plus;
        }
        if str == "-" {
            return oper::minus;
        }
        error!("Unknown operation");
        process::exit(1);
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct dzialanie {
    liczby: Vec<Decimal>,
    operacje: Vec<oper>,
}

pub fn stworz_dzial(str: &str) -> dzialanie {
    let mut dzial = dzialanie {
        liczby: vec![],
        operacje: vec![],
    };
    let mut str_better = str.replace(",", ".");
    str_better = str_better.replace(" ", "");

    let (liczby, operacje) = helpers::split_strings_by_list(&str_better, oper::return_list());
    debug!("liczby wyszly: {:?}", liczby);
    debug!("operacje wyszly: {:?}", operacje);

    for liczba in liczby {
        dzial.liczby.push(Decimal::from_str(&liczba).unwrap());
    }

    for operacja in operacje {
        dzial.operacje.push(oper::from_str(&operacja));
    }

    dzial
}

// Najmniejsza ilość liczb po przecinku
pub fn najmniej_dokladna_liczba(dzial: dzialanie) -> i32 {
    let mut smallest = i32::MAX;
    for liczba in dzial.liczby {
        let mut after_dec: i32 = 0;
        let decimal_str = liczba.to_string();
        if let Some(dot_index) = decimal_str.find('.') {
            after_dec = (decimal_str.len() - dot_index - 1) as i32;
        }
        if after_dec < smallest {
            smallest = after_dec;
        }
    }
    smallest
}

pub fn oblicz(dzial: dzialanie) -> Decimal {
    let mut dzial_mut = dzial.clone();
    let mut fin: Decimal = dzial_mut.liczby.remove(0);
    while dzial_mut.liczby.len() > 0 {
        let operacja = dzial_mut.operacje.remove(0);
        let to_dec = dzial_mut.liczby.remove(0);
        match operacja {
            oper::plus => {
                fin = fin + to_dec;
            },
            oper::minus => {
                fin = fin - to_dec;
            },
        }
    }
    fin
}

pub fn zaokrąglij(obliczona_licz: Decimal, dzial: dzialanie) -> Decimal {
    let mut obliczona_licz_mut = obliczona_licz.clone();
    let n = najmniej_dokladna_liczba(dzial.clone()) as u32;
    debug!("Najmniej dokladna liczba dla {:?} wynosi: {:?}", dzial, n);
    obliczona_licz_mut.rescale(n);

    debug!("Zaokrąglij zwraca: {}", obliczona_licz_mut);
    obliczona_licz_mut
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    debug!("Starting!");
}

use std::sync::Once;

static INIT: Once = Once::new();

fn test_setup() {
    INIT.call_once(|| {
        env_logger::init_from_env(
            env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
        );
    });
}

#[test]
fn small_tworzenie_dzialania() {
    test_setup();

    let dzial = dzialanie {
        liczby: vec![dec!(0.232), dec!(5.538), dec!(43.2)],
        operacje: vec![oper::plus, oper::plus],
    };
    assert_eq!(stworz_dzial("0,232+5,538+43,2"), dzial)
}

#[test]
fn small_obliczanie_dodawanie() {
    test_setup();

    assert_eq!(oblicz(stworz_dzial("0,232+5,538+43,2")), dec!(48.970));
}

#[test]
fn small_obliczanie_odejmowanie() {
    test_setup();

    assert_eq!(oblicz(stworz_dzial("0,00335+10,689-10")), dec!(0.69235));
}

#[test]
fn small_najmniej_dokladna_liczba_1() {
    test_setup();

    assert_eq!(najmniej_dokladna_liczba(stworz_dzial("0,232+5,538+43,2")), 1);
}

#[test]
fn small_najmniej_dokladna_liczba_2() {
    test_setup();

    assert_eq!(najmniej_dokladna_liczba(stworz_dzial("0,00335+10,689-10")), 0);
}

// obliczanie_dodawanie_zaokraglanie
#[test]
fn odz1() {
    test_setup();

    let d = stworz_dzial("0,232+5,538+43,2");
    let o = oblicz(d.clone());
    assert_eq!(zaokrąglij(o, d).to_string(), dec!(49.0).to_string());
}

#[test]
fn odz2() {
    test_setup();

    let d = stworz_dzial("0,00335 + 10,689 - 10");
    let o = oblicz(d.clone());
    assert_eq!(zaokrąglij(o, d).to_string(), dec!(1).to_string());
}

#[test]
fn odz3() {
    test_setup();

    let d = stworz_dzial("66,45 + 1,05 - 2,225");
    let o = oblicz(d.clone());
    assert_eq!(zaokrąglij(o, d).to_string(), dec!(65.28).to_string());
}

#[test]
fn odz4() {
    test_setup();

    let d = stworz_dzial("6,70002 + 11,00 + 2,295");
    let o = oblicz(d.clone());
    assert_eq!(zaokrąglij(o, d).to_string(), dec!(20.00).to_string());
}

#[test]
fn odz5() {
    test_setup();

    let d = stworz_dzial("2,25 + 0,0073 + 0,0655");
    let o = oblicz(d.clone());
    assert_eq!(zaokrąglij(o, d).to_string(), dec!(2.32).to_string());
}