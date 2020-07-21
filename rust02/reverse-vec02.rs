fn main(){
    let mut v = vec![1,2,3,4,5];
    let len = v.len();
   for i in 0..len/2{
      let temp = v[i];
       v[i] = v[len-i-1];
       v[len-i-1] = temp;
   }
   println!("{:?}",v)
   
}