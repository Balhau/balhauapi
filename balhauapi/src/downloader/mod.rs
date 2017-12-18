pub mod youtube;

pub trait Downloadable<Result> {
    fn download(&self) -> Result;
}
