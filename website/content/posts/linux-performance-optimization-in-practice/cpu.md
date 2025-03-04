---
title: "Linux性能优化实战(CPU篇)"
date: 2025-03-04T19:14:40+08:00
categories: ["linux性能优化实战"]
tags: ["linux"]
toc: true
math: false
draft: false
---

## 从 top 开始聊 CPU

让我们在 Linux 执行 `top` 命令，看看当前系统的 CPU 使用情况。

> <small>该环境是通过 vagrant 虚拟机创建的，在这里可以找到环境配置 [Vagrantfile](https://github.com/cndoit18/10x-engineer/blob/master/courses/linux-performance-optimization-in-practice/Vagrantfile)</small>

```
top - 11:52:22 up 13:26,  1 user,  load average: 0.12, 0.03, 0.01
Tasks:  99 total,   1 running,  50 sleeping,   0 stopped,   0 zombie
%Cpu(s):  0.0 us,  0.2 sy,  0.0 ni, 99.8 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
KiB Mem :  1008928 total,   140696 free,   122764 used,   745468 buff/cache
KiB Swap:  1003516 total,  1002736 free,      780 used.   727580 avail Mem

  PID USER      PR  NI    VIRT    RES    SHR S  %CPU %MEM     TIME+ COMMAND
22807 vagrant   20   0  108344   5260   4112 S   0.3  0.5   0:00.08 sshd
    1 root      20   0   77972   9052   6600 S   0.0  0.9   0:02.70 systemd
    2 root      20   0       0      0      0 S   0.0  0.0   0:00.02 kthreadd
    4 root       0 -20       0      0      0 I   0.0  0.0   0:00.00 kworker/0:0H
    6 root       0 -20       0      0      0 I   0.0  0.0   0:00.00 mm_percpu_wq
    7 root      20   0       0      0      0 S   0.0  0.0   0:00.46 ksoftirqd/0
```

我们先聚焦 `%Cpu(s)` 和 `load average` 这两部分。在 `man top` 中可以找到具体的解释。

```
   2a. UPTIME and LOAD Averages
       This portion consists of a single line containing:
           program or window name, depending on display mode
           current time and length of time since last boot
           total number of users
           system load avg over the last 1, 5 and 15 minutes

   ...

       Line 2 shows CPU state percentages based on the interval since the last refresh.
       As a default, percentages for these individual categories are displayed.  Where two labels are shown below, those for more recent kernel versions are shown first.
           us, user    : time running un-niced user processes
           sy, system  : time running kernel processes
           ni, nice    : time running niced user processes
           id, idle    : time spent in the kernel idle handler
           wa, IO-wait : time waiting for I/O completion
           hi : time spent servicing hardware interrupts
           si : time spent servicing software interrupts
           st : time stolen from this vm by the hypervisor
```

```
   2a. UPTIME 和 LOAD Averages
       这部分由一行组成，其中包含
           程序或窗口名称，取决于显示模式
           当前时间和距上次启动后的时间
           用户总数
           过去 1 分钟、5 分钟和 15 分钟的系统负载平均值

   ...

       第二行显示基于自上次刷新后的时间间隔所统计的CPU状态百分比。
       默认情况下，会显示这些单个类别的百分比。 如果下面显示两个标签，则先显示较新内核版本的标签。
           us, user    : 用户态时间
           sy, system  : 内核态时间
           ni, nice    : 低优先级用户态时间
           id, idle    : 内核空闲处理程序所消耗的时间
           wa, IO-wait : 等待 I/O 完成的时间
           hi : 处理硬件中断所消耗的时间
           si : 处理软件中断所消耗的时间
           st : 虚拟机被虚拟机监控程序偷取的时间
```

> `top` 命令也是通过 [proc 文件系统](https://docs.kernel.org/filesystems/proc.html) 来获取信息的。你可以试试执行 `man proc` 命令，查看更多相关的信息。

### 平均负载

我们来看看 `load average` 这一项。这里提一个问题，平均负载是怎么计算的？

为了解释清楚这个，我们需要看看在 [loadavg.c](https://github.com/torvalds/linux/blob/master/kernel/sched/loadavg.c#L16-L17) 中是怎么描述的。

```
 * The global load average is an exponentially decaying average of nr_running +
 * nr_uninterruptible.
```

```
 * 全局平均负载是 nr_running + nr_uninterruptible 的指数衰减平均值。
```

也就是说，在计算平均负载时，会将当前正在运行的进程数和不可中断的进程数加起来，然后再除以单位时长。以1分钟平均负载为2为例，即表示在过去的1分钟内，平均有2个进程在运行。
在不同个数的 CPU 上，这个2所代表的含义也不一样。

- 在有4个CPU的系统上，意味着所有CPU都刚好被占用一半。
- 在只有2个CPU的系统上，意味着所有CPU都刚好被完全占用。
- 在只有1个CPU的系统上，意味着有一半的进程竞争不到CPU。

### CPU利用率
