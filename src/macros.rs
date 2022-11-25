macro_rules! keccakf {
    (
        $type: ty,
        $loop: expr,
        $state: expr,
        $RHO: expr,
        $PI: expr,
        $RC: expr
    ) => {
        {
            for i in 0..$loop {
                let mut array: [$type; 5] = [0; 5];

                // Theta
                unroll! {
                    for x in 0..5 {
                        unroll! {
                            for y_count in 0..5 {
                                let y = y_count * 5;
                                array[x] ^= $state[x + y];
                            }
                        }
                    }
                }

                unroll! {
                    for x in 0..5 {
                        unroll! {
                            for y_count in 0..5 {
                                let y = y_count * 5;
                                $state[y + x] ^= array[(x + 4) % 5] ^ array[(x + 1) % 5].rotate_left(1);
                            }
                        }
                    }
                }

                // Rho and pi
                let mut last = $state[1];
                unroll! {
                    for x in 0..24 {
                        array[0] = $state[$PI[x]];
                        $state[$PI[x]] = last.rotate_left($RHO[x]);
                        last = array[0];
                    }
                }

                // Chi
                unroll! {
                    for y_step in 0..5 {
                        let y = y_step * 5;

                        unroll! {
                            for x in 0..5 {
                                array[x] = $state[y + x];
                            }
                        }

                        unroll! {
                            for x in 0..5 {
                                $state[y + x] = array[x] ^ ((!array[(x + 1) % 5]) & (array[(x + 2) % 5]));
                            }
                        }
                    }
                };

                // Iota
                $state[0] ^= $RC[i];
            }
        }
    };
}

pub(crate) use keccakf;
