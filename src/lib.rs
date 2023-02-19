#[derive(Clone, Debug, PartialEq)]
pub struct Universe(usize, usize, Vec<bool>, Vec<u8>);

impl Universe {
    pub fn new(w: usize, h: usize) -> Self {
        Universe(w, h, vec![false; w * h], vec![0; w * h])
    }

    pub fn step(&mut self) {
        let &mut Universe(w, h, ref mut states, ref mut counts) = self;

        for i in 0..h {
            for j in 0..w {
                if states[j + i * w] {
                    if counts[j + i * w] < 2 || counts[j + i * w] > 3 {
                        states[j + i * w] = false;
                    }
                } else {
                    if counts[j + i * w] == 3 {
                        states[j + i * w] = true;
                    }
                }
            }
        }

        self.count();
    }

    fn count(&mut self) {
        let &mut Universe(w, h, ref mut states, ref mut counts) = self;
        counts.iter_mut().for_each(|b| *b = 0);
        for y in 0..h {
            for x in 0..w {
                counts[x + y * w] = states[(x as isize - 1).rem_euclid(w as _) as usize
                    + (y as isize - 1).rem_euclid(h as _) as usize * w]
                    as u8
                    + states[x + (y as isize - 1).rem_euclid(h as _) as usize * w] as u8
                    + states[(x + 1) % w + (y as isize - 1).rem_euclid(h as _) as usize * w] as u8
                    + states[(x as isize - 1).rem_euclid(w as _) as usize + y * w] as u8
                    + states[(x + 1) % w + y * w] as u8
                    + states[(x as isize - 1).rem_euclid(w as _) as usize + ((y + 1) % h) * w]
                        as u8
                    + states[x + ((y + 1) % h) * w] as u8
                    + states[(x + 1) % w + ((y + 1) % h) * w] as u8;
            }
        }
    }
}

impl From<&'static str> for Universe {
    fn from(other: &'static str) -> Self {
        let mut u = other
            .lines()
            .next()
            .map(move |l| {
                let w = l.chars().count();
                other.lines().skip(1).fold(
                    Universe(
                        w,
                        1,
                        l.chars()
                            .map(char::is_whitespace)
                            .map(<bool as std::ops::Not>::not)
                            .collect(),
                        vec![0; w],
                    ),
                    |Universe(w, h, s, c), l| {
                        assert_eq!(w, l.chars().count());
                        Universe(
                            w,
                            h + 1,
                            s.into_iter()
                                .chain(
                                    l.chars()
                                        .map(char::is_whitespace)
                                        .map(<bool as std::ops::Not>::not),
                                )
                                .collect(),
                            c.into_iter().chain(vec![0; w]).collect(),
                        )
                    },
                )
            })
            .unwrap_or(Universe(0, 0, vec![], vec![]));
        (&mut u).count();
        u
    }
}

impl Iterator for Universe {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.step();
        Some(self.clone())
    }
}

#[test]
fn test_count() {
    assert_eq!(Universe::from(""), Universe(0, 0, vec![], vec![]));
    assert_eq!(Universe::from(" "), Universe(1, 1, vec![false], vec![0]));
    assert_eq!(
        Universe::from("  "),
        Universe(2, 1, vec![false; 2], vec![0; 2]),
    );
    assert_eq!(
        Universe::from(" o "),
        Universe(3, 1, vec![false, true, false], vec![3, 2, 3]),
    );
    assert_eq!(
        Universe::from(
            r#"   
   
   "#
        ),
        Universe(3, 3, vec![false; 9], vec![0; 9]),
    );
    assert_eq!(
        Universe::from(
            r#" o 
 o 
 o "#
        ),
        Universe(
            3,
            3,
            vec![false, true, false, false, true, false, false, true, false],
            vec![3, 2, 3, 3, 2, 3, 3, 2, 3],
        ),
    );
}

#[test]
fn test_step() {
    let mut u = Universe::from(
        r#"     
  o  
  o  
  o  
     "#,
    );
    u.step();
    assert_eq!(
        u,
        Universe::from(
            r#"     
     
 ooo 
     
     "#,
        ),
    );
}

#[test]
fn test_iter() {
    assert_eq!(
        Universe::from(
            r#"     
  o  
  o  
  o  
     "#,
        )
        .take(3)
        .collect::<Vec<_>>(),
        vec![
            Universe::from(
                r#"     
     
 ooo 
     
     "#,
            ),
            Universe::from(
                r#"     
  o  
  o  
  o  
     "#,
            ),
            Universe::from(
                r#"     
     
 ooo 
     
     "#,
            ),
        ],
    );
    assert_eq!(
        Universe::from(
            r#"     
   o 
 o o 
  oo 
     "#,
        )
        .take(4)
        .collect::<Vec<_>>(),
        vec![
            Universe::from(
                r#"     
  o  
   oo
  oo 
     "#,
            ),
            Universe::from(
                r#"     
   o 
    o
  ooo
     "#,
            ),
            Universe::from(
                r#"     
     
  o o
   oo
   o "#,
            ),
            Universe::from(
                r#"     
     
    o
  o o
   oo"#,
            ),
        ],
    );
}
