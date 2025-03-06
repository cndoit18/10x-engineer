---
title: "Linux性能优化实战(CPU篇)"
date: 2025-03-04T19:14:40+08:00
categories: ["linux性能优化实战"]
tags: ["linux"]
toc: true
math: false
draft: false
---

# 从 top 开始聊 CPU

让我们在 Linux 执行 `top` 命令，看看当前系统的 CPU 使用情况。

> <small>该环境是通过 vagrant 虚拟机创建的，在这里可以找到环境配置 [Vagrantfile](https://github.com/cndoit18/10x-engineer/blob/master/courses/linux-performance-optimization-in-practice/Vagrantfile)</small>

```console
$ top
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

```console
$ man top
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

```console
$ man top
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

## 平均负载

我们来看看 `load average` 这一项。这里提一个问题，平均负载是怎么计算的？

为了解释清楚这个，我们需要看看在 [loadavg.c](https://elixir.bootlin.com/linux/v6.13.5/source/kernel/sched/loadavg.c#L16-L17) 中是怎么描述的。

```
 * The global load average is an exponentially decaying average of nr_running +
 * nr_uninterruptible.
```

```
 * 全局平均负载是 nr_running + nr_uninterruptible 的指数衰减平均值。
```

所以平均负载和task相关，而非CPU相关，但是在不同个数的 CPU 上，这个平均负载所代表的含义也不一样。

- 在四核 CPU 系统上，意味着所有CPU都刚好被占用一半。
- 在双核 CPU 的系统上，意味着所有CPU都刚好被完全占用。
- 在单核 CPU 的系统上，意味着有一半的进程竞争不到CPU。

### 实验验证

为了更好的理解这个问题，我们在环境上跑一个命令。

```console
# yes 命令会一直输出 yes，直到被杀掉
$ yes &>/dev/null &

# 后续需要使用 fg 命令将后台的进程拉到前台并停止运行
```

接着我们执行 `top` 命令 [^uptime]，会发现 `load average` 的值在不断变化。

```console
$ top
top - 03:09:06 up 15:19,  1 user,  load average: 0.83, 0.30, 0.11
Tasks: 100 total,   2 running,  50 sleeping,   0 stopped,   0 zombie
%Cpu(s): 28.3 us, 22.0 sy,  0.0 ni, 49.8 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
KiB Mem :  1008928 total,   139568 free,   122896 used,   746464 buff/cache
KiB Swap:  1003516 total,  1002736 free,      780 used.   727440 avail Mem

  PID USER      PR  NI    VIRT    RES    SHR S  %CPU %MEM     TIME+ COMMAND
23179 vagrant   20   0    7468    760    696 R 100.0  0.1   1:41.16 yes
```

我们可以看到在 `yes` 命令这行，它的状态是 `R` (Running)，同时它的 `%CPU` 是 100%。 一分钟的平均负载是 `0.83`，在双核 CPU 系统上，整体 CPU 的使用率应该是 40% 左右（可以将 28.3 us 和 22.0 sy 相加）。

为了验证这个结果，我们需要使用 `pidstat`[^sysstat] 来观察指定进程的状态。

```console
$ pidstat -p 23388 1
Linux 4.15.0-58-generic (vagrant)       03/05/2025      _x86_64_        (2 CPU)

05:27:15 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
05:27:16 AM  1000     23388   57.00   43.00    0.00    0.00  100.00     0  yes
05:27:17 AM  1000     23388   53.00   46.00    0.00    0.00   99.00     0  yes
05:27:18 AM  1000     23388   66.00   34.00    0.00    0.00  100.00     0  yes
05:27:19 AM  1000     23388   59.00   41.00    0.00    0.00  100.00     0  yes
05:27:20 AM  1000     23388   56.00   45.00    0.00    0.00  100.00     0  yes
05:27:21 AM  1000     23388   64.00   35.00    0.00    0.00   99.00     0  yes
05:27:22 AM  1000     23388   59.41   39.60    0.00    0.00   99.01     0  yes
05:27:23 AM  1000     23388   64.00   36.00    0.00    1.00  100.00     0  yes
05:27:24 AM  1000     23388   58.00   42.00    0.00    0.00  100.00     0  yes
05:27:25 AM  1000     23388   55.00   45.00    0.00    0.00  100.00     0  yes
05:27:26 AM  1000     23388   61.00   39.00    0.00    0.00  100.00     0  yes
```

可以看到 `yes` 命令大概有一半左右的时间是在用户态，一半时间是在内核态，符合我们上面提到的整体 CPU 使用情况。

所以，我们在看到平均负载的时候，先要观察不同时间段的差异，如果相差不大，那说明系统负载很平稳。
而平均负载高于 CPU 数量的 70% 时，就需要排查负载搞的原因了。一旦负载过高，就会导致进程响应慢，进而导致服务的正常功能。

## CPU利用率

### CPU 密集型进程

我们来复现一个由于代码实现导致 CPU 利用率过高的场景。

在 [vagrant](https://github.com/cndoit18/10x-engineer/blob/master/courses/linux-performance-optimization-in-practice) 环境里， 提供了一个 high-cpu 的程序。

```bash
# 构建一个high-cpu
go build /vagrant/high-cpu/main.go
# 后台执行
./main &
```

通过 `ab` [^apache2-utils] 对这个服务进行压力测试。

```console
$ ab -c 10 -n 100 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient).....done


Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        10 bytes

Concurrency Level:      10
Time taken for tests:   14.033 seconds
Complete requests:      100
Failed requests:        0
Total transferred:      12700 bytes
HTML transferred:       1000 bytes
Requests per second:    7.13 [#/sec] (mean)
Time per request:       1403.326 [ms] (mean)
Time per request:       140.333 [ms] (mean, across all concurrent requests)
Transfer rate:          0.88 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.6      0       3
Processing:   745 1380 213.4   1379    2082
Waiting:      744 1378 214.4   1379    2082
Total:        745 1380 213.3   1379    2082

Percentage of the requests served within a certain time (ms)
  50%   1379
  66%   1463
  75%   1528
  80%   1550
  90%   1634
  95%   1698
  98%   1872
  99%   2082
 100%   2082 (longest request)
```

这个服务居然需要2秒才能返回，这显然是不合理的。我们提高一下压测时长，进一步观察 CPU 的变化。

```console
$ ab -c 10 -n 100000 http://localhost:8080/ &>/dev/null &
$ pidstat 1
Linux 4.15.0-58-generic (vagrant)       03/06/2025      _x86_64_        (2 CPU)

04:53:49 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:50 AM  1000     23855  100.00    8.82    0.00    0.00  100.00     1  main

04:53:50 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:51 AM  1000     23767    0.00    1.00    0.00    0.00    1.00     0  sshd
04:53:51 AM  1000     23855  100.00    5.00    0.00    0.00  100.00     1  main
04:53:51 AM  1000     23873    1.00    0.00    0.00    0.00    1.00     0  pidstat

04:53:51 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:52 AM  1000     23855  100.00    1.98    0.00    0.00  100.00     1  main

04:53:52 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:53 AM     0     23543    0.00    0.99    0.00    0.00    0.99     0  kworker/0:0
04:53:53 AM  1000     23855  100.00    3.96    0.00    0.00  100.00     1  main

04:53:53 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:54 AM  1000     23855  100.00    6.93    0.00    0.00  100.00     1  main
04:53:54 AM  1000     23873    0.99    0.00    0.00    0.00    0.99     0  pidstat

04:53:54 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:55 AM  1000     23855  100.00    2.00    0.00    0.00  100.00     1  main

04:53:55 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:56 AM     0     23232    0.00    0.99    0.00    0.00    0.99     1  kworker/1:1
04:53:56 AM  1000     23855  100.00    0.00    0.00    0.99  100.00     1  main

04:53:56 AM   UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
04:53:57 AM  1000     23855  100.00    6.93    0.00    0.00  100.00     1  main
```

可以看到，这个 `main` 程序已经把 CPU 打满了，我们继续用 `perf` 来分析这个进程。

```console
$ sudo perf top -p 23855
  74.69%  main      [.] main.main.func1
  11.99%  [kernel]  [k] finish_task_switch
   5.15%  [kernel]  [k] __softirqentry_text_start
   4.29%  [kernel]  [k] exit_to_usermode_loop
   1.07%  [kernel]  [k] _raw_spin_unlock_irqrestore
   0.24%  [kernel]  [k] get_signal
   0.18%  [kernel]  [k] __fget
   0.11%  [kernel]  [k] do_syscall_64
   0.08%  [kernel]  [k] VbglR0GRPerform
   0.07%  [kernel]  [k] __radix_tree_lookup
   0.07%  [kernel]  [k] sock_poll
   0.07%  [kernel]  [k] queue_work_on
   0.06%  main      [.] runtime.preemptone
   0.05%  main      [.] runtime.mcall
   0.05%  main      [.] runtime.retake
   0.05%  main      [.] runtime.findRunnable
   0.05%  [kernel]  [k] apparmor_task_kill
   0.05%  main      [.] runtime.netpoll
   0.04%  main      [.] runtime.findfunc
   0.04%  main      [.] runtime.lock2
   0.04%  [kernel]  [k] do_signal
   0.04%  [kernel]  [k] copy_user_generic_string
   0.04%  main      [.] runtime.asyncPreempt.abi0
   0.04%  main      [.] runtime.sysmon

```

有 `74.69%` 的时间花在了 `main` 函数里。所以我们继续检查 `main`，回车选择 `Annotate main.main.func1`。

```console
       │    func main() {
       │            http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
       │ 0:   cmp    0x10(%r14),%rsp
       │    ↓ jbe    9d
       │      push   %rbp
       │      mov    %rsp,%rbp
       │      sub    $0x20,%rsp
       │      mov    %rax,0x30(%rsp)
       │      mov    %rbx,0x38(%rsp)
       │                    x := 0.0
       │                    for i := 0; i < 1000000000; i++ {
       │      xor    %ecx,%ecx
       │      xchg   %ax,%ax
       │    ↓ jmp    25
 99.44 │22:   inc    %rcx
  0.56 │25:   cmp    $0x3b9aca00,%rcx
       │    ↑ jl     22
       │                            x += math.Sqrt(x)
       │                    }
       │
       │                    io.WriteString(w, "It works!\n")
       │      test   %rax,%rax
       │    ↓ je     47
       │      mov    0x8(%rax),%rdx
       │      mov    main..typeAssert.0,%rsi
       │      mov    (%rsi),%r8
       │      mov    0x10(%rax),%r9d
       │    ↓ jmp    5e
       │47:   lea    go:string.*+0x3f97,%rcx
       │      mov    $0xa,%edi
       │    → callq  io.WriteString
       │            })
```

能看到，`main.main.func1` 有 `99.44%` 的时间花在了 `for` 循环中了。

看一下源代码

```go
package main

import (
        "io"
        "math"
        "net/http"
        _ "net/http/pprof"
)

func main() {
        http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
                x := 0.0
                for i := 0; i < 1000000000; i++ {
                        x += math.Sqrt(x)
                }

                io.WriteString(w, "It works!\n")
        })

        if err := http.ListenAndServe(":8080", nil); err != nil {
                panic(err)
        }
}
```

我们居然花了这么多的时间在无用的 `Sqrt` 函数上。我们把这段循环注释掉，重新编译试试看。

```console
$ ab -c 10 -n 1000 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Completed 600 requests
Completed 700 requests
Completed 800 requests
Completed 900 requests
Completed 1000 requests
Finished 1000 requests


Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        10 bytes

Concurrency Level:      10
Time taken for tests:   0.142 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      127000 bytes
HTML transferred:       10000 bytes
Requests per second:    7048.26 [#/sec] (mean)
Time per request:       1.419 [ms] (mean)
Time per request:       0.142 [ms] (mean, across all concurrent requests)
Transfer rate:          874.15 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   0.6      0       4
Processing:     0    1   0.5      1       3
Waiting:        0    0   0.4      0       3
Total:          0    1   0.9      1       5
WARNING: The median and mean for the initial connection time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      2
  80%      2
  90%      3
  95%      3
  98%      4
  99%      4
 100%      5 (longest request)
```

性能得到巨大提升，现在一个请求不到5ms就完成了。

[^uptime]: uptime 命令也可以查看平均负载

[^sysstat]: [sysstat](https://github.com/sysstat/sysstat) 是一个系统管理工具包，包含了许多系统监控工具，可以查看进程状态

[^apache2-utils]: [httpd](https://github.com/apache/httpd) 中的工具，有包含 `ab` 这种压测工具
