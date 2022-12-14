use anyhow::{Context, Error, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn prev(&self) -> Axis {
        match self {
            Axis::X => Axis::Z,
            Axis::Y => Axis::X,
            Axis::Z => Axis::Y,
        }
    }

    pub fn coor(&self) -> Coor3 {
        match self {
            Axis::X => Coor3::new(1, 0, 0),
            Axis::Y => Coor3::new(0, 1, 0),
            Axis::Z => Coor3::new(0, 0, 1),
        }
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct Coor3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coor3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Coor3 { x, y, z }
    }

    pub fn axis(&self, axis: Axis) -> i64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn manhattan(&self) -> i64 {
        self.x + self.y + self.z
    }

    pub fn dot(&self, other: Coor3) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Coor3) -> Coor3 {
        Coor3::new(
            -self.z * other.y + self.y * other.z,
            self.z * other.x - self.x * other.z,
            -self.y * other.x + self.x * other.y,
        )
        // (a,b,c)x(d,e,f) = (-c e + b f, c d - a f, -b d + a e)
    }
}
impl fmt::Debug for Coor3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for Coor3 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (x, yz) = s.split_once(',').context("No 1st comma")?;
        let (y, z) = yz.split_once(',').context("No 2nd comma")?;
        Ok(Coor3::new(x.parse()?, y.parse()?, z.parse()?))
    }
}

impl From<(i64, i64, i64)> for Coor3 {
    fn from(tup: (i64, i64, i64)) -> Self {
        let (x, y, z) = tup;
        Coor3 { x, y, z }
    }
}

impl Add for Coor3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Coor3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Coor3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other * -1
    }
}

impl SubAssign for Coor3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Coor3 {
    type Output = Self;

    fn neg(self) -> Self {
        Coor3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<i64> for Coor3 {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Coor3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<i64> for Coor3 {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Coor3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<Coor3> for i64 {
    type Output = Coor3;
    fn mul(self, rhs: Coor3) -> Self::Output {
        rhs * self
    }
}

impl PartialOrd for Coor3 {
    fn partial_cmp(&self, other: &Coor3) -> Option<std::cmp::Ordering> {
        (self.x, self.y, self.x).partial_cmp(&(other.x, other.y, other.z))
    }
}

impl Ord for Coor3 {
    fn cmp(&self, other: &Coor3) -> std::cmp::Ordering {
        (self.x, self.y, self.x).cmp(&(other.x, other.y, other.z))
    }
}
