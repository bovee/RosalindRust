//
// FIXME: not finished

fn main() {

}

// Aa Bb
//
// 0.25 AA
// 0.50 Aa
// 0.25 aa
//
// 0.25 BB
// 0.50 Bb
// 0.25 bb
//
// so, 0.25 chance of Aa Bb
//
// N AaBb out of 2^k kids
// Binomial (need to sum with all N from N to k^2?)
// (2^k)! / (N! * (2^k - N)!) * p^N * (1-p)^(k^2-N)
