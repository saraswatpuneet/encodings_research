pub trait Encoding {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Vec<u8>;
}

pub struct EncodingMetrics {
    pub encoded_size: usize,
    pub decoding_time: f64,
}

impl Encoding {
    pub fn get_metrics(&self, data: &[u8]) -> EncodingMetrics {
        let start = std::time::Instant::now();
        let encoded = self.encode(data);
        let encoding_time = start.elapsed();

        let start = std::time::Instant::now();
        let decoded = self.decode(&encoded);
        let decoding_time = start.elapsed();

        EncodingMetrics {
            encoded_size: encoded.len(),
            decoding_time: decoding_time.as_secs_f64(),
        }
    }
}
