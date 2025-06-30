fn bubblesort(mut array: Vec<i32>) -> Vec<i32> {
    let mut cycler = 0;
    while cycler < (array.len() - 3) {
        let mut current = 0;
        let mut next = 1;
        while current <= (array.len() - 2) {
            if array[current] > array[next] {
                array.swap(current, next);
            }
            current = current + 1;
            next = next + 1;
        }
        cycler = cycler + 1;
    }
    array
}

fn main() {
    let mut array: Vec<i32> = vec![9, 7, 2, 73, 6, 99, 5, 4, 1, 4, 5, 8, 3, 2];
    array = bubblesort(array);
    println!("{array:?}");
}

#[cfg(test)]
pub mod test {
    use super::bubblesort;

    #[test]
    fn can_sort_small_vector() -> Result<(), ()> {
        let vector = vec![99, 1, 7, 2, 73, 3];
        let vector = bubblesort(vector);
        
        assert_eq!(vector, vec![1, 2, 3, 7, 73, 99]);

        Ok(())
    }
}