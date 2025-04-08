pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev: i32 = 0;

    while x != 0 {
        let pop = x % 10;
        x /= 10;

        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
            return 0;
        }

        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
            return 0;
        }

        rev = rev * 10 + pop;
    }

    rev
}

fn main () {
    let examples = [123, -123, 120, 0, 1534236469];
    for &num in examples.iter() {
        let res = reverse(num);
        println!("Input: {:>10} -> Output: {:>10}", num, res);
    }
}
