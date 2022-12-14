use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

//repr is needed to define the alignment of the enum  
#[repr(u32)]
//need to add the traits from external crates to the derive attribute
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {

    //from int returns an enum with the "OK" values and the error values.

    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => String::from("value out of range")
    }
    

}

pub fn colors() -> Vec<ResistorColor> {

    let mut colors_vector:Vec<_> = all::<ResistorColor>().collect::<Vec<_>>();

    colors_vector.sort_by(|a, b| a.int_value().cmp(&b.int_value()));

    colors_vector

}
