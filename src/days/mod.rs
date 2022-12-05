pub(crate) mod day_01;
pub(crate) mod day_02;
pub(crate) mod day_03;
pub(crate) mod day_04;
pub(crate) mod day_05;

pub(crate) trait Day {
    fn part_1(&mut self) -> String;
    fn part_2(&mut self) -> String;
}
