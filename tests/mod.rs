#[cfg(test)]
mod tests {
    use compusplit::{map_reduce1, map_reduce2, map_reduce3};

    pub fn test_fn(a: i32) -> i32 {
        a + 1
    }

    mod map_reduce1 {
        use super::{map_reduce1, test_fn};

        #[test]
        fn empty_data() {
            let input: Vec<i32> = Vec::new();
            let output = map_reduce1(input, test_fn);
            let expected: Vec<i32> = Vec::new();
            assert_eq!(expected, output);
        }

        #[test]
        fn under_threshold_data() {
            let input: Vec<i32> = (0..10).collect();
            let output = map_reduce1(input, test_fn);
            let expected: Vec<i32> = (1..11).collect();
            assert_eq!(expected, output);
        }

        #[test]
        fn above_threshold_data() {
            let input: Vec<i32> = (0..10000000).collect();
            let output = map_reduce1(input, test_fn);
            let expected: Vec<i32> = (1..10000001).collect();
            assert_eq!(expected, output);
        }
    }

    mod map_reduce2 {
        use super::{map_reduce2, test_fn};

        #[test]
        fn empty_data() {
            let input: Vec<i32> = Vec::new();
            let output = map_reduce2(input, test_fn);
            let expected: Vec<i32> = Vec::new();
            assert_eq!(expected, output);
        }

        #[test]
        fn under_threshold_data() {
            let input: Vec<i32> = (0..10).collect();
            let output = map_reduce2(input, test_fn);
            let expected: Vec<i32> = (1..11).collect();
            assert_eq!(expected, output);
        }

        #[test]
        fn above_threshold_data() {
            let input: Vec<i32> = (0..10000000).collect();
            let output = map_reduce2(input, test_fn);
            let expected: Vec<i32> = (1..10000001).collect();
            assert_eq!(expected, output);
        }
    }

    mod map_reduce3 {
        use super::{map_reduce3, test_fn};

        #[test]
        fn empty_data() {
            let input: Vec<i32> = Vec::new();
            let output = map_reduce3(input, test_fn);
            let expected: Vec<i32> = Vec::new();
            assert_eq!(expected, output);
        }

        #[test]
        fn under_threshold_data() {
            let input: Vec<i32> = (0..10).collect();
            let output = map_reduce3(input, test_fn);
            let expected: Vec<i32> = (1..11).collect();
            assert_eq!(expected, output);
        }

        #[test]
        fn above_threshold_data() {
            let input: Vec<i32> = (0..1000000000).collect();
            let output = map_reduce3(input, test_fn);
            let expected: Vec<i32> = (1..1000000001).collect();
            assert_eq!(expected, output);
        }
    }
}
