use std::thread::sleep;
use std::time::Duration;

fn main() {
    screen_clear();
    let t = 9;

    let mut a = vec![
        [t, 0, 0, 0, 0, 0, 0],
        [0, t, 0, 0, 0, 0, 0],
        [0, 0, t, 0, 0, 0, 0],
        [0, 0, 0, t, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    
    let rings = ring_no(&mut a);
    
    loop {
        for j in 0..rings {
            spin(&mut a, j);
        }
        print_matrix(&mut a);
        screen_clear()
    }
}
fn print_matrix(array1: &mut Vec<[i32; 7]>) {
    for i in 0..array1.len() {
        for j in 0..array1.len() {
            if array1[i][j] == 0 {
                print!("{}", array1[i][j]);
            } else {
                print!("\x1b[93m{}\x1b[0m", array1[i][j]);
            }

            print!(" ");
        }
        println!();
    }
}
fn ring_no(array1: &mut Vec<[i32; 7]>) -> i32 {
   // println!("Order:{}", array1.len());
    let mut l = array1.len();
    let mut counter = 0;
    while l > 1 {
        l = l - 2;
        counter = counter + 1
    }
    return counter;
}
fn screen_clear() {
    print!("\x1B[2J\x1B[1;1H");
    let time = Duration::from_millis(360);
    sleep(time)
}
fn spin(ar: &mut Vec<[i32; 7]>, ring: i32) {
    let b = ar.len() as i32;
    let mut ar0: Vec<[i32; 2]> = Vec::new();

    for i in ring..b - ring {
        ar0.push([ring, i]);
    }
    for i in ring + 1..b - ring {
        ar0.push([i, b - 1 - ring]);
    }

    for i in ((ring)..(b - ring - 1)).rev() {
        ar0.push([b - 1 - ring, i]);
    }

    for i in ((ring + 1)..(b - ring - 1)).rev() {
        ar0.push([i, ring]);
    }

    if ring % 2 == 0 {
        let temp = ar[ar0[0][0] as usize][ar0[0][1] as usize];
        for i in 0..ar0.len() - 1 {
            ar[ar0[i][0] as usize][ar0[i][1] as usize] =
                ar[ar0[i + 1][0] as usize][ar0[i + 1][1] as usize];
        }
        ar[ar0[ar0.len() - 1][0] as usize][ar0[ar0.len() - 1][1] as usize] = temp;
    } else {
        let temp = ar[ar0[ar0.len() - 1][0] as usize][ar0[ar0.len() - 1][1] as usize];
        for i in (0..ar0.len()).rev() {
            ar[ar0[i][0] as usize][ar0[i][1] as usize] =
                ar[ar0[i.saturating_sub(1)][0] as usize][ar0[i.saturating_sub(1)][1] as usize];
        }
        ar[ar0[0][0] as usize][ar0[0][1] as usize] = temp;
    }
}
