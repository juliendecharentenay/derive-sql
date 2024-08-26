#[derive(Debug)]
#[derive(derive_sql::DeriveSqlStatement)]
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

#[cfg(test)]
impl Artist {
  pub fn make(
    constituent_id: usize,
    display_name: &str,
    artist_bio: &str,
    nationality: &str,
    begin_date: usize,
    end_date: usize,
  ) -> Artist {
    Artist {
      constituent_id,
      display_name: display_name.to_string(),
      artist_bio: Some(artist_bio.to_string()),
      nationality: Some(nationality.to_string()),
      gender: None,
      begin_date,
      end_date,
      wiki_qid: None,
      ulan: None,
    }
  }
}

