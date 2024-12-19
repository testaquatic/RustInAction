use f32_parts::F32Parts;

mod f32_parts {
    const BIAS: i32 = 127;
    const RADIX: f32 = 2.0;

    #[derive(Clone)]
    pub struct F32Parts {
        sign: u32,
        exponent: u32,
        fraction: u32,
    }

    impl F32Parts {
        pub fn to_parts(n: f32) -> F32Parts {
            let bits = n.to_bits();
            let sign = (bits >> 31) & 0x1;
            let exponent = (bits >> 23) & 0xff;
            let fraction = bits & 0x7f_ffff;

            F32Parts {
                sign,
                exponent,
                fraction,
            }
        }

        pub fn sign(&self) -> u32 {
            self.sign
        }

        pub fn exponent(&self) -> u32 {
            self.exponent
        }

        pub fn fraction(&self) -> u32 {
            self.fraction
        }

        pub fn decode(&self) -> (f32, f32, f32) {
            let signed_1 = (-1.0_f32).powf(self.sign as f32);

            let exponent = (self.exponent as i32) - BIAS;
            let exponent = RADIX.powf(exponent as f32);

            let mantissa = 1.0
                + (0..23)
                    .map(|i| {
                        let mask = 1 << i;
                        if self.fraction & mask != 0 {
                            2_f32.powf(i as f32 - 23.0)
                        } else {
                            0.0
                        }
                    })
                    .sum::<f32>();

            (signed_1, exponent, mantissa)
        }

        pub fn to_f32(&self) -> f32 {
            let (sign, exponent, mantissa) = self.decode();

            sign * exponent * mantissa
        }
    }

    impl From<f32> for F32Parts {
        fn from(value: f32) -> F32Parts {
            F32Parts::to_parts(value)
        }
    }

    impl From<F32Parts> for f32 {
        fn from(value: F32Parts) -> Self {
            value.to_f32()
        }
    }
}

fn main() {
    let n = 42.42_f32;

    let f32_parts: F32Parts = n.into();
    let n_: f32 = f32_parts.clone().into();

    println!("{n} -> {n_}");
    println!("field     | as bits  | as real number");
    println!(
        "sign      |        {:01b} |              {}",
        f32_parts.sign(),
        f32_parts.decode().0
    );
    println!(
        "exponent  | {:08b} |             {}",
        f32_parts.exponent(),
        f32_parts.decode().1
    );
    println!(
        "mantissa  | {:23b} | {}",
        f32_parts.fraction(),
        f32_parts.decode().2
    );
}
