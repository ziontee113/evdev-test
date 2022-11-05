# Practical Ideas:

## Holding down a set of keys acts like a single "modifier"
- Powered by time calculation between `value 1`s.
- We can call this "Union Trigger", which a layer is triggered
using multiple keys pressed down together within a set of interval.

# Short Term Todos:
- Convert ev.timestamp() into human readable time.
- Apply ev.timestamp() with multiple threads 😬

## What is "now"?
- after pushing new `alias_and_code`, if `keypress_vector.len()` == `1`
--> Mutate `now`
