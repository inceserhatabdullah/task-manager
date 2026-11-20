//! task manager kütüphanesi
//! 
//! Katmanlar:
//! - `domain`: saf veri modelleri
//! - `service`: iş mantığı
//! - `storage`: repository
//! - `cli`: komut satırı arayüz
//! - `error`: uygulama hataları

pub mod cli;
pub mod domain;
pub mod error;
pub mod service;
pub mod storage;


