#[cfg(test)]
mod benches {
    use bytes::Bytes;
    use compact_str::CompactString;
    use pb_jelly::{
        Lazy,
        Message,
    };
    use proto_pbtest::bench::{
        BytesData,
        Cities,
        CitiesSSO,
        City,
        CitySSO,
        StringMessage,
        StringMessageSSO,
        VecData,
    };
    use serde::Deserialize;
    use std::{
        hint::black_box,
        io::Cursor,
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
            // Deserialize our proto
            let mut de_proto = BytesData::default();
            de_proto.deserialize(&mut Cursor::new(bytes_buf.clone())).unwrap();
            assert!(de_proto.has_data());
            de_proto
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
            // Deserialize our proto
            let mut de_proto = VecData::default();
            de_proto.deserialize(&mut Cursor::new(bytes_buf.clone())).unwrap();
            assert!(de_proto.has_data());
            de_proto
        });
    }

    #[bench]
    fn bench_deserialize_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        let mut proto = StringMessage::default();
        proto.set_data(data);

        let bytes: Vec<u8> = proto.serialize_to_vec();
        let bytes = Bytes::from(bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut de_proto = StringMessage::default();
            de_proto.deserialize(&mut Cursor::new(bytes.clone())).unwrap();
            black_box(de_proto)
        });
    }

    #[bench]
    fn bench_deserialize_string_sso(b: &mut Bencher) {
        let data = CompactString::from(include_str!("../data/moby_dick.txt"));

        let mut proto = StringMessageSSO::default();
        proto.set_data(data);

        let bytes: Vec<u8> = proto.serialize_to_vec();
        let bytes = Bytes::from(bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut de_proto = StringMessageSSO::default();
            de_proto.deserialize(&mut Cursor::new(bytes.clone())).unwrap();
            black_box(de_proto)
        });
    }

    #[bench]
    fn bench_deserialize_small_string(b: &mut Bencher) {
        let data = String::from("IMG_1234.png");

        let mut proto = StringMessage::default();
        proto.set_data(data);

        let bytes: Vec<u8> = proto.serialize_to_vec();
        let bytes = Bytes::from(bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut de_proto = StringMessage::default();
            de_proto.deserialize(&mut Cursor::new(bytes.clone())).unwrap();
            black_box(de_proto)
        });
    }

    #[bench]
    fn bench_deserialize_small_string_sso(b: &mut Bencher) {
        let data = CompactString::from("IMG_1234.png");

        let mut proto = StringMessageSSO::default();
        proto.set_data(data);

        let bytes: Vec<u8> = proto.serialize_to_vec();
        let bytes = Bytes::from(bytes);

        b.iter(|| {
            // Deserialize our proto
            let mut de_proto = StringMessageSSO::default();
            de_proto.deserialize(&mut Cursor::new(bytes.clone())).unwrap();
            black_box(de_proto)
        });
    }

    #[derive(Debug, Deserialize)]
    struct JsonCity {
        city: CompactString,
        growth_from_2000_to_2013: CompactString,
        latitude: f64,
        longitude: f64,
        population: CompactString,
        rank: CompactString,
        state: CompactString,
    }

    #[bench]
    fn bench_deserialize_many_strings(b: &mut Bencher) {
        let json = String::from(include_str!("../data/cities.json"));
        let json_cities: Vec<JsonCity> = serde_json::from_str(&json).expect("failed to parse cities.json");

        let cities: Vec<City> = json_cities
            .into_iter()
            .map(|city| City {
                city: Some(city.city.to_string()),
                growth_from_2000_to_2013: Some(city.growth_from_2000_to_2013.to_string()),
                latitude: Some(city.latitude),
                longitude: Some(city.longitude),
                population: Some(city.population.to_string()),
                rank: Some(city.rank.to_string()),
                state: Some(city.state.to_string()),
            })
            .collect();
        let proto = Cities { cities };

        let bytes = proto.serialize_to_vec();

        b.iter(|| {
            // Deserialize our proto
            let de_proto: Cities = Message::deserialize_from_slice(&bytes[..]).unwrap();
            black_box(de_proto)
        });
    }

    #[bench]
    fn bench_deserialize_many_strings_sso(b: &mut Bencher) {
        let json = String::from(include_str!("../data/cities.json"));
        let json_cities: Vec<JsonCity> = serde_json::from_str(&json).expect("failed to parse cities.json");

        let cities: Vec<CitySSO> = json_cities
            .into_iter()
            .map(|city| CitySSO {
                city: Some(city.city),
                growth_from_2000_to_2013: Some(city.growth_from_2000_to_2013),
                latitude: Some(city.latitude),
                longitude: Some(city.longitude),
                population: Some(city.population),
                rank: Some(city.rank),
                state: Some(city.state),
            })
            .collect();
        let proto = CitiesSSO { cities };

        let bytes = proto.serialize_to_vec();

        b.iter(|| {
            // Deserialize our proto
            let de_proto: CitiesSSO = Message::deserialize_from_slice(&bytes[..]).unwrap();
            black_box(de_proto)
        });
    }
}

#[cfg(all(test, feature = "bench_prost"))]
mod prost {
    use prost::bytes::Bytes;
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
        StringMessage,
    };
    use bytes::Bytes;
    use protobuf::{
        CodedInputStream,
        Message,
    };
    use test::Bencher;

    #[bench]
    fn bench_deserialize_rust_protobuf_bytes(b: &mut Bencher) {
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
    fn bench_deserialize_rust_protobuf_string(b: &mut Bencher) {
        let data = String::from(include_str!("../data/moby_dick.txt"));

        // Create our proto struct
        let mut proto = StringMessage::new();
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
            let mut de_proto = StringMessage::default();
            de_proto
                .merge_from(&mut input_stream)
                .expect("failed to decode rust_protobuf proto!");
            assert!(de_proto.has_data());
        });
    }
}
