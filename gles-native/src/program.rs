#[derive(Debug, Default)]
pub(crate) struct Program {
    pub deletion_mark: bool,
    pub validate_state: bool,

    pub update_descriptor_sets: bool,
}
