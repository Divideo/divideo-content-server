use std::path::PathBuf;

use opentube_models::VideoDescriptor;

pub trait Database
where
    Self: Sized,
{
    fn open(path: PathBuf) -> Option<Self>;

    fn search(&mut self, start: usize, num: usize) -> Vec<VideoDescriptor>;
}
