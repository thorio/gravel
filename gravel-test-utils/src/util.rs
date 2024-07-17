pub fn slices_equal<A: PartialEq<B>, B>(a: &[A], b: &[B]) -> bool {
	a.iter().zip(b.iter()).all(|(a, b)| a == b)
}
