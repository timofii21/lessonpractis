#[test]
fn main() {
    let base_height = 3;
    let number_of_triangles = 3;
    let max_height = base_height + number_of_triangles - 1;
    for j in 0..3 {
        let height = base_height + j;
        for i in 0..height {
            let stars = "*".repeat(2 * i + 1);
            let spaces = " ".repeat(max_height - i - 1);
            println!("{}{}", spaces, stars);
        }
    }
}
