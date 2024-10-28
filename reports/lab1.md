# 总结功能

为`TaskControlBlock`结构体新增`syscall_times`和`start_time`字段，在第一次调度task时，记录时间；在`trap_handler`的系统调用case，统计系统调用的种类和次数。

最后实现新的系统调用`sys_task_info`，整合`TaskControlBlock`中记录的信息，返回`TaskInfo`对象的指针。

# 简答作业

出错行为：

控制台打印错误信息，并开始执行下一个应用程序。

ch2b_bad_address.rs 对空指针`(0x0 as *mut u8)`进行写操作，触发了`StoreFault`异常；ch2b_bad_instructions.rs 试图在U模式下运行S模式的指令`sret`，ch2b_bad_register.rs试图在U模式下访问CSR，都触发了`IllegalInstruction`异常。代码在`trap_handler`中对这两个异常的处理是打印错误信息，执行下一个应用程序。

RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0

RustSBI-QEMU Version 0.2.0-alpha.2

---

1.
场景一：`__alltraps`调用完`trap_handler`后，继续执行后续的指令，这时`a0`是`trap_handler`的返回值，是`TrapContext`对象的指针。

场景二：首次调度task时，由于初始化`TaskContext`时`ra`的值为`__restore`的地址，所以当`__switch`最后执行`ret`时(`ret`是Pseudo Instructions，对应`jalr x0, x1, 0`，会跳转到x1即ra内存储的地址)，跳转到`__restore`，这时候`a0`的值是`__restore`的地址。

2.
sstatus: Trap发生之前CPU处在哪个特权级

sepc: 记录Trap发生之前执行的最后一条指令的地址

sscratch: 用来存放要切换的状态的sp

3.
x2寄存器是sp，剩下的其他指令依赖sp做相对寻址，当其它寄存器都恢复后，最后执行`csrrw sp, sscratch, sp`指令，将sp恢复。

x4寄存器是Thread pointer，单核系统用不到。

4.
sp中存放用户栈栈顶的地址，sscratch中存放内核栈栈顶的地址。

5.
`sret`指令。sstatus中的`spp`如果特权级是U，执行流会转移到用户态，通过把sepc的值传递给pc寄存器实现。如果出现嵌套的Trap，则状态不会改变。

6.
sp中存放内核栈栈顶的地址，sscratch中存放用户栈栈顶的地址。

7.
L13: `csrrw sp, sscratch, sp`

这条指令切换了栈空间。

# 荣誉准则
1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2.此外，我也参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3.我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4.我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。