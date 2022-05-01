/// A module containing operations for working with integer functions as
/// presented in the paper "Counting subgraph patterns in large graphs" by
/// Emil Ruhwald Nielsen, Otto Stadel Clausen and Elisabeth Terp Reeve.
pub mod integer_functions {
    use std::collections::HashMap;

    /// Defining the type Mapping to distinguish the operation from normal u64 variables.
    pub type Mapping = u64;

    /// Given the integer function f of basis n. Apply returns the digit with significance s.
    /// This is achieved by by shifting all digits s positions to the right and then take the rest
    /// of the division by n which removes the least significant digit.
    pub fn apply(n : Mapping, f : Mapping, s : Mapping) -> Mapping{
        ( f / (n.pow(s as u32) as u64) ) % n
    }

    /// Given the integer function f of basis n. Extend increases the number of digits by one.
    /// This will be done by shifting all digits with significance higher than s one position
    /// to the left(increase their significance by one). Then the digit with significance s will
    /// be set to v
    pub fn extend(n : Mapping, f : Mapping, s : Mapping, v : Mapping) -> Mapping{
        let r = f % (n.pow(s as u32) as Mapping);
        let l = f - r;
        (n * l) + (n.pow(s as u32) as Mapping) * v + r
    }

    /// Given the integer function f of basis n. Reduce decreases the number of digits by one.
    /// This will be done by deleting the digit with significance s and then shifting all digits
    /// with higher significance one to the right (decrease their significance by one).
    pub fn reduce(n : Mapping, f : Mapping, s : Mapping) -> Mapping{
        let r = f % n.pow(s as u32);
        let l = f - (f % n.pow((s + 1) as u32));
        (l / n) + r
    }

    /// Returns the maximal amount of mappings from a set of d elements to
    /// a set of n elements. This mappings can be represented by the integers
    /// {0,1,...,max_mapping - 1}
    pub fn max_mappings(d : Mapping, n : Mapping) -> Mapping{
        n.pow(d as u32)
    }

    /// Takes an mapping f to the base n as input and returns the mapping as a hashmap
    pub fn to_hashmap(n : Mapping, f : Mapping) -> HashMap<Mapping,Mapping>{
        let mut mapping = HashMap::new();

        let mut rest = f;
        let mut pos = 0;

        // this follows the simple iterative method of getting the representation of the number f
        // to the basis of n
        // see also: https://www.ics.uci.edu/~irani/w17-6D/BoardNotes/12_NumberRepresentationPost.pdf
        while rest > 0 {
            mapping.insert(pos, rest % n);
            pos += 1;
            rest = rest / n;
        }

        mapping
    }
}
