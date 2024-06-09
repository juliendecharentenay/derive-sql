#[derive(Debug)]
#[derive(derive_sql::DeriveSqlite)]
#[derive(serde::Deserialize)]
pub struct Artist {
  #[serde(rename="ConstituentID")]
  constituent_id: usize,
  #[serde(rename="DisplayName")]
  display_name: String,
  #[serde(rename="ArtistBio")]
  artist_bio: Option<String>,
  #[serde(rename="Nationality")]
  nationality: Option<String>,
  #[serde(rename="Gender")]
  gender: Option<String>,
  #[serde(rename="BeginDate")]
  begin_date: usize,
  #[serde(rename="EndDate")]
  end_date: usize,
  #[serde(rename="Wiki QID")]
  wiki_qid: Option<String>,
  #[serde(rename="ULAN")]
  ulan: Option<String>,
}

