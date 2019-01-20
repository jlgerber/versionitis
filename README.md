# Design
- repo stores all packages and their versions
- manifest stores package version and dependency rnges
- package_map stores array of ints corresponding with SAT solver requirements

# Interval expansion
In order to facilitate shorthand intervals in yaml, we need to implement spec expansion in intervals during equality checking and ordering. What this means is:
```rust
foo-1 == foo-1.0.0
foo-1 < foo-1.0.1
```

# PackageInterval from string
Intervals are modeled as an enum generic over T.
```rust
enum Interval<T> {
    Single(T),
    Open{start:T, end:T},
    HalfOpen{start:T, end:T}
}
```

Principally, T is defined as Package. We need to define Interval as generic primarily because there are currently two different implemntations of package - owned and referenced. These versions differ with respect to ownership of their string contents. When serializing an Interval, the results are quite verbose:
```yaml
interval:
    open:
       start: foo-1.0.0
       end: foo-2.0.0
```
Mind you, this is already cleaned up, as we have a custom serialize for package and interval. However, we can do better. What we wnt to be able to do is define the serialized version of interval to be something like:

yaml | rust
--- | ---
fred = 1.0.0 | Single(Package{name:fred, spec:[1,0,0]})
fred = 1.3.2=<2.0.0 | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[3,0,0]}}
fred = 1.3.2^ | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[2,0,0]}}
fred = 1.3.2^2 | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[3,0,0]}}