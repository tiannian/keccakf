use crate::Permutation;

macro_rules! define_keccak_state {
    ($ty:ident, $length:expr, $pty:ty, $kf:ident) => {
        #[derive(Clone, Debug)]
        #[repr(align(8))]
        /// State of keccak
        pub struct $ty([u8; 200]);

        impl Default for $ty {
            fn default() -> Self {
                Self([0u8; 200])
            }
        }

        impl Permutation for $ty {
            fn permute(&mut self) {
                let s = unsafe { &mut *(self as *mut Self as *mut [$pty; 25]) };

                crate::$kf(s);
            }
        }

        impl AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}

define_keccak_state!(Keccak1600State, 200, u64, keccakf1600);
define_keccak_state!(Keccak800State, 100, u32, keccakf800);
define_keccak_state!(Keccak400State, 50, u16, keccakf400);
define_keccak_state!(Keccak200State, 25, u8, keccakf200);
