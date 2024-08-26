#[derive(serde::Deserialize)]
pub struct ArtworkRaw {
  #[serde(rename="ObjectID")]
  object_id: usize,
  #[serde(rename="Cataloged")]
  cataloged: Option<String>,
  #[serde(rename="Title")]
  title: Option<String>,
  #[serde(rename="Artist")]
  artist: Vec<String>,
  #[serde(rename="ConstituentID")]
  constituent_id: Vec<usize>,
  #[serde(rename="Date")]
  date: Option<String>,
  #[serde(rename="Medium")]
  medium: Option<String>,
  #[serde(rename="Dimensions")]
  dimensions: Option<String>,
  #[serde(rename="CreditLine")]
  credit_line: Option<String>,
  #[serde(rename="AccessionNumber")]
  accession_number: Option<String>,
  #[serde(rename="Classification")]
  classification: Option<String>,
  #[serde(rename="Department")]
  department: Option<String>,
  #[serde(rename="DateAcquired")]
  date_acquired: Option<String>,
  #[serde(rename="URL")]
  url: Option<String>,
  #[serde(rename="ImageURL")]
  image_url: Option<String>,
  #[serde(rename="Height (cm)")]
  height_cm: Option<f32>,
  #[serde(rename="Width (cm)")]
  width_cm: Option<f32>,
}

#[cfg(test)]
impl ArtworkRaw {
  pub fn make(
    object_id: usize,
    title: &str,
    artist: Vec<&str>,
    constituent_id: Vec<usize>,
  ) -> ArtworkRaw {
    ArtworkRaw {
      object_id,
      cataloged: None,
      title: Some(title.to_string()),
      artist: artist.iter().map(|a| a.to_string()).collect(),
      constituent_id,
      date: None,
      medium: None,
      dimensions: None,
      credit_line: None,
      accession_number: None,
      classification: None,
      department: None,
      date_acquired: None,
      url: None,
      image_url: None,
      height_cm: None,
      width_cm: None,
    }
  }
}

impl ArtworkRaw {
  pub fn as_artwork(&self) -> Artwork {
    Artwork {
      object_id: self.object_id,
      cataloged: self.cataloged.clone(),
      title: self.title.clone(),
      date: self.date.clone(),
      medium: self.medium.clone(),
      dimensions: self.dimensions.clone(),
      credit_line: self.credit_line.clone(),
      accession_number: self.accession_number.clone(),
      classification: self.classification.clone(),
      department: self.department.clone(),
      date_acquired: self.date_acquired.clone(),
      url: self.url.clone(),
      image_url: self.image_url.clone(),
      height_cm: self.height_cm,
      width_cm: self.width_cm,
    }
  }

  pub fn as_artwork_attribution(&self) -> Vec<ArtworkAttribution> {
    self.constituent_id.iter()
    .zip(self.artist.iter())
    .map(|(constituent_id, artist)| ArtworkAttribution {
      object_id: self.object_id,
      constituent_id: constituent_id.clone(),
      artist: artist.clone(),
    })
    .collect()
  }
}

#[derive(derive_sql::DeriveSqlStatement)]
pub struct Artwork {
  object_id: usize,
  cataloged: Option<String>,
  title: Option<String>,
  date: Option<String>,
  medium: Option<String>,
  dimensions: Option<String>,
  credit_line: Option<String>,
  accession_number: Option<String>,
  classification: Option<String>,
  department: Option<String>,
  date_acquired: Option<String>,
  url: Option<String>,
  image_url: Option<String>,
  height_cm: Option<f32>,
  width_cm: Option<f32>,
}

#[derive(derive_sql::DeriveSqlStatement)]
pub struct ArtworkAttribution {
  object_id: usize,
  constituent_id: usize,
  artist: String,
}


