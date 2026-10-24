/*fn main () { // harmless casting code
    let cows_per_barn = 20;
    let number_of_barns = 30;
    let total_cows = cow_per_barn * number_of_barns;
    println!("Total cows equalls: {total_cows}");
}*/

/*fn main () { // wrong casting code
    let cows_per_barn: i32 = 20;
    let number_per_barns:i64 = 30;
    let total_cows = cows_per_barn * number_per_barns;
    println!("Total cows equals: {total_cows}");
}*/

/*fn main() { // casting code
    let cows_per_barn: i32 = 10;
    let number_per_barns: i64 = 40;

    let total_barns = cows_per_barn as i64 * number_per_barns;

    println!("Total of cows: {total_barns}");
}*/

fn main() { // casting code 2
    let amina_hostel: i32 = 20000000;
    let azeeza_hostel: i64 = 15000000;

    let total_amount = amina_hostel as i64 * azeeza_hostel;

    println!("Total amount for both hostel is: {total_amount}");
}
