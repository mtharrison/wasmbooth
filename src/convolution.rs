pub type ConvolutionMatrix = [[f32; 3]; 3];

pub static EDGE_DETECT: ConvolutionMatrix = [
    [-1.0, -1.0, -1.0], //
    [-1.0, 8.0, -1.0],  //
    [-1.0, -1.0, -1.0], //
];

pub static SHARPEN: ConvolutionMatrix = [
    [0.0, -1.0, 0.0],  //
    [-1.0, 5.0, -1.0], //
    [0.0, -1.0, 0.0],  //
];

pub static BLUR: ConvolutionMatrix = [
    [0.0625, 0.125, 0.0625],
    [0.125, 0.25, 0.125],
    [0.0625, 0.125, 0.0625],
];

pub static EMBOSS: ConvolutionMatrix = [
    [-2.0, -1.0, 0.0], //
    [-1.0, 1.0, 1.0],  //
    [0.0, 1.0, 2.0],   //
];

pub fn apply_convolution(m1: [u8; 9], m2: ConvolutionMatrix) -> u8 {
    let mut accum: f32 = 0.0;

    for i in 0..3 {
        for j in 0..3 {
            accum += (m1[(i * 3) + j] as f32) * (m2[i][j]);
        }
    }

    if accum < 0.0 {
        return 0;
    }

    if accum > 255.0 {
        return 255;
    }

    return accum as u8;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_apply_convolution() {
        assert_eq!(
            apply_convolution(
                [
                    1, 2, 3, //
                    4, 5, 6, //
                    7, 8, 9  //
                ],
                EDGE_DETECT
            ),
            0
        );

        assert_eq!(
            apply_convolution(
                [
                    5, 5, 5, //
                    5, 6, 5, //
                    5, 5, 5  //
                ],
                EDGE_DETECT
            ),
            8
        );

        assert_eq!(
            apply_convolution(
                [
                    0, 0, 0, //
                    0, 6, 0, //
                    0, 0, 0  //
                ],
                EDGE_DETECT
            ),
            48
        );
    }
}
