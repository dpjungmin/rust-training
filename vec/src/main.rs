use vec::MyVec;

fn main() {
    let mut vec = MyVec::<usize>::new();

    vec.push(1usize);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    assert_eq!(vec.len(), 5);
    assert_eq!(vec.capacity(), 8);

    for n in 0..vec.len() {
        assert_eq!(vec.get(n), Some(&(n + 1)));
    }
}
