use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(length: usize) -> String {
  let mut rng = thread_rng();
  let string = rng.sample_iter(&Alphanumeric).take(length).collect();
  string
}
