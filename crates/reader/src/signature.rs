// TODO: split the parsing code from teh gen code

use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Signature {
    pub kind: ElementType,
    pub pointers: usize,
    pub by_ref: bool,
    pub is_const: bool,
    pub is_array: bool,
}

impl Signature {
    pub fn definition(&self, include: TypeInclude) -> Vec<TypeEntry> {
        self.kind.definition(include)
    }

    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        self.kind.dependencies(include)
    }

    pub fn is_blittable(&self) -> bool {
        self.pointers > 0 || self.kind.is_blittable()
    }

    pub fn is_udt(&self) -> bool {
        self.pointers == 0 && self.kind.is_udt()
    }

    pub fn has_explicit(&self) -> bool {
        self.pointers == 0 && self.kind.has_explicit()
    }

    pub fn is_packed(&self) -> bool {
        if self.pointers > 0 {
            return false;
        }

        match &self.kind {
            ElementType::TypeDef(def) => def.is_packed(),
            ElementType::Array((signature, _)) => signature.is_packed(),
            _ => false,
        }
    }
}
