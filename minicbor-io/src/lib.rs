#![forbid(unsafe_code)]

pub mod io;

static_assertions::const_assert! {
    std::mem::size_of::<u32>() <= std::mem::size_of::<usize>()
}
