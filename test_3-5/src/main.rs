fn main() {
    const FA:f64 = 54.0;
    println!("화씨 {} => 섭씨 {}",FA, change_fahrenheit_to_celsius(FA));
    const CE:f64 = -10.0;
    println!("섭씨 {} => 화씨 {}",CE, change_celsius_to_fahrenheit(CE));

    let mut f_num_a = 1;
    let mut f_num_b = 1;
    const N:i64 = 20;
    for i in 1..N+1{
        println!("{} 번째 피보나취 수여얼 = {}", i, if i > 2 { if i % 2 == 0 { f_num_a += f_num_b; f_num_a } else { f_num_b += f_num_a; f_num_b } } else { f_num_a });
    }
    
    const GASA: [&str; 12] = [
        "partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",   
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];

    const SEOSU: [&str; 12] = [
        "1st",
        "2nd",
        "3rd",
        "4th", //th 통일합니다... 귀찮네요
        "5th",
        "6th",
        "7th",
        "8th",
        "9th",
        "10th",
        "11th",
        "12th"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", SEOSU[i]);
        println!("My true love sent     to me");
        for j in (0..i+1).rev() {
            if j == 0 {
                if i == 11{
                    println!("{} {}", if i == 0 { "A" } else { "And a" }, GASA[j]);
                }
                println!("{} {}", if i == 0 { "A" } else { "And a" }, GASA[j]);
            }else{
                println!("{}", GASA[j]);
            }

            if j == 5 { println!(""); }
        }
        println!("");
    }
}


// 화씨를 섭씨로 ㅎㅎ
fn change_fahrenheit_to_celsius (num: f64) -> f64 {
    (num - 32.0) * 5.0/9.0
}

fn change_celsius_to_fahrenheit (num: f64) -> f64 {
    num / 5.0*9.0 + 32.0
}