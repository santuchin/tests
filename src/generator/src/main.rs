
pub enum Chunk<G, I, R = ()> {
	Yield(I, G),
	Return(R),
}

pub trait Generator<I, R = ()>
	where Self: Sized
{

	fn next(self) -> Chunk<Self, I, R>;

	fn unwrap(self) ->  (I, Self) {
		
		if let Chunk::Yield(item, next) = self.next() {
			(item, next)

		} else {
			panic!();
		}
	}

	fn consume(self) -> R
	{

		let mut generator = self;
 
		loop {
			match generator.next() {
				Chunk::Return(value) => break value,
				Chunk::Yield(_, next) => {
					generator = next;
				},
			}
		}

	}

	fn skip(self, mut times: usize) -> Chunk<Self, I, R> {

		let mut generator = self;

		loop {

			let chunk = generator.next();

			match chunk {
				Chunk::Return(value) => break Chunk::Return(value),
				Chunk::Yield(item, next) => {

					if times > 0 {
						generator = next;
						times -= 1;

					} else {
						break Chunk::Yield(item, next)
					}

				}
			}
		}

	}

	// fn map<T>(self, operation: fn(I) -> T) -> impl Generator<T, R> {}
	// fn filter(self, criteria: fn(I) -> bool) -> impl Generator<I, R> {}

}


struct Range {
	start: isize,
	step: isize,
	times: usize,
}

impl Range {

	fn new(start: isize, step: isize, times: usize) -> Self {
		Self {
			start,
			step,
			times,
		}
	}
}

impl Generator<isize> for Range {

	fn next(mut self) -> Chunk<Self, isize> {

		if self.times > 0 {

			let temporal = self.start;

			self.start += self.step;

			self.times -= 1;

			Chunk::Yield(temporal, self)

		} else {
			Chunk::Return(())
		}
	}
}

fn main() {

	let mut nums = Range::new(0, 1, 10);

	dbg!(nums.consume());
}

