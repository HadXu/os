# Rust OS
从头写一个操作系统非常有意思，整个过程非常爽

1. 实现一个最简单的内核
2. 打印东西
3. 实现中断
4. 分页
5. 物理内存分配
6. 堆内存分配算法
7. 并发

## Todo

0. [Programmable Interval Timer (PIT)](https://wiki.osdev.org/Programmable_Interval_Timer)
1. 线程
2. shell
3. 文件系统
4. 网络

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
- 时钟中断

## 分页

在x86系统中，```CR3```寄存器存放着页表。很多系统中有一块专门的缓存存放该数据结构。其中页是虚拟内存中的概念，而对应的是frame对应的是实际硬件内存的地址

![](https://os.phil-opp.com/paging-introduction/paging-page-tables.svg)

这样好处就是可以将虚拟内存划分为很多很多page，以此来映射到内存中的区域，而不是浪费内存的空间 如图所示

![](https://os.phil-opp.com/paging-introduction/segmentation-fragmentation.svg)

一般的x86_64系统采用4级页表，大大提高了性能。目前的64位其实并不是64位，而是48位，还有16位没有用到。

## 分页实现

![](https://os.phil-opp.com/paging-introduction/x86_64-page-table-translation.svg)

物理内存，一般称之为frame，也需要进行分配，在开机启动的时候就需要得到4级的地址，然后向上传递，最后是一级地址映射的物理地址。在页表中存放了哪些物理地址已经被使用。

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
Rust支持Future，Future是一种特殊的数据结构，存放着两种类型(已经完成的以及没有完成的)。



## 时间中断
[Programmable Interval Timer (PIT)](https://wiki.osdev.org/Programmable_Interval_Timer)
通过端口来读取实际的时钟时间，然后进入到计算机的软时钟，开始工作。具体的端口是CMOS (and the Real-Time Clock) can only be accessed through IO Ports 0x70 and 0x71. 直接使用x86_64来去读就行。
```rust
impl CMOS {
    pub fn new() -> Self {
        CMOS {
            addr: Port::new(0x70),
            data: Port::new(0x71),
        }
    }

    pub fn rtc(&mut self) -> RTC {
        while self.is_updating() {
            x86_64::instructions::hlt();
        }

        let mut second = self.read_register(Register::Second);
        let mut minute = self.read_register(Register::Minute);
        let mut hour = self.read_register(Register::Hour);
        let mut day = self.read_register(Register::Day);
        let mut month = self.read_register(Register::Month);
        let mut year = self.read_register(Register::Year) as u16;

        let b = self.read_register(Register::B);
        
        if b & 0x04 == 0 { // BCD Mode
            second = (second & 0x0F) + ((second / 16) * 10);
            minute = (minute & 0x0F) + ((minute / 16) * 10);
            hour = ((hour & 0x0F) + (((hour & 0x70) / 16) * 10)) | (hour & 0x80);
            day = (day & 0x0F) + ((day / 16) * 10);
            month = (month & 0x0F) + ((month / 16) * 10);
            year = (year & 0x0F) + ((year / 16) * 10);
        }

        if (b & 0x02 == 0) && (hour & 0x80 == 0) { // 12 hour format
            hour = ((hour & 0x7F) + 12) % 24;
        }

        year += 2000;
        RTC { year, month, day, hour, minute, second }
    }

    fn is_updating(&mut self) -> bool {
        unsafe {
            self.addr.write(0x0A as u8);
            (self.data.read() & 0x80 as u8) == 1
        }
    }

    fn read_register(&mut self, reg: Register) -> u8 {
        unsafe {
            self.addr.write(reg as u8);
            self.data.read()
        }
    }
}
```
[时钟中断](https://blog.csdn.net/wrx1721267632/article/details/50527595)
[CMOS](https://wiki.osdev.org/CMOS)
[C++ memory order](https://www.zhihu.com/question/24301047)
[C++20 memory](https://en.cppreference.com/w/cpp/atomic/memory_order).
[PIC](https://wiki.osdev.org/8259_PIC)
有两个中断结构，具体参考[IBM PC 8259 PIC 架构](https://wiki.osdev.org/8259_PIC).

## [PCI](https://wiki.osdev.org/PCI)
在Qemu中支持硬件需要一些参数，具体参考[bootimage](https://github.com/rust-osdev/bootimage), 但是需要注意的是在参数传递的时候这样```run-args = ["-nic", "model=rtl8139"]```, 而不能直接添加为一个命令。可以添加的硬件在这里[pci bus](https://www.linux-kvm.org/page/Hotadd_pci_devices)

## [ATA](https://wiki.osdev.org/ATA_PIO_Mode)
ATA磁盘驱动器开发。

## File-System

## Shell
最激动人心的地方到了，就是shell，实现一个shell可不简单，涉及到打印，控制台开发，文件，系统调用等等。

# 参考
[Writing an OS in Rust](https://os.phil-opp.com)
[osdev](https://wiki.osdev.org)
[osblog](https://github.com/sgmarz/osblog)
