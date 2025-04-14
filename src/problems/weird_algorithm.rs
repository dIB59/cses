#[allow(dead_code)]
fn wal(num: &mut i32) -> &mut i32 {
    while *num != 1 {
        if *num % 2 == 0 {
            *num = *num / 2;
        } else {
            *num = (3 * *num) + 1;
        }
        print!("{:?} -> ", num);
    }
    println!();
    num
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
