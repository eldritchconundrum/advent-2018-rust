// how comes unwrap doesn't do this by default?
#[macro_export]
macro_rules! e {
    ($fmt:expr, $($e:expr),*) => { &format!(concat!("{}:{}: ", $fmt), file!(), line!(), $($e),*) };
    ($e:expr) => { &format!("{}:{}: {}", file!(), line!(), $e) };
    () => { &format!("{}:{}", file!(), line!()) };
}

// how comes we can't derive Display? Debug is a fine default.
#[macro_export]
macro_rules! implement_display_as_debug {
    ($type: ty) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

// we need this macro to bypass the "helpful" compiler error in fmt_!():
// "attempted to repeat an expression containing no syntax variables matched as repeating at this depth"
#[macro_export]
macro_rules! fmt_internal_helper {
    ($e:expr) => {
        "{}"
    };
}

// with this, no need for all those format strings that are just "{}"
#[macro_export]
macro_rules! fmt_ {
    ($($e:expr),*) => {
        format!(
            concat!( $( fmt_internal_helper!($e), )* "" ),
            $($e),*
            )
    }
}
//TODO: make my own version of print!, write!, println!, writeln!...

// ---------------------------------------------------------------------------------------------------------------------

pub use std::collections::HashMap;
pub use std::hash::Hash;
pub use std::ops::Range;
pub use regex::Regex;
pub use std::collections::HashSet;

// does this exist in the std lib?
pub fn range<T>(vec: &Vec<T>) -> Range<usize> {
    0..vec.len()
}

// // does this exist in the std lib?
// pub fn count_where<T>(items: &[T], predicate: &dyn Fn(T) -> bool) {
//     items.iter().filter(predicate).count()
//}

// I somehow didn't find this in the std lib.
// itertools's group_by() only works on consecutive elements!..  misleading. should have been called "chunk_by" or something.
pub fn group_by<K, V, F>(items: &[V], get_key: F) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: Fn(&V) -> &K,
{
    let mut result: HashMap<K, Vec<V>> = HashMap::new();
    for item in items {
        let key = get_key(item);
        result.entry(key.clone()).or_insert(Vec::new()).push(item.clone());
    }
    result
}
