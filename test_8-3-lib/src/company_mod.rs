use std::{collections::HashMap, io};

pub fn select_all (company: &HashMap<&str, Vec<&str>>) {
  let mut merged_vec: Vec<(&str, &str)> = vec![];

  for (team, names) in company {
    for name in names {
      merged_vec.push((name, team));
    }
  }

  merged_vec.sort();

  println!(" 전체 목록 ");
  for man in merged_vec {
    println!("{} - [{}]", man.0, man.1);
  }
}

pub fn select_team (company: &HashMap<&str, Vec<&str>>) {
  let mut sorted_team_names : Vec<&str> = vec![];
  let mut copy_company = company.clone();

  for (team, mut names) in &mut copy_company {
    sorted_team_names.push(team);
    names.sort();
  }

  sorted_team_names.sort();

  print!("\t");
  for team in &sorted_team_names {
    print!("[{}]\t\t", team);
  }
  println!("");

  let mut idx = 0;
  loop {
    let mut break_switch = true;
    print!("\t");
    for team in &sorted_team_names {
      match copy_company.get(team) {
        Some(names) => {
          let name = names.get(idx);
          if let Some(name) = name {
            break_switch = false;
            print!(" {}\t\t\t", name);
          } else {
            print!("\t\t\t");
          }
        },
        None => print!("\t\t\t"),
      }
    }
    println!("");
    idx += 1;
    if break_switch { break }
  }
}

pub fn insert_member (company: &mut HashMap<&str, Vec<&str>>) {
  let mut team = String::new();
  println!("추가할 팀: ");
  {
    io::stdin().read_line(&mut team)
      .expect("Failed to read line");
  }
  
  println!("추가할 이름: ");
  let mut name = String::new();
  io::stdin().read_line(&mut name)
  .expect("Failed to read line");

  let team = &team[..];

  // company.insert(team);
}