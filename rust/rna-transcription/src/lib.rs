use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    strand: String,
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(input: &str) -> Self {
        DeoxyribonucleicAcid {
            strand: String::from(input),
        }
    }

    pub fn to_rna(
        &self,
    ) -> Result<RibonucleicAcid, &'static str> {
        self.strand
            .chars()
            .map(|c| match c {
                'G' => Ok('C'),
                'C' => Ok('G'),
                'T' => Ok('A'),
                'A' => Ok('U'),
                _ => Err("Given DNA is invalid"),
            })
            .collect()
    }
}

impl RibonucleicAcid {
    pub fn new(input: &str) -> Self {
        RibonucleicAcid {
            strand: String::from(input),
        }
    }
}

impl FromIterator<char> for RibonucleicAcid {
    fn from_iter<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        RibonucleicAcid {
            strand: chars.into_iter().collect(),
        }
    }
}
