#[cfg(test)]
mod tests {
    #[test]
    fn century_from_year() {
        let func = |year: i32| -> i32 {
            let century: f32 = year as f32 / 100 as f32;
            let century = century.ceil() as i32;
            century
        };

        assert_eq!(func(1905), 20);
    }

    #[test]
    fn str_reverse_eq() {
        let func = |txt: &str| -> String { String::from(txt).chars().rev().collect() };

        assert_eq!(func("world"), "dlrow")
    }

    #[test]
    fn adjacent_elements_product() {
        // Given an array of integers,
        // find the pair of adjacent elements that has the largest product
        // and return that product.
        // EXAMPLE:
        // For inputArray = [3, 6, -2, -5, 7, 3]
        // The output should be adjacentElementsProduct(inputArray) = 21
        // Because 7 and 3 produce the largest product.
        let func = |list: Vec<i32>| -> i32 {
            let mut prev = 0 as i32;
            let mut prod = 0 as i32;

            if list.len() < 1 {
                return 0;
            }

            if list.len() == 1 {
                return list[1];
            }

            for (i, v) in list.iter().enumerate() {
                if i == 0 {
                    prev = *v;
                    continue;
                }

                let cur = prev * *v;
                prev = *v;

                if i == 1 {
                    prod = cur;
                    continue;
                }

                if cur > prod {
                    prod = cur
                }
            }

            prod
        };

        assert_eq!(func(vec![3, 6, -2, -5, 7, 3]), 21);
        assert_eq!(func(vec![-1, -2]), 2);
        assert_eq!(func(vec![5, 1, 2, 3, 1, 4]), 6);
        assert_eq!(func(vec![1, 2, 3, 0]), 6);
        assert_eq!(func(vec![9, 5, 10, 2, 24, -1, -48]), 50);
        assert_eq!(func(vec![5, 6, -4, 2, 3, 2, -23]), 30);
        assert_eq!(func(vec![4, 1, 2, 3, 1, 5]), 6);
        assert_eq!(func(vec![-23, 4, -3, 8, -12]), -12);
    }
}
