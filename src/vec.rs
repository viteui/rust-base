fn main() {
    let mut v = vec![1, 2];
    assert!(!v.is_empty()); // 检查 v 是否为空

    v.insert(2, 3); // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3]
    assert_eq!(v.remove(1), 2); // 移除指定位置的元素并返回, v: [1, 3]
    assert_eq!(v.pop(), Some(3)); // 删除并返回 v 尾部的元素，v: [1]
    assert_eq!(v.pop(), Some(1)); // v: []
    assert_eq!(v.pop(), None); // 记得 pop 方法返回的是 Option 枚举值
    v.clear(); // 清空 v, v: []
    let mut a = vec![10000];
    let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
    v.push(10000000);
    v.append(&mut v1); // 将 v1 中的所有元素附加到 v 中, v1: []
    v.append(&mut a);
    println!("{:?}  v1: {:?}", v, v1);
    v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
    v.retain(|x| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let _v2 = m.split_off(1); // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]

}
