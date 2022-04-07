use ronp::diff::Diff;

fn main() {
    let a = vec![1,2,3];
    let b = vec![1,5,3];

    println!("A:{:?}", a);
    println!("B:{:?}", b);

    let diff = Diff::new(a, b);
    let res = diff.build();
    println!("editdistance: {}", res.ed());
}
