use test_8_3_lib::*;
use std::{io, collections::HashMap};

fn main() {
    let num_list = vec![6,9,1,2,3,4,5,6,6,6,7,4,2,3,4,5];

    println!("평균: {}", vec_mod::get_avg(&num_list));
    println!("중간: {}", vec_mod::get_median(&num_list));
    println!("최빈: {}", vec_mod::get_mode(&num_list));

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Failed to read line");
    parse_pig_latin(&mut text);
    println!("{}", text);
}

// fn main() {
//     let mut company = HashMap::new();

//     company.insert("프론트팀", vec!["원성철", "김정은", "이상우", "조이성", "강수성", "김나현", "강수연", "노태경", "고지훈"]);
//     company.insert("데브옵스팀", vec!["정규석", "송재진", "임진태", "최정민"]);
//     company.insert("서버팀", vec!["김은하", "이병우", "장환석", "이하림", "이상훈"]);

//     loop {
//         println!("1.전체 조회 2.팀별 조회 3.팀원 추가 4.종료");
//         let mut select = String::new();
//         io::stdin().read_line(&mut select)
//             .expect("Failed to read line");
//         select = select.trim().to_string();

//         match &select[..] {
//             "1" => company_mod::select_all(&company),
//             "2" => company_mod::select_team(&company),
//             "3" => company_mod::insert_member(&mut company),
//             "4" => break,
//             _ => (),
//         }
//         println!(" --- \n")
//     }
// }