use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub input: String,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Default)]
pub struct Segment {
    pub start: usize,
    pub len: usize,
    pub mul: u32,
    pub enabled: bool,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input_data = String::from_utf8(input.0.clone()).unwrap();
        let segments = Self::collect_all_mul(&input_data, false);
        Self {
            input: input_data,
            segments,
        }
    }

    fn collect_all_mul(string: &str, disable_conditionals: bool) -> Vec<Segment> {
        let mut chars = string.chars();
        let mut muls = Vec::new();

        let mut state_machine = StateMachine::Start;

        let mut lhs = 0;
        let mut rhs = 0;

        let mut enabled = true;

        let mut segment = Segment {
            enabled,
            ..Default::default()
        };

        loop {
            let Some(c) = chars.next() else {
                break;
            };

            segment.len += 1;

            state_machine = match (state_machine, c) {
                (StateMachine::Start, 'm') => {
                    // 'm' is not included
                    segment.len -= 1;
                    let new_segment = std::mem::take(&mut segment);

                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 1,
                        mul: 0,
                        enabled,
                    };

                    muls.push(new_segment);

                    StateMachine::ReadM
                }
                (StateMachine::ReadM, 'u') => StateMachine::ReadU,
                (StateMachine::ReadU, 'l') => StateMachine::ReadL,
                (StateMachine::ReadL, '(') => StateMachine::ReadLParen(0),
                (StateMachine::ReadLParen(0) | StateMachine::ReadingDigits1, '0'..='9') => {
                    lhs *= 10;
                    lhs += c.to_digit(10).unwrap();
                    StateMachine::ReadingDigits1
                }
                (StateMachine::ReadingDigits1, ',') => StateMachine::ReadComma,
                (StateMachine::ReadComma | StateMachine::ReadingDigits2, '0'..='9') => {
                    rhs *= 10;
                    rhs += c.to_digit(10).unwrap();
                    StateMachine::ReadingDigits2
                }
                (StateMachine::ReadingDigits2, ')') => {
                    let mut new_segment = std::mem::take(&mut segment);
                    new_segment.mul = lhs * rhs;

                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 0,
                        mul: 0,
                        enabled,
                    };

                    muls.push(new_segment);

                    lhs = 0;
                    rhs = 0;
                    StateMachine::Start
                }
                (StateMachine::Start, 'd') => {
                    // 'd' is not included
                    segment.len -= 1;
                    let new_segment = std::mem::take(&mut segment);

                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 1,
                        mul: 0,
                        enabled,
                    };

                    StateMachine::ReadD
                }
                (StateMachine::ReadD, 'o') => StateMachine::ReadO,
                (StateMachine::ReadO, '(') => StateMachine::ReadLParen(1),
                (StateMachine::ReadO, 'n') => StateMachine::ReadN,
                (StateMachine::ReadN, '\'') => StateMachine::ReadQuote,
                (StateMachine::ReadQuote, 't') => StateMachine::ReadT,
                (StateMachine::ReadT, '(') => StateMachine::ReadLParen(2),
                (StateMachine::ReadLParen(1), ')') => {
                    let mut new_segment = std::mem::take(&mut segment);
                    new_segment.enabled = true;

                    enabled = true;
                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 0,
                        mul: 0,
                        enabled,
                    };

                    muls.push(new_segment);

                    StateMachine::Start
                }
                (StateMachine::ReadLParen(2), ')') => {
                    let new_segment = std::mem::take(&mut segment);

                    enabled = disable_conditionals;
                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 0,
                        mul: 0,
                        enabled,
                    };

                    muls.push(new_segment);

                    StateMachine::Start
                }
                (StateMachine::Start, _) => {
                    lhs = 0;
                    rhs = 0;
                    StateMachine::Start
                }
                _ => {
                    let new_segment = std::mem::take(&mut segment);

                    segment = Segment {
                        start: new_segment.start + new_segment.len,
                        len: 0,
                        mul: 0,
                        enabled,
                    };

                    muls.push(new_segment);

                    lhs = 0;
                    rhs = 0;
                    StateMachine::Start
                }
            };
        }

        muls
    }
}

enum StateMachine {
    Start,
    ReadM,
    ReadU,
    ReadL,
    ReadLParen(u8),
    ReadingDigits1,
    ReadComma,
    ReadingDigits2,
    ReadD,
    ReadO,
    ReadN,
    ReadQuote,
    ReadT,
}
