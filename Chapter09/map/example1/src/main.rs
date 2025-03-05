fn main() {

    let numbers = vec![1,2,3,4,5];

    let squared_numbers: Vec<i32> = numbers.iter().map(|&a| a * a).collect();

    println!("{:?}",squared_numbers);

    


}
