fn abbrev_name(name: &str) -> String {

let mut string = name.split_ascii_whitespace()
                        .map(|el| (el.as_bytes()[0] as char).to_uppercase())
                        .map(|el| el.to_string())
                        .collect::<String>();


string.insert(1, '.');
string

}

// Rust test example:
#[test]
fn sample_tests() {
  println!("{}", abbrev_name("Jim Bob"));
  assert_eq!(abbrev_name("Sam Harris"), "S.H");
  assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
  assert_eq!(abbrev_name("Evan Cole"), "E.C");
  assert_eq!(abbrev_name("P Favuzzi"), "P.F");
  assert_eq!(abbrev_name("David Mendieta"), "D.M");
  assert_eq!(abbrev_name("GertRude Cook"), "G.C")
}