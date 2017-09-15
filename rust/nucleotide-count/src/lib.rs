use std::collections::HashMap;

static NUCLEOTIDS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(
    nuc: char,
    dna: &str,
) -> Result<usize, &'static str> {
    validate_nucleo(nuc)?;
    validate_dna(dna)?;
    Ok(do_count(&nuc, dna))
}

pub fn nucleotide_counts(
    dna: &str,
) -> Result<HashMap<char, usize>, &'static str> {
    validate_dna(dna)?;
    Ok(
        NUCLEOTIDS
            .iter()
            .map(|n| (*n, do_count(n, dna)))
            .collect(),
    )
}

fn validate_nucleo(nuc: char) -> Result<(), &'static str> {
    if !NUCLEOTIDS.contains(&nuc) {
        Err("Invalid nucleotide")
    } else {
        Ok(())
    }
}

fn validate_dna(dna: &str) -> Result<(), &'static str> {
    if dna.chars().any(|c| !NUCLEOTIDS.contains(&c)) {
        Err("Invalid DNA")
    } else {
        Ok(())
    }
}

#[inline]
fn do_count(nuc: &char, dna: &str) -> usize {
    dna.chars().filter(|elem| elem == nuc).count()
}
