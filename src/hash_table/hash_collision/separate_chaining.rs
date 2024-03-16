#[derive(Debug, Clone)]
struct Pair {
    key: i32,
    val: String,
}

/// 链式地址
/// 将单个元素转换为链表, 将键值对作为链表节点, 将所有发生冲突的键值对都存储在同一链表中.
pub struct HashMapChaining {
    /// 键值对的数量
    size: usize,
    /// 哈希表容量
    capacity: usize,
    /// 触发扩容的负载因子阈值
    load_threshold: f32,
    /// 扩容倍数
    extend_ratio: usize,
    /// 桶向量
    buckets: Vec<Vec<Pair>>,
}

impl HashMapChaining {
    pub fn new() -> Self {
        HashMapChaining {
            size: 0,
            capacity: 4,
            load_threshold: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![vec![]; 4],
        }
    }
}
