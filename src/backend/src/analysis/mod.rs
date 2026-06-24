pub mod tracks;
pub mod albums;
pub mod artists;

pub trait GroupScrobble<'a> {
    type Result;
    type Source;

    fn group(source: Self::Source) -> Self::Result;
}
