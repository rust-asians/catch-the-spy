pub mod health;
mod money;
mod pressure;
mod body_part;
mod name;
mod hit_rate;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
pub struct Hello;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
