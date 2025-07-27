use lazy_static::lazy_static;
use time::format_description;
use time::format_description::BorrowedFormatItem;

lazy_static! {
    pub static ref DATE_FORMATTER: Vec<BorrowedFormatItem<'static>> =
        format_description::parse("[year]-[month]-[day]").expect("Invalid format description");
}
