# Rust OS

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

# 参考
[Writing an OS in Rust](https://os.phil-opp.com)
[osdev](https://wiki.osdev.org)
