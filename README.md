# Design

## Components

File | Description
--- | ---
errors | contains VersionitisError, which implements Failure
interval | A generic struct (Interval<T>) representing an interval of some sort
interval_map | contains IntervalMap, which maps a String to an Interval<VersionNumber>
package::owned | a versioned package which owns its data
package::reference | a versioned package which referenes its data
manifest | a package with an interval_map of dependencies
package_map | store a vector of packages and a map of package name,indices (not used)
range | contains Range enum which is used to characterize input intervals
repo | store a map of packages (not package_map)
traits | like it sounds... like it sounds
vernum_interval_parser | parse an Interval<VersionNumber> from a str
version_number_interval | Interval<VersionNumber> implementation
version_number | encode semantics of a version number - a dot separated list of u16

## Improved Efficiency
### Arena
An arena will allow us to reduce heap allocations. Arenas work by storing a vector of owned data and handing out references to said data. As an example, one may keep an arena of package versions, and maintain reference semantics throughout the rest of the package.

We will store the entity with ownership semantics in an arena and reference into that arena
throughout. Based on initial tests, we can do something like the following, where PackageVersion is the owned name (eg foo-1.2.3):

```rust
use typed_arena::Arena;
use std::collections::HashMap;

pub type PackageVersion = String;
pub type PackageArena = Arena<PackageVersion>;
pub type _PackageMap<'a> = HashMap<&'a str, PackageVersion<'a>>;

pub struct PackageMap<'a, 'b: 'a> {
    arena: &'b PackageArena,
    map: _PackageMap<'a>,
}

impl<'a, 'b> PackMap<'a, 'b> {
    /// Arena must be passed into constructor in order for borrow
    /// checker to be satisifed. We cannot have sibling references.
    pub fn new(arena: &'b PackageArena) -> Self {
        Self {
            arena,
            map: _PackageMap::new(),
        }
    }

    pub fn add< I: Into<String>>(&mut self, package: I) {
        let v1: &'b str = self.arena.alloc(package.into());
        let p1 = PackageVersion::new(v1);
        self.map.insert(v1, p1);
    }

    pub fn get(&self, name: &str) -> Option<&PackageVersion<'a>> {
        self.map.get(name)
    }
}
```
- Package and Manifest would not own their strings. They would operate on &str
- versionless package names would constructed as a slice of a versioned name
## Interval expansion
In order to facilitate shorthand intervals in yaml, we need to implement spec expansion in intervals during equality checking and ordering. What this means is:
```rust
foo-1 == foo-1.0.0
foo-1 < foo-1.0.1
```

## PackageInterval from string
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
fred = 1.3.2<=3.0.0 | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[3,0,0]}}
fred = 1.3.2^ | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[2,0,0]}}
fred = 1.3.2^2 | HalfOpen { start: Package{name:fred, spec:[1,3,2]}, end: Package{name:fred, spec:[3,0,0]}}