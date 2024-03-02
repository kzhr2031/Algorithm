// linear search (線形探索)

fn main() {
    let N = 50;

    // 値の全探索
    for i in 0..=9 {
        for j in 0..=9 {
            if i * j == N {
                println!("{}", N);
                return;
            }
        }
    }
}