use std::time::{Instant, Duration};

fn main() {
    let list = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100];
    let target = 3;
    let (duration, count) = binary_search(list, target);
    println!("Temps écoulé: {:?}, Nombre de boucles: {}", duration, count);
}

fn binary_search(list: &[i32], target: i32) -> (Duration, i32) {
    let start = Instant::now();
    let mut low = 0;
    let mut high = list.len() - 1;
    let mut count = 0;

    loop {
        count += 1;
        let mid = (low + high) / 2;
        let guess = list[mid];

        if guess == target {
            println!("Found it!");
            break;
        } else if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }

        if low > high {
            println!("Not found!");
            break;
        }
    }
    let duration = start.elapsed();
    (duration, count)
}