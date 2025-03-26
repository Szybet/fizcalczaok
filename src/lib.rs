use std::{i32, process, str::FromStr};

use log::{debug, error};
use rust_decimal::{dec, Decimal, MathematicalOps};

mod helpers;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum oper {
    plus,
    minus,
    razy,
    dzielenie,
}

impl ToString for oper {
    fn to_string(&self) -> String {
        match self {
            oper::plus => "+".to_string(),
            oper::minus => "-".to_string(),
            oper::razy => "*".to_string(),
            oper::dzielenie => "/".to_string(),
        }
    }
}

impl oper {
    pub fn return_list() -> Vec<String> {
        vec![
            oper::plus.to_string(),
            oper::minus.to_string(),
            oper::razy.to_string(),
            oper::dzielenie.to_string(),
        ]
    }

    pub fn from_str(str: &str) -> oper {
        if str == "+" {
            return oper::plus;
        }
        if str == "-" {
            return oper::minus;
        }
        if str == "*" {
            return oper::razy;
        }
        if str == "/" {
            return oper::dzielenie;
        }
        error!("Unknown operation");
        process::exit(1);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct liczba {
    pub liczba: Decimal,
    pub dokladna: bool,
}

impl liczba {
    pub fn new(licz: Decimal, dokl: bool) -> liczba {
        liczba {liczba: licz, dokladna: dokl}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct dzialanie {
    pub liczby: Vec<liczba>,
    pub operacje: Vec<oper>,
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
        let dec = Decimal::from_str(&liczba.replace("d", "")).unwrap();
        if liczba.contains("d") {
            dzial.liczby.push(liczba::new(dec, true));
        } else {
            dzial.liczby.push(liczba::new(dec, false));
        }
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
        if liczba.dokladna {
            continue;
        }
        let mut after_dec: i32 = 0;
        let decimal_str = liczba.liczba.to_string();
        if let Some(dot_index) = decimal_str.find('.') {
            after_dec = (decimal_str.len() - dot_index - 1) as i32;
        }
        if after_dec < smallest {
            smallest = after_dec;
        }
    }
    smallest
}

pub fn ile_cyfr_znaczacych(dec: Decimal) -> usize {
    let mut str = dec.to_string();
    str = str.replace(".", "");
    let mut vec = str.chars().collect::<Vec<_>>();
    while vec[0] == '0' {
        vec.remove(0);
    }
    vec.iter().count() as usize
}

pub fn oblicz(dzial: dzialanie) -> Decimal {
    let mut dzial_mut = dzial.clone();
    let mut fin = dzial_mut.liczby.remove(0);
    while dzial_mut.liczby.len() > 0 {
        let fin_saved = fin.clone();
        let operacja = dzial_mut.operacje.remove(0);
        let to_dec = dzial_mut.liczby.remove(0);
        match operacja {
            oper::plus => {
                fin.liczba = fin.liczba + to_dec.liczba;
            }
            oper::minus => {
                fin.liczba = fin.liczba - to_dec.liczba;
            }
            oper::razy => {
                fin.liczba = fin.liczba * to_dec.liczba;
            }
            oper::dzielenie => {
                fin.liczba = fin.liczba / to_dec.liczba;
            }
        }
        if dzial.operacje.contains(&oper::razy) || dzial.operacje.contains(&oper::dzielenie) {
            let tmp_dzial = dzialanie {
                liczby: vec![fin_saved, to_dec] ,
                operacje: vec![operacja],
            };
            fin = zaokrąglij(fin.clone(), tmp_dzial);    
        }
    }

    if dzial.operacje.contains(&oper::plus) || dzial.operacje.contains(&oper::minus) {
        debug!("Minus lub plus, dodatkowe zaokraglenie w obliczeniach");
        fin = zaokrąglij(fin.clone(), dzial);
    }

    fin.liczba
}

pub fn zaokrąglij(obliczona_licz: liczba, dzial: dzialanie) -> liczba {
    let mut obliczona_licz_mut = obliczona_licz.clone();
    if (dzial.operacje.contains(&oper::plus) || dzial.operacje.contains(&oper::minus))
        && (dzial.operacje.contains(&oper::razy) || dzial.operacje.contains(&oper::dzielenie))
    {
        error!("Nie mieszaj operacji!");
        process::exit(1);
    }

    if dzial.operacje.contains(&oper::plus) || dzial.operacje.contains(&oper::minus) {
        let n = najmniej_dokladna_liczba(dzial.clone()) as u32;
        debug!("Operacja dodawania lub odejmowania...");
        debug!("Najmniej dokladna liczba dla {:?} wynosi: {:?}", dzial, n);
        obliczona_licz_mut.liczba.rescale(n);

        debug!("Zaokrąglij zwraca: {:?}", obliczona_licz_mut);
        return obliczona_licz_mut;
    }

    if dzial.operacje.contains(&oper::razy) || dzial.operacje.contains(&oper::dzielenie) {
        let mut n = usize::MAX;
        for i in dzial.liczby.clone() {
            if i.dokladna {
                continue;
            }
            let x = ile_cyfr_znaczacych(i.liczba);
            if x < n {
                n = x;
            }
        }
        debug!("Operacja mnozenia lub dzielenia...");
        debug!("Możliwa ilość znaczących liczb dla {:?} wynosi: {:?}", dzial, n);

        // Tu uciac do liczb znaczacych
        obliczona_licz_mut.liczba = obliczona_licz_mut.liczba.round_sf(n as u32).unwrap();

        debug!("Zaokrąglij zwraca: {:?}", obliczona_licz_mut);
        return obliczona_licz_mut;
    }

    error!("Nie udało sie wykryć operacji!");
    process::exit(1);
}

pub fn potegowanie(licz: Decimal, do_potegi: Decimal) -> Decimal {
    let mut licz_tmp = licz;
    licz_tmp = licz_tmp.powd(do_potegi);

    licz_tmp = licz_tmp.round_sf(ile_cyfr_znaczacych(licz) as u32).unwrap();
    licz_tmp
}

pub fn potegowanie_str_wrapper(str: String) -> Decimal {
    let splitted = str.split("^").collect::<Vec<_>>();
    let first = Decimal::from_str(splitted.first().unwrap()).unwrap();
    let sec = Decimal::from_str(splitted.last().unwrap()).unwrap();
    potegowanie(first, sec)
}

// Tests

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
        liczby: vec![liczba::new(dec!(0.232), false), liczba::new(dec!(5.538), false), liczba::new(dec!(43.2), false)],
        operacje: vec![oper::plus, oper::plus],
    };
    assert_eq!(stworz_dzial("0,232+5,538+43,2"), dzial)
}

#[test]
fn small_najmniej_dokladna_liczba_1() {
    test_setup();

    assert_eq!(
        najmniej_dokladna_liczba(stworz_dzial("0,232+5,538+43,2")),
        1
    );
}

#[test]
fn small_najmniej_dokladna_liczba_2() {
    test_setup();

    assert_eq!(
        najmniej_dokladna_liczba(stworz_dzial("0,00335+10,689-10")),
        0
    );
}

// obliczanie_zaokraglanie
#[test]
fn oz1() {
    test_setup();

    let d = stworz_dzial("0,232+5,538+43,2");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(49.0).to_string());
}

#[test]
fn oz2() {
    test_setup();

    let d = stworz_dzial("0,00335 + 10,689 - 10");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(1).to_string());
}

#[test]
fn oz3() {
    test_setup();

    let d = stworz_dzial("66,45 + 1,05 - 2,225");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(65.28).to_string());
}

#[test]
fn oz4() {
    test_setup();

    let d = stworz_dzial("6,70002 + 11,00 + 2,295");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(20.00).to_string());
}

#[test]
fn oz5() {
    test_setup();

    let d = stworz_dzial("2,25 + 0,0073 + 0,0655");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(2.32).to_string());
}

#[test]
fn oz6() {
    test_setup();

    let d = stworz_dzial("12,56 / 4,2");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(3.0).to_string());
}

#[test]
fn oz7() {
    test_setup();

    let d = stworz_dzial("5,001 / d5");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(1.000).to_string());
}

#[test]
fn oz8() {
    test_setup();

    let d = stworz_dzial("2,2 * 9,337 / 0,0836");
    let o = oblicz(d.clone());
    assert_eq!(o.to_string(), dec!(250).to_string());
}

#[test]
fn oz9() {
    test_setup();

    let d1 = stworz_dzial("d15 * 0,526");
    let o1 = oblicz(d1.clone());
    assert_eq!(o1.to_string(), dec!(7.89).to_string());

    let d2 = stworz_dzial("d0,33 * 12,429");
    let o2: Decimal = oblicz(d2.clone());
    assert_eq!(o2.to_string(), dec!(4.1016).to_string());

    let d3 = stworz_dzial(&format!("{} / {}", o1, o2));
    let o3: Decimal = oblicz(d3.clone());
    assert_eq!(o3.to_string(), dec!(1.92).to_string());
}

#[test]
fn oz10() {
    test_setup();

    assert_eq!(potegowanie(dec!(10.02), dec!(2)).to_string(), dec!(100.4).to_string());
}

#[test]
fn oz11() {
    test_setup();

    assert_eq!(potegowanie(dec!(10.02), dec!(0.5)).to_string(), dec!(3.165).to_string());
}

#[test]
fn oz12() {
    test_setup();

    assert_eq!(potegowanie(dec!(0.947), dec!(3)).to_string(), dec!(0.849).to_string());
}

#[test]
fn oz13() {
    test_setup();

    assert_eq!(potegowanie(dec!(0.947), dec!(0.333333)).to_string(), dec!(0.982).to_string());
}

#[test]
fn oz13_str() {
    test_setup();

    assert_eq!(potegowanie_str_wrapper("0.947^0.333333".to_string()).to_string(), dec!(0.982).to_string());
}

