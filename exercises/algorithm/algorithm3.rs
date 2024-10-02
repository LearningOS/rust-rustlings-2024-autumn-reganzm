/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    let mut left = array.len() - 1;
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if array[i] > array[pos_max] {
                pos_max = i;
            }
        }
        // 交换数据，完成一轮数据的排序，并将待排序数据个数减1
        array.swap(left, pos_max);
        left -= 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
