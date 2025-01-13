use std::str::FromStr;

pub(super) enum Mime {
    Pdf,
    Docx,
    Csv,
    Excel,
}

/// Reference:
///     <https://www.runoob.com/http/mime-types.html>
impl FromStr for Mime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/pdf" => Ok(Mime::Pdf),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                Ok(Mime::Docx)
            }
            "text/csv" => Ok(Mime::Csv),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Ok(Mime::Excel),
            "application/vnd.ms-excel" => Ok(Mime::Excel),
            _ => Err(()),
        }
    }
}
