# 总结功能

将lab1和lab2中实现的syscall移植到当前框架。实现新的系统调用`spawn`，从`TaskControlBlock`创建新的进程，配置好parent和children的关系后加入任务队列。实现stride调度算法，给每个进程增加了新的属性`stride`和`priority`，进程调度一次，`stride`增加值，为了保证优先级，这个增加的值为$\frac {stride}  {priority}$

# 简答作业

实际情况是轮到 p1 执行吗？为什么？

实际情况是轮到p2执行。
p2.stride = 250, pass = 10，p2执行完之后，p2.stride会溢出，变成4, 此时`p2.stride<p1.stride`，按照算法，还是选择p2执行。

---

为什么？尝试简单说明（不要求严格证明）。

要证

$$STRIDE\_MAX – STRIDE\_MIN \leq \frac {BigStride}  2$$

可以假设

$$STRIDE\_MAX – STRIDE\_MIN \gt \frac {BigStride}  2$$

由于$priority \gt 2$，pass的最大值为$\frac {BigStride}  2$，所以`p.stride==STRIDE_MAX`的进程被调度前，`p.stride`最小值为$STRIDE\_MAX - \frac {BigStride}  2$，这个值依然大于`STRIDE_MIN`，应该优先调度`p.stride=STRIDE_MIN`的进程，推出矛盾。

---

已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。

在不考虑溢出的情况下，上述不等式成立。考虑溢出，设两者中的较大值为`max`，较小值为`min`，则会出现两种情况

1. `max`和`min`的溢出次数相等
2. `min`比`max`多溢出一次

第一种情况

$$max - min \leq \frac {BigStride} 2$$

第二种情况

$$(min + (n + 1) \cdot BigStride) - (max + n \cdot BigStride) \leq \frac {BigStride} 2$$

即

$$max - min \geq \frac {BigStride} 2$$


```rust
use core::cmp::Ordering;

struct Stride(u64);
const BigStride = 1_000_000;

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.0;
        let b = other.0;
        let max = a.max(b);
        let min = a.min(b);
        if max - min <= BIG_STRIDE / 2 {
            if max == a {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else {
            if min == a {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

# 荣誉准则
1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2.此外，我也参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3.我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4.我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。