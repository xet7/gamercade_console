use super::OPERATOR_COUNT;

pub enum ModulatedBy {
    None,
    Single(usize),
    Double(usize, usize),
    Triple(usize, usize, usize),
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Algorithm(pub u8);

// These are similar to those found on the Dirtywave m8
impl Algorithm {
    pub const fn min() -> u8 {
        0
    }

    pub const fn max() -> u8 {
        11
    }

    pub fn get_definition(self) -> &'static AlgorithmDefinition {
        match self.0 {
            // A > B > C > D
            0 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },

            // [A + B] > C > D
            1 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::Double(0, 1),
                    ModulatedBy::Single(2),
                ],
            },

            // [[A > B] + C] > D
            2 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Double(1, 2),
                ],
            },

            // [[A > B] + [A > C]] > D
            3 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::Double(1, 2),
                ],
            },

            // [A + B + C] > D
            4 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::None,
                    ModulatedBy::Triple(0, 1, 2),
                ],
            },

            // [A > B > C] + D
            5 => &AlgorithmDefinition {
                carriers: [false, false, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::None,
                ],
            },

            // [A > B > C] + [A > B > D]
            6 => &AlgorithmDefinition {
                carriers: [false, false, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(1),
                ],
            },

            // [A > B] + [C > D]
            7 => &AlgorithmDefinition {
                carriers: [false, true, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Single(2),
                ],
            },

            // [A > B] + [A > C] + [A > D]
            8 => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                ],
            },

            // [A > B] + [A > C] + D
            9 => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                ],
            },

            // [A > B] + C + D
            10 => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [ModulatedBy::Single(0), ModulatedBy::None, ModulatedBy::None],
            },

            // A + B + C + D
            11 => &AlgorithmDefinition {
                carriers: [true, true, true, true],
                modulators: [ModulatedBy::None, ModulatedBy::None, ModulatedBy::None],
            },
            _ => panic!("invalid algorithm value"),
        }
    }
}

pub struct AlgorithmDefinition {
    pub(crate) carriers: [bool; OPERATOR_COUNT],
    pub(crate) modulators: [ModulatedBy; OPERATOR_COUNT - 1],
}
