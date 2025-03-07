use {
    criterion::{criterion_group, criterion_main, Criterion},
    solana_bn254::prelude::{alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing},
};

fn bench_addition(c: &mut Criterion) {
    let p_bytes = [
        24, 177, 138, 207, 180, 194, 195, 2, 118, 219, 84, 17, 54, 142, 113, 133, 179, 17, 221, 18,
        70, 145, 97, 12, 93, 59, 116, 3, 78, 9, 61, 201, 6, 60, 144, 156, 71, 32, 132, 12, 181, 19,
        76, 185, 245, 159, 167, 73, 117, 87, 150, 129, 150, 88, 211, 46, 252, 13, 40, 129, 152,
        243, 114, 102,
    ];
    let q_bytes = [
        7, 194, 183, 245, 138, 132, 189, 97, 69, 240, 12, 156, 43, 192, 187, 26, 24, 127, 32, 255,
        44, 146, 150, 58, 136, 1, 158, 124, 106, 1, 78, 237, 6, 97, 78, 32, 193, 71, 233, 64, 242,
        215, 13, 163, 247, 76, 154, 23, 223, 54, 23, 6, 164, 72, 92, 116, 43, 214, 120, 132, 120,
        250, 23, 215,
    ];

    let input_bytes = [&p_bytes[..], &q_bytes[..]].concat();

    c.bench_function("bn128 addition", |b| {
        b.iter(|| alt_bn128_addition(&input_bytes))
    });
}

fn bench_multiplication(c: &mut Criterion) {
    let point_bytes = [
        43, 211, 230, 208, 243, 177, 66, 146, 79, 92, 167, 180, 156, 229, 185, 213, 76, 71, 3, 215,
        174, 86, 72, 230, 29, 2, 38, 139, 26, 10, 159, 183, 33, 97, 28, 224, 166, 175, 133, 145,
        94, 47, 29, 112, 48, 9, 9, 206, 46, 73, 223, 173, 74, 70, 25, 200, 57, 12, 174, 102, 206,
        253, 178, 4,
    ];
    let scalar_bytes = [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];

    let input_bytes = [&point_bytes[..], &scalar_bytes[..]].concat();

    c.bench_function("bn128 multiplication", |b| {
        b.iter(|| alt_bn128_multiplication(&input_bytes))
    });
}

fn bench_pairing(c: &mut Criterion) {
    let p_bytes = [
        28, 118, 71, 111, 77, 239, 75, 185, 69, 65, 213, 126, 187, 161, 25, 51, 129, 255, 167, 170,
        118, 173, 166, 100, 221, 49, 193, 96, 36, 196, 63, 89, 48, 52, 221, 41, 32, 246, 115, 226,
        4, 254, 226, 129, 28, 103, 135, 69, 252, 129, 155, 85, 211, 233, 210, 148, 228, 92, 155, 3,
        167, 106, 239, 65,
    ];
    let q_bytes = [
        32, 157, 209, 94, 191, 245, 212, 108, 75, 216, 136, 229, 26, 147, 207, 153, 167, 50, 150,
        54, 198, 53, 20, 57, 107, 74, 69, 32, 3, 163, 91, 247, 4, 191, 17, 202, 1, 72, 59, 250,
        139, 52, 180, 53, 97, 132, 141, 40, 144, 89, 96, 17, 76, 138, 192, 64, 73, 175, 75, 99, 21,
        164, 22, 120, 43, 184, 50, 74, 246, 207, 201, 53, 55, 162, 173, 26, 68, 92, 253, 12, 162,
        167, 26, 205, 122, 196, 31, 173, 191, 147, 60, 42, 81, 190, 52, 77, 18, 10, 42, 76, 243,
        12, 27, 249, 132, 95, 32, 198, 254, 57, 224, 126, 162, 204, 230, 31, 12, 155, 176, 72, 22,
        95, 229, 228, 222, 135, 117, 80,
    ];

    let input_bytes = [&p_bytes[..], &q_bytes[..]].concat();

    c.bench_function("bn128 pairing", |b| {
        b.iter(|| alt_bn128_pairing(&input_bytes))
    });
}

criterion_group!(benches, bench_addition, bench_multiplication, bench_pairing,);
criterion_main!(benches);
