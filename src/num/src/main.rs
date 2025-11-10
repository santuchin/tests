
type Chunk = usize;

#[derive(Clone, Debug)]
pub struct Natural {
	raw: Vec<Chunk>,
}

impl Natural {

	fn new() -> Self {
		Self {
			raw: Vec::new(),
		}
	}
}

impl std::fmt::Display for Natural {

	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		write!(formatter, "{:?}", self.raw).unwrap();

		Ok(())
	}
}

impl std::ops::AddAssign<usize> for Natural {

	fn add_assign(&mut self, mut value: usize) {

		let mut iterator = self.raw.iter_mut();

		loop {

			if let Some(chunk) = iterator.next() {

				let (result, overflow) = (*chunk).overflowing_add(value);

				*chunk = result;

				if overflow {
					value = overflow as _; // always 1

				} else {
					break;
				}

			} else {
				self.raw.push(value);
				break;
			}
		}

	}
}

impl std::ops::Add<usize> for Natural {

	type Output = Self;

	fn add(mut self, value: usize) -> Self::Output {
		self += value;
		self
	}
}

impl std::ops::Add<usize> for &Natural {

	type Output = Natural;

	fn add(mut self, value: usize) -> Self::Output {
		let mut result = self.clone();
		result += value;
		result
	}
}

macro_rules! nat {
	($number:expr) => {
		{

		}
	};
}

fn main() {

	let times: usize = std::env::args()
		.nth(1).unwrap()
		.parse().unwrap();



	let mut num = Natural::new();

	dbg!(&num + 2);

	dbg!(num);
}
