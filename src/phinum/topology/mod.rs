//! Language topology — sentence and paragraph types as the spider net.

pub mod paragraph;
pub mod sentence;
pub mod spider_net;
#[cfg(test)]
mod tests;

pub use paragraph::ParagraphType;
pub use sentence::SentenceType;
pub use spider_net::LanguageSpiderNet;
