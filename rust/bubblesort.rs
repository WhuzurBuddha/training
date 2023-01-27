fn main(){
    let mut vec = vec![32, 1, 9, 17, -6, 12, 4, 8, 16 ];
    for i in 0..vec.len(){
        for j in 1..vec.len(){
            if vec[i]<vec[j]{
                vec.swap(j,i);
            }
        }
    }println!("{:?}",vec);
}
