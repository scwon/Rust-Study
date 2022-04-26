pub mod vec_mod;
pub mod company_mod;

pub fn parse_pig_latin (text: &mut String) {
  const VOWEL:[char; 5] = ['a', 'e', 'i', 'o', 'u']; // 대문자 생략...

  *text = text.trim().to_string();
  let first_str = match text.chars().next() {
    Some(t) => t,
    _ => ' ', 
  };

  let mut first_is_vowel = false;
  for c in VOWEL.iter(){
    if *c == first_str { first_is_vowel = true };
  }

  if first_is_vowel {
    text.push_str("-hay");
  }else {
    text.remove(0);
    text.push_str(&format!("-{}ay", first_str));
  }
}
