use itertools::Itertools;

/// cargo aoc bench results
/// Day4 - Part1/(default)  time:   [797.15 ns 819.82 ns 843.86 ns]                                    
/// Day4 - Part2/(default)  time:   [11.430 us 11.807 us 12.207 us]                                    
/// Generator Day4/(default) time:   [198.48 us 199.66 us 201.10 us]

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<[Option<String>; 7]> {
    input
        .split("\n\n")
        .map(|entry| {
            let mut passport = [None, None, None, None, None, None, None];
            entry
                .split(|b| b == ':' || b == ' ' || b == '\n')
                .tuples()
                .for_each(|(key, value)| {
                    let index = match key {
                        "byr" => 0,
                        "iyr" => 1,
                        "eyr" => 2,
                        "hgt" => 3,
                        "hcl" => 4,
                        "ecl" => 5,
                        "pid" => 6,
                        "cid" => 7,
                        _ => panic!("Unknown key {}", key),
                    };
                    if index < 7 {
                        passport[index] = Some(value.to_owned());
                    };
                });
            passport
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[[Option<String>; 7]]) -> usize {
    input
        .iter()
        .filter(|passport| passport.iter().all(|val| val.is_some()))
        .count()
}

fn validate(passport: &[Option<String>; 7]) -> bool {
    match passport {
        [Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)] => {
            byr.len() == 4
                && iyr.len() == 4
                && eyr.len() == 4
                && hgt.len() > 2
                && hcl.len() == 7
                && pid.len() == 9
                && pid.chars().all(char::is_numeric)
                && { ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()) }
                && { hcl.as_bytes()[0] == b'#' && hcl.chars().skip(1).all(char::is_alphanumeric) }
                && {
                    if let Ok(height) = hgt[..hgt.len() - 2].parse::<u8>() {
                        match &hgt[hgt.len() - 2..] {
                            "cm" => height >= 150 && height <= 193,
                            "in" => height >= 59 && height <= 76,
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                && {
                    if let (Ok(byr), Ok(iyr), Ok(eyr)) =
                        (byr.parse::<u32>(), iyr.parse::<u32>(), eyr.parse::<u32>())
                    {
                        byr >= 1920
                            && byr <= 2002
                            && iyr >= 2010
                            && iyr <= 2020
                            && eyr >= 2020
                            && eyr <= 2030
                    } else {
                        false
                    }
                }
        }
        _ => false,
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[[Option<String>; 7]]) -> usize {
    input.iter().filter(|passport| validate(passport)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = generate(
            r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#,
        );
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = generate(
            r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#,
        );
        assert_eq!(solve_part2(&input), 0);
        let input = generate(
            r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#,
        );
        assert_eq!(solve_part2(&input), 4);
    }
}
