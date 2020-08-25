#![warn(rust_2018_idioms)]


#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use pb::{
        Blob,
        Lazy,
        Message
    };
    use proto_pbtest::zero_copy::{
        BytesData,
        VecData,
    };
    use test::Bencher;

    #[bench]
    fn bench_deserialize_zero_copy_bytes(b: &mut Bencher) {
        // Generate 4MB of data 
        let data = Lazy::new(Bytes::from(vec![42 as u8; 4 * 1024 * 1024]));

        // Create our proto struct
        let mut proto = BytesData::default();
        proto.set_data(data);

        // Serialize the proto
        let ser_bytes: Vec<u8> = proto.serialize_to_vec();

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Convert our bytes::Bytes into a pb::BlobReader
            let mut bytes_reader = bytes_buf.clone().into_reader();

            // Deserialize our proto
            let mut de_proto = BytesData::default();
            de_proto.deserialize(&mut bytes_reader).unwrap();
            assert!(de_proto.has_data());
        });
    }

    #[bench]
    fn bench_deserialize_vec_bytes(b: &mut Bencher) {
        // Generate 4MB of data
        let data = vec![42 as u8; 4 * 1024 * 1024];

        // Create our proto struct
        let mut proto = VecData::default();
        proto.set_data(data);

        // Serialize the proto
        let ser_bytes: Vec<u8> = proto.serialize_to_vec();

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Convert our bytes::Bytes into a pb::BlobReader
            let mut bytes_reader = bytes_buf.clone().into_reader();

            // Deserialize our proto
            let mut de_proto = VecData::default();
            de_proto.deserialize(&mut bytes_reader).unwrap();
            assert!(de_proto.has_data());
        });
    }
}
