pub fn eq<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialEq,
{
  move |actual: T| expected == actual
}

pub fn ne<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialEq,
{
  move |actual: T| expected != actual
}

pub fn ge<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialOrd,
{
  move |actual: T| actual >= expected
}

pub fn gt<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialOrd,
{
  move |actual: T| actual > expected
}

pub fn le<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialOrd,
{
  move |actual: T| actual <= expected
}

pub fn lt<T>(expected: T) -> impl Fn(T) -> bool
where
  T: PartialOrd,
{
  move |actual: T| actual < expected
}

pub fn and<T>(a: impl Fn(T) -> bool, b: impl Fn(T) -> bool) -> impl Fn(T) -> bool
where
  T: Clone,
{
  move |actual: T| a(actual.clone()) && b(actual)
}

pub fn or<T>(a: impl Fn(T) -> bool, b: impl Fn(T) -> bool) -> impl Fn(T) -> bool
where
  T: Clone,
{
  move |actual: T| a(actual.clone()) || b(actual)
}

pub fn not<T>(a: impl Fn(T) -> bool) -> impl Fn(T) -> bool
where
  T: Sized,
{
  move |actual: T| !a(actual)
}

pub fn contains<T>(needle: T) -> impl Fn(T) -> bool
where
  T: AsRef<[u8]>,
{
  let needle = String::from_utf8_lossy(needle.as_ref()).to_string();
  move |actual: T| String::from_utf8_lossy(actual.as_ref()).contains(&needle)
}
