use clap::{Args as ClapArgs, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    // TODO : Optimize this later
    #[arg(default_value_t = String::from("."), help = "Path to scan")]
    pub dir: String,

    #[command(flatten)]
    pub display_options: DisplayOptions,

    #[command(flatten)]
    pub sorting_filtering_options: SortingFilteringOptions,
}

#[derive(ClapArgs, Debug)]
#[command(next_help_heading = "Display Options")]
pub struct DisplayOptions {
    #[arg(
        short = '1',
        long = "oneline",
        default_value_t = false,
        help = "Display one entry per line"
    )]
    pub one_line: bool,

    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        help = "Display extended file metadata as table"
    )]
    pub long: bool,

    #[arg(
        short = 'G',
        long,
        default_value_t = false,
        help = "Display entries as grid (default)"
    )]
    pub grid: bool,

    #[arg(
        short = 'T',
        long,
        default_value_t = false,
        help = "Recurse into directories as tree"
    )]
    pub tree: bool,

    #[arg(long, default_value_t = false, help = "Display entries as hyperlink")]
    pub hyperlink: bool,
}

#[derive(ClapArgs, Debug)]
#[command(next_help_heading = "Sorting and Filtering Options")]
pub struct SortingFilteringOptions {
    #[arg(
        short = 'a',
        long,
        default_value_t = false,
        help = "Show hidden and `dot` files"
    )]
    pub all: bool,

    #[arg(
        short = 'A',
        long,
        default_value_t = false,
        help = "For compability purpose"
    )]
    pub almost_all: bool,

    #[arg(short = 'D', long, help = "List only directories")]
    pub only_dirs: bool,

    #[arg(short = 'f', long, help = "List only files")]
    pub only_files: bool,

    #[arg(
        short = 'r',
        long,
        default_value_t = false,
        help = "Reverse the sort order"
    )]
    pub reverse: bool,
}
