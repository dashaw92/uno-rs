use std::time::SystemTime;

struct Xorshift {
    state: u32,
}

impl Xorshift {
    fn new(state: u32) -> Xorshift {
        Xorshift { state }
    }

    fn next(&mut self) -> u32 {
        let mut s = self.state;
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        self.state = s;
        self.state
    }
}

pub fn shuffle<T>(data: &mut [T]) {
    let time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u32;
    let mut r = Xorshift::new(time);

    let len = data.len();
    (0..len).into_iter().for_each(|i| {
        let idx = usize::min(r.next() as usize % len, i + 1);
        data.swap(i, idx);
    });
}