mod entropy;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn entropy(arg: &str) -> f32 {
    entropy::process(arg)
}

#[cfg(test)]
mod tests {
    use crate::entropy;

    #[test]
    fn test_entropy() {
        let num = "123456";

        let actual = entropy(num);

        println!("Entropy of {} is {}.", num, actual);
        assert_eq!(actual, 2.5849626);
    }
}
