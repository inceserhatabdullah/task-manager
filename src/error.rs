use crate::domain::TaskId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("görev bulunamadı: #{0}")]
    NotFound(TaskId),

    #[error("geçersiz girdi: {0}")]
    InvalidInput(String),

    #[error("dosya hatası: {0}")]
    Io(#[from] std::io::Error),

    #[error("veri dosyası bozuk veya okunamadı: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, TaskError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found() {
        let err = TaskError::NotFound(TaskId::new(1));
        assert_eq!(err.to_string(), "görev bulunamadı: #1");
    }

    #[test]
    fn invalid_input_error() {
        let err = TaskError::InvalidInput("girdi boş olamaz".to_string());
        assert_eq!(err.to_string(), "geçersiz girdi: girdi boş olamaz");
    }

    #[test]
    fn io_error() {
        fn read() -> Result<()> {
            std::fs::read_to_string("/no/found/file")?;
            Ok(())
        }

        let err = read().unwrap_err();
        assert!(matches!(err, TaskError::Io(_)));
    }

    #[test]
    fn json_error() {
        fn read() -> Result<()> {
            let _: Vec<u8> = serde_json::from_str("bozuk json")?;
            Ok(())
        }

        let err = read().unwrap_err();
        assert!(matches!(err, TaskError::Serialization(_)));
    }
}
