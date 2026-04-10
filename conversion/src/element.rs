/// Extension traits for element-wise conversion of collections via [`Into`].
pub trait ElementInto<CT> {
    /// Convert each element of a collection into a new type.
    fn element_into(self) -> CT;
}
