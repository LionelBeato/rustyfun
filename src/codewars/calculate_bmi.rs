/**
 * 
 * Write function bmi that calculates
 * body mass index (bmi = weight / height2).
 * if bmi <= 18.5 return "Underweight"
 * if bmi <= 25.0 return "Normal"
 * if bmi <= 30.0 return "Overweight"
 * if bmi > 30 return "Obese"
 * 
*/

pub fn bmi(weight: u32, height: f32) -> &'static str {
    let weight_cast = weight as f32; 
    let bmi = match weight as f32 / height.powf(2.0) {
        bmi if bmi <= 18.5  => "Underweight",
        bmi if bmi <= 25.0 => "Normal",
        bmi if bmi <= 30.0 => "Overweight",
        _ => "Obese",
    };
    bmi
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}