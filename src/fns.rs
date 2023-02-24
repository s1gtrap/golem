pub fn from_with_mask(other: &'static str, mask: u32) -> (usize, usize, Vec<u32>, Vec<u8>) {
    let mut u = other
        .lines()
        .next()
        .map(move |l| {
            let w = l.chars().count();
            other.lines().skip(1).fold(
                (
                    w,
                    1,
                    l.chars()
                        .map(char::is_whitespace)
                        .map(<bool as std::ops::Not>::not)
                        .map(u32::from)
                        .collect::<Vec<_>>(),
                    vec![0; w],
                ),
                |(w, h, s, c), l| {
                    assert_eq!(w, l.chars().count());
                    (
                        w,
                        h + 1,
                        s.into_iter()
                            .chain(
                                l.chars()
                                    .map(char::is_whitespace)
                                    .map(<bool as std::ops::Not>::not)
                                    .map(u32::from),
                            )
                            .collect(),
                        c.into_iter().chain(vec![0; w]).collect(),
                    )
                },
            )
        })
        .unwrap_or((0, 0, vec![], vec![]));
    let (w, h, ref mut s, ref mut c) = u;
    count(w, h, s, c, mask);
    u
}

pub fn step(w: usize, h: usize, states: &mut [u32], counts: &mut [u8], mask: u32) {
    for i in 0..h {
        for j in 0..w {
            if states[j + i * w] & mask == mask {
                if counts[j + i * w] < 2 || counts[j + i * w] > 3 {
                    states[j + i * w] ^= mask;
                }
            } else {
                if counts[j + i * w] == 3 {
                    states[j + i * w] ^= mask;
                }
            }
        }
    }

    count(w, h, states, counts, mask);
}

pub fn count(w: usize, h: usize, states: &mut [u32], counts: &mut [u8], mask: u32) {
    counts.iter_mut().for_each(|b| *b = 0);
    for y in 0..h {
        for x in 0..w {
            counts[x + y * w] = (((states[(x as isize - 1).rem_euclid(w as _) as usize
                + (y as isize - 1).rem_euclid(h as _) as usize * w]
                & mask)
                + (states[x + (y as isize - 1).rem_euclid(h as _) as usize * w] & mask)
                + (states[(x + 1) % w + (y as isize - 1).rem_euclid(h as _) as usize * w] & mask)
                + (states[(x as isize - 1).rem_euclid(w as _) as usize + y * w] & mask)
                + (states[(x + 1) % w + y * w] & mask)
                + (states[(x as isize - 1).rem_euclid(w as _) as usize + ((y + 1) % h) * w]
                    & mask)
                + (states[x + ((y + 1) % h) * w] & mask)
                + (states[(x + 1) % w + ((y + 1) % h) * w] & mask))
                / mask) as u8;
        }
    }
}

#[test]
fn test_step() {
    let mut s = [0, 128, 0, 0, 64, 192, 64, 0, 0, 128, 0, 0, 0, 0, 0, 0];
    let mut c0 = [0; 16];
    let mut c1 = [0; 16];
    count(4, 4, &mut s, &mut c0, 128);
    count(4, 4, &mut s, &mut c1, 64);
    step(4, 4, &mut s, &mut c0, 128);
    assert_eq!(s, [0, 0, 0, 0, 192, 192, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    step(4, 4, &mut s, &mut c1, 64);
    assert_eq!(s, [0, 64, 0, 0, 128, 192, 128, 0, 0, 64, 0, 0, 0, 0, 0, 0]);
}
