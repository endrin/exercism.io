#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    strand: String,
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    strand: String,
    valid: Result<(), &'static str>,
}

impl DeoxyribonucleicAcid {
    pub fn new(input: &str) -> Self {
        let strand = String::from(input);
        let valid =
            if strand.chars().all(|c| "ACGT".contains(c)) {
                Ok(())
            } else {
                Err("Given DNA is invalid")
            };

        DeoxyribonucleicAcid { strand, valid }
    }

    pub fn to_rna(
        &self,
    ) -> Result<RibonucleicAcid, &'static str> {
        self.valid?;
        let complement = self.strand
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>();
        Ok(RibonucleicAcid::new(complement.as_str()))
    }
}

impl RibonucleicAcid {
    pub fn new(input: &str) -> Self {
        RibonucleicAcid {
            strand: String::from(input),
        }
    }
}
