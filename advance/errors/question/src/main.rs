use std::num::ParseFloatError;

fn sub(x: String, y: String) -> Result<f64, ParseFloatError> {
    let a = x.parse::<f64>()?;
    let b = y.parse::<f64>()?;
    Ok(a - b)
}

fn print(result: Result<f64, ParseFloatError>) {
    if let Ok(value) = result {
        println!("here is your value: {value}")
    } else {
        println!("we already make technical discourse>...");
    }
}

fn main() {
    let a = "1.21".to_string();
    let b = "3".to_string();
    print(sub(b, a))
}
