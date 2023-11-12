use crate::args::DisplayOptions;

pub enum DisplayOptionEnum {
    OneLine,
    Long,
    Grid,
    Tree,
}

impl From<&DisplayOptions> for DisplayOptionEnum {
    fn from(value: &DisplayOptions) -> Self {
        if value.one_line {
            return Self::OneLine;
        } else if value.long {
            return Self::Long;
        } else if value.grid {
            return Self::Grid;
        } else if value.tree {
            return Self::Tree;
        }

        Self::Grid
    }
}
