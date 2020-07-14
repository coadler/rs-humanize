use iota::iota;

// IEC Sizes
// Binary bytes.
// 1024
iota! {
    pub const BYTE: u64 = 1 << (iota * 10);
        , KI_BYTE
        , MI_BYTE
        , GI_BYTE
        , TI_BYTE
        , PI_BYTE
        , EI_BYTE
}

// SI sizes.
// 1000
pub const K_BYTE: u64 = BYTE * 1000;
pub const M_BYTE: u64 = K_BYTE * 1000;
pub const G_BYTE: u64 = M_BYTE * 1000;
pub const T_BYTE: u64 = G_BYTE * 1000;
pub const P_BYTE: u64 = T_BYTE * 1000;
pub const E_BYTE: u64 = P_BYTE * 1000;

#[inline]
fn logn(n: f64, b: f64) -> f64 {
    n.ln() / b.ln()
}

#[inline]
fn _bytes(s: f64, base: f64, sizes: &[&str; 7]) -> String {
    if s < 10f64 {
        return format!("{} B", s);
    }

    let e = logn(s, base).floor() as u64;
    let suf = sizes[e as usize];
    let val = (s / (base as u64).pow(e as u32) as f64).floor();

    if val < 10f64 {
        return format!("{:.0} {}", val, suf);
    }

    format!("{:.1} {}", val, suf)
}

const SIZES: [&str; 7] = ["B", "kB", "MB", "GB", "TB", "PB", "EB"];

#[inline]
pub fn bytes(s: u64) -> String {
    _bytes(s as f64, 1000f64, &SIZES)
}

const ISIZES: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

#[inline]
pub fn ibytes(s: u64) -> String {
    _bytes(s as f64, 1024f64, &ISIZES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes() {
        struct BytesTest(u64, &'static str);

        let tests = vec![
            BytesTest(0, "0 B"),
            BytesTest(K_BYTE * 1, "1 kB"),
            BytesTest(K_BYTE * 5, "5 kB"),
            BytesTest(M_BYTE * 1, "1 MB"),
            BytesTest(M_BYTE * 5, "5 MB"),
            BytesTest(G_BYTE * 1, "1 GB"),
            BytesTest(G_BYTE * 5, "5 GB"),
            BytesTest(T_BYTE * 1, "1 TB"),
            BytesTest(T_BYTE * 5, "5 TB"),
            BytesTest(P_BYTE * 1, "1 PB"),
            BytesTest(P_BYTE * 5, "5 PB"),
            BytesTest(E_BYTE * 1, "1 EB"),
            BytesTest(E_BYTE * 5, "5 EB"),
        ];

        for test in tests.iter() {
            let fmted = bytes(test.0);
            assert_eq!(fmted, test.1);
        }
    }

    #[test]
    fn test_ibytes() {
        struct IBytesTest(u64, &'static str);

        let tests = vec![
            IBytesTest(0, "0 B"),
            IBytesTest(KI_BYTE * 1, "1 KiB"),
            IBytesTest(KI_BYTE * 5, "5 kiB"),
            IBytesTest(MI_BYTE * 1, "1 MiB"),
            IBytesTest(MI_BYTE * 5, "5 MiB"),
            IBytesTest(GI_BYTE * 1, "1 GiB"),
            IBytesTest(GI_BYTE * 5, "5 GiB"),
            IBytesTest(TI_BYTE * 1, "1 TiB"),
            IBytesTest(TI_BYTE * 5, "5 TiB"),
            IBytesTest(PI_BYTE * 1, "1 PiB"),
            IBytesTest(PI_BYTE * 5, "5 PiB"),
            IBytesTest(EI_BYTE * 1, "1 EiB"),
            IBytesTest(EI_BYTE * 5, "5 EiB"),
        ];

        for test in tests.iter() {
            let fmted = ibytes(test.0);
            assert_eq!(fmted, test.1);
        }
    }

    extern crate test;
    use rand::Rng;
    use test::Bencher;

    #[bench]
    fn bench_bytes(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let r: u64 = rng.gen();

        b.iter(|| bytes(r))
    }

    #[bench]
    fn bench_ibytes(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let r: u64 = rng.gen();

        b.iter(|| bytes(r))
    }
}
