use serde::{Deserialize, Serialize};

/// Certification metadata returned by the certifications endpoints.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[expect(clippy::struct_field_names)]
pub struct Certification {
    pub certification: String,
    pub meanings: Vec<CertificationMeaning>,
    pub order: u16,
}

/// Certification meaning metadata.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CertificationMeaning {
    pub id: String,
    pub meaning: String,
    pub order: u16,
}

/// A full certifications payload grouped by region.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CertificationsPayload {
    pub certifications: std::collections::HashMap<String, Vec<Certification>>,
}
