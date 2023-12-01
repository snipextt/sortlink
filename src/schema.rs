use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shortlink {
    pub shortlink: String,
    pub link: String,
}

rbatis::crud!(Shortlink {}, "shortlinks");
rbatis::impl_select!(Shortlink{find_shortlinks(shortlink: &str) -> Option => "`where shortlink = #{shortlink} limit 1`"}, "shortlinks");
