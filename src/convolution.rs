pub type ConvolutionMatrix = [f32; 9];

pub static EDGE_DETECT: ConvolutionMatrix = [-1_f32, -1_f32, -1_f32, -1_f32, 8_f32, -1_f32, -1_f32, -1_f32, -1_f32];
pub static SHARPEN: ConvolutionMatrix = [0_f32, -1_f32, 0_f32, -1_f32, 5_f32, -1_f32, 0_f32, -1_f32, 0_f32];
pub static BLUR: ConvolutionMatrix = [0.0625, 0.125, 0.0625, 0.125, 0.25, 0.125, 0.0625, 0.125, 0.0625];
pub static EMBOSS: ConvolutionMatrix = [-2_f32, -1_f32, 0_f32, -1_f32, 1_f32, 1_f32, 0_f32, 1_f32, 2_f32];

pub fn apply_convolution(m1: [u8; 9], m2: ConvolutionMatrix) -> u8 {
    let mut accum: f32 = 0_f32;

    for i in 0..9 {
        accum += (m1[i] as f32) * (m2[i]);
    }

    if accum < 0_f32 {
        return 0;
    }

    if accum > 255_f32 {
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
            apply_convolution([1, 2, 3, 4, 5, 6, 7, 8, 9], EDGE_DETECT),
            30
        );
    }
}
