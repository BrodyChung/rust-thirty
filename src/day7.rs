// ref: 
pub struct Fibo {
    pub curr: usize,
    next: usize,
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo { curr: 0, next: 1 }
    }
}

impl Iterator for Fibo {
    type Item = usize; // associate type, just like swift

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.next;
        self.next = self.curr + self.next;
        self.curr = tmp;
        Some(self.curr)
    }
}

pub fn gen_fib(n: usize) -> Vec<i32> {
    let mut fibo = vec![0, 1];
    for i in 2..n {
        let next = fibo[i-1] + fibo[i-2];
        fibo.push(next)
    }
    fibo
}

pub fn recurv_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let curr = recurv_fib(n-1);
    let next = recurv_fib(n-2);
    curr + next
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fib() {
        let mut fibo: Fibo = Fibo::new();
        // ugly suffix of 1usize!
        assert_eq!(fibo.next(), Some(1usize));
        assert_eq!(fibo.next(), Some(1usize));
        assert_eq!(fibo.next(), Some(2usize));
        assert_eq!(fibo.next(), Some(3usize));
        assert_eq!(fibo.next(), Some(5usize));
        assert_eq!(fibo.next(), Some(8usize));
    }

    #[test]
    fn test_fib_gen() {
        let fibo = gen_fib(7);
        let expect = vec![0, 1, 1, 2, 3, 5, 8];
        // use Vec::eq to compare.
        assert!(fibo.eq(&expect));
    }

    #[test]
    fn test_recurv_fib() {
        assert_eq!(recurv_fib(7), 13);
    }
}
