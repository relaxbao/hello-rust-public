fn main(){
    //注意，要往向量中增加元素，需要用mut创建可变绑定
    let mut v1 = vec![];//初始化方法1
    v1.push(1);
    v1.push(2);
    v1.push(3);

    //用下标索引访问
    assert_eq!(v1,[1,2,3]);
    assert_eq!(v1[1],2);

    let mut v2 = vec![0,10];//初始化方法2
    let mut v3 = Vec::new();//初始化方法3

    v3.push(4);
    v3.push(5);
    v3.push(6);

    println!("{:?}",v2)

    //v3[4]; error: index out of bounds
}