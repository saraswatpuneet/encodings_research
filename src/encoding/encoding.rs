pub trait Encoding {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Vec<u8>;
}

pub struct EncodingMetrics {
    pub encoded_size: usize,
    pub decoding_time: f64,
    pub compression_ratio: f64,
    pub encoding_time: f64,
}

impl Encoding {
    pub fn get_metrics(&self, data: &[u8]) -> EncodingMetrics {
        let start = std::time::Instant::now();
        let encoded = self.encode(data);
        let encoding_time = start.elapsed();

        let start = std::time::Instant::now();
        let decoded = self.decode(&encoded);
        let decoding_time = start.elapsed();

        let encoded_size = encoded.len();

        let compression_ratio = encoded_size as f64 / data.len() as f64;

        let encoding_time = encoding_time.as_secs_f64();
        let decoding_time = decoding_time.as_secs_f64();

        EncodingMetrics {
            encoded_size,
            decoding_time,
            compression_ratio,
            encoding_time,
        }
    }
}
