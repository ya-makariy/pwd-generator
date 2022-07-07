
use rand::{
    rngs::StdRng,
    Rng,
    SeedableRng
};

pub fn five_results(size: usize) -> Vec<String> {
    vec![
        generate(size),
        generate(size),
        generate(size),
        generate(size),
        generate(size),
    ]
}

fn generate(size: usize) -> String {
    let result = default(size);
    result.iter().
        map(|i| ALPHABET[(*i as usize) / 4]).
        collect()
}

fn default(size: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut result: Vec<u8> = vec![0; size];

    rng.fill(&mut result[..]);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deault() {
        let bytes = default(5);

        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn test_generate() {
        let v = generate(5);

        assert_eq!(v.len(), 5)
    }

    #[test]
    fn test_five_results() {
        let v = five_results(8);

        assert_eq!(v.len(), 5);

        for pwd in v {
            assert_eq!(pwd.len(), 8);
        }
    }
}

pub const ALPHABET: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];