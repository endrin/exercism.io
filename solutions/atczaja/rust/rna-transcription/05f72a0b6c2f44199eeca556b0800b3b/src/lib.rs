use std::cmp::PartialEq;

#[derive(Debug)]
pub struct RibonucleicAcid {
	seq: String,
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
	seq: String,
}

impl RibonucleicAcid {
	pub fn new(seq: &str) -> Self {
		RibonucleicAcid{seq: String::from(seq)}
	}
}

impl PartialEq for RibonucleicAcid {
	fn eq(&self, other: &RibonucleicAcid) -> bool {
		self.seq == other.seq
	}
}

impl DeoxyribonucleicAcid {
	pub fn new(seq: &str) -> Self {
		DeoxyribonucleicAcid{seq: String::from(seq)}
	}

	pub fn to_rna(self) -> Result<RibonucleicAcid, &'static str> {

		let transcribed: Result<String, &'static str> = self.seq.chars().map(|c| 
			transcribe_base(c)
		).collect();

		match transcribed {
			Ok(seq) => Ok(RibonucleicAcid{seq: seq}),
			Err(message) => Err(message),
		}

	}

}

fn transcribe_base(c: char) -> Result<char, &'static str> {
	match c {
		'A' => Ok('U'),
		'T' => Ok('A'),
		'G' => Ok('C'),
		'C' => Ok('G'),
		_ => Err("Encountered invalid base when trancscribing.")
	}
}

impl PartialEq for DeoxyribonucleicAcid {
	fn eq(&self, other: &DeoxyribonucleicAcid) -> bool {
		self.seq == other.seq
	}
}