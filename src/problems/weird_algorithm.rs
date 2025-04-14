#[allow(dead_code)]
fn wal(num: &mut i32) -> &mut i32 {
    while *num != 1 {
        if *num % 2 == 1 {
            *num = *num / 2;
        } else {
            *num = (3 * *num) + 1;
        }
        print!("{:?} -> ", num);
    }

    print!("{:?}", num);
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal() {
        assert!(*wal(&mut 3) == 1)
    }
}
