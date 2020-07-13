#![warn(rust_2018_idioms)]

extern crate test;

extern crate blob;
extern crate blob_pb;
extern crate pb;
extern crate proto_mp;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use test::{
        black_box,
        Bencher,
    };

    use blob::Blob;
    use blob_pb::{
        BlobReaderImpl,
        WrappedBlob,
    };
    use pb::{
        Lazy,
        Message,
    };

    use proto_mp::osd::{
        OsdGetStripesResponse,
        OsdStripe,
    };

    #[bench]
    fn bench_protos(b: &mut Bencher) {
        // 18 stripes (256kb each)
        let data = Lazy::new(WrappedBlob(Blob::from_vec(vec![0 as u8; 256 * 1024])));

        let mut proto = OsdGetStripesResponse::default();
        for idx in 0..18 {
            let mut stripe = OsdStripe::default();
            stripe.set_number(idx as u32);
            stripe.set_data(data.clone());
            proto.mut_stripes().push(stripe);
        }

        let csz = proto.compute_size();
        let mut buf = vec![0 as u8; csz as usize];

        b.iter(|| {
            let mut w = Cursor::new(&mut buf[..]);
            proto.serialize(&mut w).unwrap();
        });
    }

    #[bench]
    fn vec_u8_deserialize(b: &mut Bencher) {
        let data_blob = Blob::from_vec(vec![0 as u8; 1024 * 4]);
        b.iter(|| {
            let mut reader: BlobReaderImpl = data_blob.clone().into();
            let mut decoded: Vec<u8> = Vec::with_capacity(0);
            Message::deserialize(&mut decoded, &mut reader).unwrap();
            black_box(&decoded);
        });
    }

    #[bench]
    fn vec_copy(b: &mut Bencher) {
        let data = vec![0 as u8; 1024 * 4];
        b.iter(|| {
            let mut decoded: Vec<u8> = Vec::with_capacity(1024 * 4);
            unsafe {
                decoded.set_len(1024 * 4);
            }
            decoded.copy_from_slice(&data);
            black_box(&decoded);
        });
    }
}
