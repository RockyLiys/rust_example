use std::fmt;
use std::fmt::{Display, Formatter};

pub(crate) struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub(crate) fn new(x: f64, y:f64, radius: f64) -> Circle {
        Circle {x, y, radius}
    }
    pub(crate) fn area(&self) -> f64 {
        std::f64::consts::PI*(self.radius * self.radius)
    }
    pub(crate) fn radius(&self) -> f64 {
        self.radius
    }
}

pub(crate) struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub(crate) fn new(width:u32, height:u32) -> Rectangle {
        Rectangle{width, height}
    }
    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }

}

impl Rectangle {
    pub(crate) fn can_hold(&self, other: &Circle) -> bool {
        self.width > other.x as u32 && self.height > other.y as u32
    }
}



#[derive(Debug, Clone)]
pub(crate) struct SomeUFn {
    pub(crate) name: String
}

#[derive(Clone)]
pub(crate) struct SomeTFn {
    pub(crate) name: String
}

impl Display for SomeTFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.name)
    }
}