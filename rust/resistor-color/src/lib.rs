use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, IntEnum, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
  Black = 0,
  Brown = 1,
  Red = 2,
  Orange = 3,
  Yellow = 4,
  Green = 5,
  Blue = 6,
  Violet = 7,
  Grey = 8,
  White = 9
}

pub fn color_to_value(color: ResistorColor) -> usize {
  color as usize
}

pub fn value_to_color_string(value: usize) -> String {
  let mut string = String::new();

  if value < 10 {
    string.push_str(&format!("{:?}", ResistorColor::from_int(value).unwrap()))
  } else {
    string.push_str("value out of range")
  }

  string
}

pub fn colors() -> Vec<ResistorColor> {
  let mut vec = Vec::new();

  ResistorColor::into_enum_iter().for_each(|e| vec.push(e));

  vec
}
