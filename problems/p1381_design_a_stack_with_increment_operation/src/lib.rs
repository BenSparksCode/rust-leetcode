struct CustomStack {
    stack: Vec<i32>,
    max: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        CustomStack {
            stack: vec![],
            max: maxSize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() != self.max as usize {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            return -1;
        } else {
            self.stack.pop().unwrap()
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let end = std::cmp::min(self.stack.len() as i32, k);

        for i in 0..end {
            self.stack[i as usize] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_stack_with_increment_operation_example_1() {
        let mut stk = CustomStack::new(3); // Stack is Empty []

        // Test operations
        assert_eq!(stk.push(1), ()); // stack becomes [1]
        assert_eq!(stk.push(2), ()); // stack becomes [1, 2]
        assert_eq!(stk.pop(), 2); // return 2, stack becomes [1]
        assert_eq!(stk.push(2), ()); // stack becomes [1, 2]
        assert_eq!(stk.push(3), ()); // stack becomes [1, 2, 3]
        assert_eq!(stk.push(4), ()); // stack still [1, 2, 3], maxSize is 3

        stk.increment(5, 100); // stack becomes [101, 102, 103]
        stk.increment(2, 100); // stack becomes [201, 202, 103]

        assert_eq!(stk.pop(), 103); // return 103, stack becomes [201, 202]
        assert_eq!(stk.pop(), 202); // return 202, stack becomes [201]
        assert_eq!(stk.pop(), 201); // return 201, stack becomes []
        assert_eq!(stk.pop(), -1); // stack is empty, return -1
    }
}
