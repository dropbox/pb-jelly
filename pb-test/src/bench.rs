#[cfg(test)]
mod benches {
    use bytes::Bytes;
    use pb_jelly::{
        Lazy,
        Message,
        PbBuffer,
    };
    use proto_pbtest::bench::{
        BytesData,
        StringMessage,
        VecData,
        ZeroCopyStringMessage,
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
            // Convert our bytes::Bytes into a pb::PbBufferReader
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
            // Convert our bytes::Bytes into a pb::PbBufferReader
            let mut bytes_reader = bytes_buf.clone().into_reader();

            // Deserialize our proto
            let mut de_proto = VecData::default();
            de_proto.deserialize(&mut bytes_reader).unwrap();
            assert!(de_proto.has_data());
        });
    }

    #[bench]
    fn bench_deserialize_zero_copy_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        let mut proto = ZeroCopyStringMessage::default();
        proto.set_data(data.into());

        let ser_bytes: Vec<u8> = proto.serialize_to_vec();

        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Convert our bytes::Bytes into a pb::PbBufferReader
            let mut bytes_reader = bytes_buf.clone().into_reader();

            // Deserialize our proto
            let mut de_proto = ZeroCopyStringMessage::default();
            de_proto.deserialize(&mut bytes_reader).unwrap();
            assert!(de_proto.has_data());
        });
    }

    #[bench]
    fn bench_deserialize_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        let mut proto = StringMessage::default();
        proto.set_data(data);

        let ser_bytes: Vec<u8> = proto.serialize_to_vec();

        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Convert our bytes::Bytes into a pb::PbBufferReader
            let mut bytes_reader = bytes_buf.clone().into_reader();

            // Deserialize our proto
            let mut de_proto = StringMessage::default();
            de_proto.deserialize(&mut bytes_reader).unwrap();
            assert!(de_proto.has_data());
        });
    }
}

#[cfg(all(test, feature = "bench_prost"))]
mod prost {
    use bytes::Bytes;
    use prost::Message;
    use test::Bencher;

    mod gen {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/prost/pbtest.rs"));
    }

    #[bench]
    fn bench_deserialize_prost_bytes(b: &mut Bencher) {
        // Generate 4MB of data
        let data = vec![42 as u8; 4 * 1024 * 1024];

        // Create our proto struct
        let mut proto = gen::BytesData::default();
        proto.data = Some(data);

        // Serialize the proto
        let csz = proto.encoded_len();
        let mut ser_bytes = Vec::with_capacity(csz);
        proto.encode(&mut ser_bytes).expect("failed to encode PROST proto!");

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes struct, this implements bytes::Buf
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Deserialize our proto
            let de_proto = gen::BytesData::decode(bytes_buf.clone()).expect("failed to decode PROST proto!");
            assert!(de_proto.data.is_some());
        });
    }

    #[bench]
    fn bench_deserialize_prost_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        // Create our proto struct
        let mut proto = gen::StringMessage::default();
        proto.data = Some(data);

        // Serialize the proto
        let csz = proto.encoded_len();
        let mut ser_bytes = Vec::with_capacity(csz);
        proto.encode(&mut ser_bytes).expect("failed to encode PROST proto!");

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes struct, this implements bytes::Buf
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Deserialize our proto
            let de_proto = gen::StringMessage::decode(bytes_buf.clone()).expect("failed to decode PROST proto!");
            assert!(de_proto.data.is_some());
        });
    }
}

#[cfg(all(test, feature = "bench_rust_protobuf"))]
mod rust_protobuf {
    use crate::gen::rust_protobuf::bench::{
        BytesData,
        ZeroCopyStringMessage,
    };
    use bytes::Bytes;
    use protobuf::{
        Chars,
        CodedInputStream,
        Message,
    };
    use test::Bencher;

    #[bench]
    fn bench_deserialize_rust_protobuf_zero_copy_bytes(b: &mut Bencher) {
        // Generate 4MB of data
        let data = Bytes::from(vec![42 as u8; 4 * 1024 * 1024]);

        // Create our proto struct
        let mut proto = BytesData::new();
        proto.set_data(data);

        // Serialize the proto
        let csz = proto.compute_size();
        let mut ser_bytes = Vec::with_capacity(csz as usize);
        proto
            .write_to_vec(&mut ser_bytes)
            .expect("failed to encode rust_protobuf proto!");

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes struct, this implements bytes::Buf
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut input_stream = CodedInputStream::from_carllerche_bytes(&bytes_buf);
            let mut de_proto = BytesData::default();
            de_proto
                .merge_from(&mut input_stream)
                .expect("failed to decode rust_protobuf proto!");
            assert!(de_proto.has_data());
        });
    }

    #[bench]
    fn bench_deserialize_rust_protobuf_zero_copy_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        // Create our proto struct
        let mut proto = ZeroCopyStringMessage::new();
        proto.set_data(Chars::from(data));

        // Serialize the proto
        let csz = proto.compute_size();
        let mut ser_bytes = Vec::with_capacity(csz as usize);
        proto
            .write_to_vec(&mut ser_bytes)
            .expect("failed to encode rust_protobuf proto!");

        // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

        // Read our serialized bytes into a Bytes struct, this implements bytes::Buf
        let bytes_buf = Bytes::from(ser_bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut input_stream = CodedInputStream::from_carllerche_bytes(&bytes_buf);
            let mut de_proto = ZeroCopyStringMessage::default();
            de_proto
                .merge_from(&mut input_stream)
                .expect("failed to decode rust_protobuf proto!");
            assert!(de_proto.has_data());
        });
    }
}
