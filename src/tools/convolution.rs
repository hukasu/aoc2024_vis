pub struct Convolution<'a> {
    data: &'a [&'a [u8]],
    line: usize,
    column: usize,
}

impl<'a> Convolution<'a> {
    pub fn new(data: &'a [&[u8]]) -> Convolution<'a> {
        Self {
            data,
            line: 0,
            column: 0,
        }
    }
}

impl Iterator for Convolution<'_> {
    type Item = ([[u8; 3]; 3], usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.line >= self.data.len() {
            None
        } else {
            let next = [
                [
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
                [
                    self.data
                        .get(self.line)
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line)
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line)
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
                [
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
            ];
            let line = self.line;
            let column = self.column;
            self.column += 1;
            if self.column >= self.data[0].len() {
                self.column = 0;
                self.line += 1;
            }
            Some((next, line, column))
        }
    }
}
