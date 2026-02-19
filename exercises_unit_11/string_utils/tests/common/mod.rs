use rand::Rng;

pub fn get_test_string()->&'static str{
    let strings=vec!["amma","malayalam","orange","pears"];
    let mut rng=rand::thread_rng();

    let length=strings.len();
    let index=rng.gen_range(0..length);
    &strings[index]
}