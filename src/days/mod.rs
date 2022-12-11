pub(crate) mod day_01;
pub(crate) mod day_02;
pub(crate) mod day_03;
pub(crate) mod day_04;
pub(crate) mod day_05;
pub(crate) mod day_06;
pub(crate) mod day_07;
pub(crate) mod day_08;
pub(crate) mod day_09;
pub(crate) mod day_10;
pub(crate) mod day_11;

pub(crate) trait Day {
    fn part_1(&mut self) -> String;
    fn part_2(&mut self) -> String;
}
