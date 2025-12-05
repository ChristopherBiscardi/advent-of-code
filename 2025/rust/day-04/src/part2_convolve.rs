use ndarray::prelude::*;
use num_traits::Float;

const PAPER: [[f32; 3]; 3] =
    [[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]];

type Kernel3x3<A> = [[A; 3]; 3];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let n = input.lines().count() + 2;
    let n2 = input.lines().next().unwrap().len();
    // assuming a square input
    debug_assert!(n == n2);

    let mut a = Array::zeros((n, n));

    let mut total = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '@' {
                a[[x + 1, y + 1]] = 1.;
                total += 1;
            }
        }
    }

    let mut last_res = Array::zeros(a.dim());
    loop {
        let mut res = Array::zeros(a.dim());
        conv_3x3(&a.view(), &mut res.view_mut(), &PAPER);
        if res == last_res {
            break;
        }
        for ((_, value), (_, current)) in
            res.indexed_iter().zip(a.indexed_iter_mut())
        {
            // the convolution counts *all* positions, not just
            // the neighbors. That includes the current roll which
            // increases the "4" limit from the problem to "5"
            if *value < 5. {
                *current = 0.;
            }
        }
        last_res = res;
    }
    let result =
        a.iter()
            .filter(|float| {
                if **float < 0.5 { false } else { true }
            })
            .count();

    Ok((total - result).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}

#[inline(never)]
fn conv_3x3<F>(
    a: &ArrayRef2<F>,
    out: &mut ArrayRef2<F>,
    kernel: &Kernel3x3<F>,
) where
    F: Float,
{
    let (n, m) = a.dim();
    let (np, mp) = out.dim();
    if n < 3 || m < 3 {
        return;
    }
    assert!(np >= n && mp >= m);
    // i, j offset by -1 so that we can use unsigned indices
    unsafe {
        for i in 0..n - 2 {
            for j in 0..m - 2 {
                let mut conv = F::zero();
                #[allow(clippy::needless_range_loop)]
                for k in 0..3 {
                    for l in 0..3 {
                        conv = conv
                            + *a.uget((i + k, j + l))
                                * kernel[k][l];
                        //conv += a[[i + k, j + l]] * x_kernel[k][l];
                    }
                }
                *out.uget_mut((i + 1, j + 1)) = conv;
            }
        }
    }
}
