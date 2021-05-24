# Rust OS
从头写一个操作系统非常有意思，整个过程非常爽

1. 实现一个最简单的内核
2. 打印东西
3. 实现中断
4. 分页
5. 物理内存分配
6. 堆内存分配算法
7. 并发

## 第一章 裸机上的文件



## 第二章 一个最小的内核


## 第三章 VGA显示

## 第四章 中断
CPU运行过程中有很多错误，这个时候需要处理错误就这需要用到中断。

- 中断向量表

### 函数调用约定
- 前6个函数参数存放在```rdi, rsi, rdx, rcx, r8, r9 ``` 中
- 其余的参数存放在栈中
- 函数返回值存放在```rax,rdx```中

在一个函数(callee)被调用过程中,参数是不能改变的，这些寄存器的值是不允许被修改的。一般来说是将值放入栈中。

## Double Faults
一旦系统出现 **triple fault** ，对于操作系统来说是致命的，因此系统会重新启动。 所以要避免三级错误，一般在二级错误的时候就进行处理。

中断类型

- Switching Stacks
- [IST & TSS](https://os.phil-opp.com/double-fault-exceptions/#the-ist-and-tss)

## 硬件中断

### The 8259 PIC

- 键盘输入中断
- 

## 分页

在x86系统中，```CR3```寄存器存放着页表。很多系统中有一块专门的缓存存放该数据结构。其中页是虚拟内存中的概念，而对应的是frame对应的是实际硬件内存的地址

![](https://os.phil-opp.com/paging-introduction/paging-page-tables.svg)

这样好处就是可以将虚拟内存划分为很多很多page，以此来映射到内存中的区域，而不是浪费内存的空间 如图所示

![](https://os.phil-opp.com/paging-introduction/segmentation-fragmentation.svg)

一般的x86_64系统采用4级页表，大大提高了性能。目前的64位其实并不是64位，而是48位，还有16位没有用到。

## 分页实现

![](https://os.phil-opp.com/paging-introduction/x86_64-page-table-translation.svg)

物理内存也需要进行分配，在开机启动的时候就需要得到4级的地址，然后向上传递，最后是一级地址映射的物理地址。在页表中存放了哪些物理地址已经被使用。

## 动态内存分配
栈中的存储具有固定性缺点，也就是大小在编译时固定，运行时不可改变。

在内存中划分出一个区域专门用来堆分配。 同时分配有各种方法

- Bump Allocator
直接开辟大量空间使用，一般不会用，因为非常浪费空间。
- Linked List Allocator
用链表来将一个个free空间串起来，以备使用。
![](https://os.phil-opp.com/allocator-designs/linked-list-allocation.svg)
- Fixed-Size Block Allocator
与Linked List Allocator不同的是，该方法是另一种分配策略，根据给定的大小来分配空间，比如分配4byte，给他16byte的块。分配48byte，给他64byte的块。
![](https://os.phil-opp.com/allocator-designs/fixed-size-block-example.svg)




## 多线程
Rust支持Future，Future是一种特殊的数据结构，存放着两种类型已经完成的以及没有完成的。


# 参考
[Writing an OS in Rust](https://os.phil-opp.com)
[osdev](https://wiki.osdev.org)
[osblog](https://github.com/sgmarz/osblog)
