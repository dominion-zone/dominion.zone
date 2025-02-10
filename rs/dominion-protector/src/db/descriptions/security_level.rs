use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "security_level")]
pub enum SecurityLevel {
    #[postgres(name = "Critical Risk")]
    CriticalRisk,
    #[postgres(name = "High Risk")]
    HighRisk,
    #[postgres(name = "Medium Risk")]
    MediumRisk,
    #[postgres(name = "Low Risk")]
    LowRisk,
    #[postgres(name = "Best Practices Compliant")]
    BestPracticesCompliant,
    #[postgres(name = "Unknown / Unassessed")]
    UnknownUnassessed,
}