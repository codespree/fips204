//! This file implements functionality from FIPS 204 section 8.5 `NTT` and `invNTT`

use crate::helpers::{full_reduce32, mont_reduce, ZETA_TABLE_MONT};
use crate::types::{R, T};
use crate::Q;


/// # Algorithm 35 NTT(w) on page 36.
/// Computes the Number-Theoretic Transform.
///
/// **Input**: polynomial `w(X) = ∑_{j=0}^{255} w_j X^j ∈ Rq` <br>
/// **Output**: `w_hat = (w_hat[0], ... , w_hat[255]) ∈ Tq`
pub(crate) fn ntt<const X: usize>(w: &[R; X]) -> [T; X] {
    // 1: for j from 0 to 255 do
    // 2: w_hat[j] ← w_j
    // 3: end for
    let mut w_hat: [T; X] = core::array::from_fn(|x| T(core::array::from_fn(|n| w[x].0[n])));

    // for each element of w_hat
    for w_element in &mut w_hat {
        //
        // 4: k ← 0
        let mut k = 0;

        // 5: len ← 128
        let mut len = 128;

        // 6: while len ≥ 1 do
        while len >= 1 {
            //
            // 7: start ← 0
            let mut start = 0;

            // 8: while start < 256 do
            while start < 256 {
                //
                // 9: k ← k+1
                k += 1;

                // 10: zeta ← ζ^{brv(k)} mod q
                let zeta = i64::from(ZETA_TABLE_MONT[k]);

                // 11: for j from start to start + len − 1 do
                for j in start..(start + len) {
                    //
                    // 12: t ← zeta · w_hat[ j + len]
                    let t = mont_reduce(zeta * i64::from(w_element.0[j + len]));

                    // 13: w_hat[j + len] ← w_hat[j] − t
                    w_element.0[j + len] = w_element.0[j] - t;

                    // 14: w_hat[j] ← w_hat[j] + t
                    w_element.0[j] += t;

                    // 15: end for
                }

                // 16: start ← start + 2 · len
                start += 2 * len;

                // 17: end while
            }

            // 18: len ← ⌊len/2⌋
            len /= 2;

            // 19: end while
        }

        // end for each element of w_hat
    }

    // 20: return ŵ
    w_hat
}


/// # Algorithm 36 NTT−1 (`w_hat`) on page 37.
/// Computes the inverse of the Number-Theoretic Transform.
///
/// **Input**: `w_hat` = `(w_hat[0], . . . , w_hat[255]) ∈ Tq` <br>
/// **Output**: polynomial `w(X) = ∑_{j=0}^{255} w_j X^j ∈ Rq`
pub(crate) fn inv_ntt<const X: usize>(w_hat: &[T; X]) -> [R; X] {
    //
    #[allow(clippy::cast_possible_truncation)]
    const F: i64 = 8_347_681_i128.wrapping_mul(1 << 32).rem_euclid(Q as i128) as i64;
    //
    // 1: for j from 0 to 255 do
    // 2: w_j ← w_hat[j]
    // 3: end for
    //let mut w_out = w_hat.clone();
    let mut w_out: [R; X] = core::array::from_fn(|x| R(core::array::from_fn(|n| w_hat[x].0[n])));

    // for each element of w_hat
    for w_element in &mut w_out {
        //
        // 4: k ← 256
        let mut k = 256;

        // 5: len ← 1
        let mut len = 1;

        // 6: while len < 256 do
        while len < 256 {
            //
            // 7: start ← 0
            let mut start = 0;

            // 8: while start < 256 do
            while start < 256 {
                //
                // 9: k ← k−1
                k -= 1;

                // 10: zeta ← −ζ^{brv(k)} mod q
                let zeta = -ZETA_TABLE_MONT[k];

                // 11: for j from start to start + len − 1 do
                for j in start..(start + len) {
                    //
                    // 12: t ← w_j
                    let t = w_element.0[j];

                    // 13: w_j ← t + w_{j+len}
                    w_element.0[j] = t + w_element.0[j + len];

                    // 14: w_{j+len} ← t − w_{j+len}
                    w_element.0[j + len] = t - w_element.0[j + len];

                    // 15: w_{j+len} ← zeta · w_{j+len}
                    w_element.0[j + len] =
                        mont_reduce(i64::from(zeta) * i64::from(w_element.0[j + len]));

                    // 16: end for
                }

                // 17: start ← start + 2 · len
                start += 2 * len;

                // 18: end while
            }

            // 19: len ← 2 · len
            len *= 2;

            // 20: end while
        }

        // 21: f ← 8347681          ▷ f = 256^{−1} mod q
        // 22: for j from 0 to 255 do
        // 23: wj ← f · wj
        for i in &mut w_element.0 {
            *i = full_reduce32(mont_reduce(F * i64::from(*i)));
        }

        // 24: end for
    }

    w_out // 25: return w
}
