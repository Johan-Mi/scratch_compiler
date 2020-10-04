use rand::random;
use std::iter;

pub type ID = String;

pub fn new_id() -> ID {
    const SOUP: &[u8] = b"!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz0123456789";

    String::from_utf8(
        iter::repeat_with(|| SOUP[random::<usize>() % SOUP.len()])
            .take(20)
            .collect(),
    )
    .unwrap()
}
