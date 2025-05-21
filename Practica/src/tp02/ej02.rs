pub fn es_primo(n: i32) -> bool {
    if n <= 1 {
        return false;
    } else {
        for i in 2..((n as f64).sqrt() as i32) + 1 {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test_es_primo() {
    assert_eq!(es_primo(2), true);
    assert_eq!(es_primo(4), false);
}
