
fn main() {

    let v1: [u32; 2] = [1,2];
    assert_eq!(get_u32_sum(&v1[..]).unwrap(), 3);
    println!("The sum of v1 is {}", get_u32_sum(&v1[..]).unwrap());
    
    let v2: [u32; 2] = [2, u32::MAX];
    assert_eq!(get_u32_sum(&v2[..]), None);
}

fn get_u32_sum(u32_vec: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in u32_vec {
        match sum.checked_add(*i) {
            Some(v) => {
                sum = v;
            },
            None => {
                return None;
            },
        }
    }
    Some(sum)
}