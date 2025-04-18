#[allow(dead_code)]
fn wal(num: &mut u64) -> &mut u64 {

    while *num != 1 {
        print!("{num} ");

        if *num % 2 == 0 {
            *num = *num / 2;
        } else {
            *num = (3 * *num) + 1;
        }
    }
    print!("{num}");
    num
}

fn main() {
    let mut inp: u64 = std::io::read_to_string(std::io::stdin())
    .unwrap()
    .trim()
    .parse()
    .unwrap();
    wal(&mut inp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal() {
        assert!(*wal(&mut 3) == 1)
    }

    #[test]
    fn test_wal_big_num() {
        assert!(*wal(&mut 900) == 1)
    }
}
