fn main() {
    // フィールドを描画
    for y in 0..21 {
        for x in 0..13 {
            if x == 0 || x == 12 || y == 20 {
                print!("[]");
            }
        }
        println!();
    }
}
