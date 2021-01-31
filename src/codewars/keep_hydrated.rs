/**
 * 
 * Nathan loves cycling.
 * Because Nathan knows it is important to stay hydrated, 
 * he drinks 0.5 litres of water per hour of cycling.
 * You get given the time in hours and you need to return 
 * the number of litres Nathan will drink, 
 * rounded to the smallest value.
 * For example:
 * time = 3 ----> litres = 1
 * time = 6.7---> litres = 3
 * time = 11.8--> litres = 5
 * 
 * 
 */


// pub fn litres(time: f64) -> i32 {
//     let litres: i32 = (time * 0.5) as i32; 
//     litres
// }

pub fn litres(time: f64) -> i32 {
    time as i32 / 2 
}

#[cfg(test)]
mod tests {
    use super::litres;

    #[test]
    fn sample_tests() {
        assert_eq!(litres(2.), 1);
        assert_eq!(litres(1.4), 0);
        assert_eq!(litres(12.3), 6);
        assert_eq!(litres(0.82), 0);
        assert_eq!(litres(11.8), 5);
        assert_eq!(litres(1787.), 893);
        assert_eq!(litres(0.), 0);
    }
}