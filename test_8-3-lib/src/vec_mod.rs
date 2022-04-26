use std::{collections::HashMap};

pub fn get_avg(vec: &Vec<i32>) -> f32 {
  let mut sum = 0;
  for num in vec {
    sum+=num;
  }

  sum as f32 / vec.len() as f32
}

pub fn get_median(vec: &Vec<i32>) -> i32 {
  let mut copy_vec = vec.clone();
  copy_vec.sort();

  // println!("(소팅된 목록) {:?}", copy_vec);
  copy_vec[(copy_vec.len() as i32/2) as usize].clone() // 정수 나누기 소수 버림되어 중간의 idx 얻음 
}

pub fn get_mode(vec: &Vec<i32>) -> i32 {
  let mut count_map = HashMap::new();

  let mut max_count_num:Option<&i32> = None;

  for num in vec {
    let this_count = {
      let count = count_map.entry(num).or_insert(0);
      *count += 1;
      *count
    };

    max_count_num = match max_count_num {
      Some(max_count_num) => {
        let get_count = count_map.get(max_count_num);
        let get_count = if let Some(get_count) = get_count  {
          *get_count
        } else { 0 };

        if get_count > this_count {
          Some(max_count_num)
        }else {
          Some(num)
        }
      },
      _ => Some(num),
    };
  };

  // println!("map = {:#?}", count_map);

  if let Some(max_count_num) = max_count_num {
    *max_count_num
  } else {
    0
  }
}