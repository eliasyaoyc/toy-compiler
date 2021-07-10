### 关于Java 基础相关

Java 三大特性：继承、封装、多态

构造方法、成员变量初始化以及静态成员变量三者的初始化顺序：
> 父类的静态变量、父类静态代码块、子类静态变量、子类静态代码块、父类非静态变量、父类非静态代码块、父类构造器、子类非静态变量、子类非静态代码块、子类构造函数
> 所有静态的都只能执行一次

接口和抽象类的相同点和区别：
- 相同点：都不可以被实例化，都需要具体的实现类
- 区别：在1.8中接口可以通过default 关键字写默认方法实现，如果子类需要父类的一点实现用继承，多种不同的实现用接口
重载与重写：
- 重载：方法名相同，参数或返回值不同
- 重写：比如继承
final、finally、finalize 的区别：
final：是关键字，用于声明方法、属性和类，表示不可变、不能被修改、不可被覆盖
finally：异常处理的一部分
注意：finally 不一定会被执行到
1. 当进入try之前就出现异常直接结束
2. 在try 中强制退出，比如 system.exit
finalize：执行垃圾回收，会执行该对象的 finalize 方法，如果在 finalize 方法内对该要回收的对象重新引用则可以「复活」一次，不建议使用因为它不一定会被jvm执行到

string、stringbuffer、stringBuilder
string: string 采用 final 修饰的字符数组进行字符串保存，因此不可变，如果对string 类型对象修改，需要新建对象，将老字符和新增加的字符一并存进去
所以频繁的修改string性能低下，它会不断创建新的string
优化：在 jdk1.9 中把原本的 char 数组变成了 byte数组，如果使用 char的话 一个char需要占用2个byte，双倍的空间
因为在长期的测试中发现常用的string latin-1编码的而不是 utf-16 所以说使用byte 性能更好，但是如果你输入的字符串是 utf16编码的则还是会和原来一样char数组
stringbuilder: 采用无final修饰的字符数组进行保存，因此可变，但线程不安全
stringbuffer: 线程安全的stringbuilder

== 和 equals 的区别
前者比较值，后者是hashcode

序列化
是什么：把对象转换成字节序列，由此可以通过网络对象进行传输

class 对象
是什么：要区分 class 对象和 实例对象的区别，每一个类都有一个class对象其包含了与该类有关的信息
可以通过 Class.forName("")，类名.class 获取
和实例对象的区别：实例对象是需要new出来的

1.8新特性
新的时间类、stream流、lambda、optional、接口的默认方法

lambda 底层实现
编译器在类中生成一个静态函数，运行时以内部类形式调用该静态函数

除了 锁 、Volatile 如何使变量的可变性
通过 asm 在你的代码逻辑前加入 monitorenter 和逻辑后加入 monitorexit 并在 try-catch 里加入 monitorexit

数组如何快速扩容
System.arraycopy -> 浅拷贝

sleep、yield、wait、join的区别
sleep：释放cpu资源不会释放锁
yield：让出cpu资源，但是不会立刻释放
wait： 让出cpu资源和锁
join：等待join线程执行完毕

枚举
是什么：

反射
是什么：反射就是创建对象的一个后门，它可以在运行时任意的构建一个类的对象，获取任意一个类的成员变量和成员方法
1. 反射类及反射方法的获取，都是通过从列表中搜索查找匹配的方法，所以查找性能会随类的大小方法多少而变化
2. 每个类都会有与之对应的class 实例(class实例是通过jvm从方法区中获取的)，从而每个类都可以获取method反射方法，并作用到其它实例上
3. 反射使用软引用relectionData 缓存class信息，避免每次重新从 jvm 获取带来开销
4. 反射调用多次生成新心理 accessor，而通过字节码生成的则考虑了卸载功能，所以会使用独立的类加载器
5. 当找到需要的则copy出来，不是使用原来的实例，从而保证数据隔离
6. 调用反射方法，最终由jvm 执行invoke
https://www.cnblogs.com/yougewe/p/10125073.html

注解
是什么：给编译器提供元信息，通过
@retention 标识该注解是编译阶段还是 运行阶段  
@target 表示作用范围
@inherited 是否可以被继承
@repeatable 可以作用多次，并且每次含义不一样

Reference

异常
error：error 无法错误 jvm 错误，比如 oom
exception：分为运行时异常和非运行时异常 可以通过 trycatch 处理

泛型
是什么：就是在编译期不知道具体的类型，在运行期决定是哪个真实的对象，具体实现就是有一个虚表，里面存放的各个实现类的指针
泛型擦除：编译器生成的字节码是不包含泛型信息的，泛型信息在编译处理是被擦除的

----

关于内部类
> 什么是：把一个类定义在另一个类里面或者方法里面
- 成员内部类: 定义在一个类里面，它可以随意访问外部类的成员和方法
- 原理：反编译之后，可以看到在成语内部类中有一个外部类的成员变量并且会通过构造器进行初始化
- 局部内部类: 定义在一个方法或者作用域下，它仅限于访问方法内或者该作用域内
- 匿名内部类: 没有构造器，比如new thread() 这就是匿名内部类
- 静态内部类: 和成员内部类相似，只是被static 修饰，其次只能访问静态的成员和方法(一般不依赖于外部类则使用静态内部类)
> 注意：匿名和局部内部类假设它们现在在一个作用域或者方法内，则方法的生命周期结束它们还在运行，它们访问的局部变量就会出现问题，是怎么解决的？
> 
> 匿名内部类或者局部内部类它们只能获取局部变量的时候，如果这个局部变量是在编译器能够确定的则直接会内部类中创建一个拷贝，反之不能够确定，则需要通过构造器传参

----

关于有栈协程和无栈协程
首先关于协程它的执行并不是由操作系统来调度，而是需要自己实现一个调度器器来调度，也就是说完全的在用户态，不会涉及到内核态，这就避免了上下文切换
Stackful：比如go 的 gmp 模型就是典型的有栈协程，也就是说每创建一个协程，该协程的上下文都会存储在栈帧中，优点就是这个协议可以出现在任意地方，缺点比较慢
Stackless：比如 rust 的 future 就是无栈协程，它在栈帧中只有一个指针，该指针指向该协程的上下文在内存中，比较快

----

关于并发的一点理解
首先对于Java 来说它的线程模型是 core-pre-thread 模型，也就注定在Java 中是无法创建大量线程的上下文切换很影响性能因为他要从用户态跨越到内核态
所以说在创建用户线程池的时候一般最大线程池数量设置到 cpu core * 2

加上java 自己的 jmm 内存模型来讲，每个线程它都有自己的本地缓存，类比 cpu 的多级缓存，所以在并发操作同一个变量的时候就会出现内存不可见问题

as-if-serial原则就是编译器会对原始的程序进行指令重排序和优化，但不管怎么重排序都需要和原始程序正常输出的结果是一致的
happens-before 原则
1. 次序规则： 一个线程内写在前面的操作先行发生于后面的
2. 锁定规则： unlock 操作先发生于一个锁的lock操作
3. volatile 规则：对volatile 变量的写操作先行发生与后面的读操作
4. 线程启动规则：start 先于线程的每个动作
5. 线程中断规则：对线程的 interrupt 方法调用先与发生与被中断线程的代码检测到中断时间的发生
6. 线程中止规则：线程内所有操作先与中止
7. 对象终结规则：对象初始化先与 finalize
8. 传递性规则：A 先于 B，B 先于C 那么A一定先于C

isinterrupt、interrupt、interrupted：是否中断，中断线程，是否中断并且会擦除中断标记

1. CAS 的原理
   不断的循环通过 旧址的内存地址 旧值 和要替换的新址，判断 旧址的内存地址的值是否与旧值相同，相同则替换。避免造成线程切换造成线程 block 提升性能。
   但是会出现 ABA 问题，可以通过 atomicStampedReference 来解决（版本号来解决的）

2. 多线程有哪些锁机制，以及实现原理

3. 解释什么是MESI协议(缓存一致性)。
   在多核CPU中，内存中的数据会在多个核心中存在数据副本，某一个核心发生修改操作，就产生了数据不一致的问题。而一致性协议正是用于保证多个CPU cache之间缓存共享数据的一致。

----

关于ThreadLocal
什么是：提供局部变量，这个局部变量和正常的变量是不同的，是线程安全的，因为它只能被自己访问
原理：每个线程持有一个 threadlocalmap 并维护了 threadlocal 对象与具体实例的映射，该map由于只被持有它的线程访问，所以是线程安全的
threadlocalmap 的entry 对threadlocal 的引用为弱引用，避免了 threadlocal 对象无法被回收的问题
threadlocalmap 的 set 方法通过调用 replaceStableEntry 方法回收键为null 的entry 对象的值以及entry对象本身从而防止内存泄露
出现内存泄露的原因：threadlocalMap 中entry 的key用到了 弱引用，当没有强引用来引用threadlocal实例的时候就会被gc 回收，但是只会回收key，
所有这个时候在map中会出现很多key为null，value不为null 的entry，这些entry 如果没有主动的调用 get、set、remove 就会发生泄露
首先由于线程一直是活跃的线程，其次不在调用 get、set、remove 方法这些方法会回收entry对象     
Netty 中 FastThreadLocal 的改进

1. 没有使用jdk线性探测法的map，底层通过数据保存，整体查询为O1
2. 它有一个回收的子线程，专门用于回收key 为null 的entry
   读取性能快了5倍，写入性能快了20%

threadlocal 有什么弊端：
单个线程生命周期强绑定，只能在某个线程的生命周期内对 ThreadLocal 进行存取，不能跨线程存取
可以通过 InheritableThreadLocal 可以不感知替代 ThreadLocal，在父子线程前提下，可以拷贝父线程本地变量缓存过的值，但是这个值只能在子线程实例化时候进行
但是无法在预先创建好的线程实例变量传递值
解决： 阿里开源的 TransmittableThreadLocal        
实现原理：就是将 runnable 增强为 TtlRunnable，然后将原本和 Thread 绑定的变量，缓存到 TtlRunnable 对象中，在执行子线程任务前，将对象中缓存的变量值
设置到子线程的 ThreadLocal 中，然后执行完，又恢复现场，不会对复用线程产生影响


----

关于线程池
线程池状态：running、shutdown(不接受任务，处理队列任务)、stop(不接受任务、不处理队列任务)、tidying(所有任务终止，线程数为0)、terminted(彻底终止)
拒绝策略：
1. 直接丢弃
2. 报错
3. 使用调用线程来执行任务
4. 丢弃最旧的
5. 可以通过实现 RejectedExecutionHandler 来自定义拒绝策略

线程池的几个参数：
1. 最小线程数
2. 最大线程数
3. 空闲时间
4. 空闲时间的时间单位
5. 工作队列
6. 创建线程的工厂
7. 拒绝策略执行器 RejectedExecutionHandler

创建线程池的4中方式：
1. Executors 类提供的方法，我一般不怎么用
2.通过构建 ThreadPoolExecutor 类自定义

线城池的执行流程：
1. 是否小于核心线程数是的话创建一个worker
2. 不是的话，等待队列是否满了，不是的话加入等待队列
3. 判断是否小于最大线程数，是的话创建
4. 执行拒绝策略
worker 具有独占锁的语义，每一个worker 只能执行一个任务，每一个任务也只能被一个worker 执行 继承了AQS
⭐ 注意没有没有设置 keepalivetime 这个worker的存活时间的话，会有问题，因为worker 去队列中获取任务是堵塞等待的方式
如果一直没有任务并且也没设置这个存活时间，如果也不调用shutdown 方法则这个线程会无法关闭造成内存泄露
定时线程池的执行流程：
1.

下面这串线程池的代码会输出什么
```java
    public static void main(String[] args) {
        ThreadPoolExecutor executor = new ThreadPoolExecutor(
                1,
                2,
                4,
                TimeUnit.SECONDS,
                new ArrayBlockingQueue<>(1));
        executor.execute(()-> {
            while (true){
                System.out.println(1);
                try {
                    Thread.sleep(5000);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        });

        executor.execute(()-> {
            while (true){
                System.out.println(2);
                try {

                    Thread.sleep(5000);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        });
    }
```
> 会无限循环输出 1，因为 2 这个task 会被塞入queue 中，但是没有第三个task 加入故不会在创建线程，所以当前只有一个线程在运行
> 所以会无限的运行 1

---- 

关于 AQS
什么是：abstractQueuedSynchronizer 是一个抽象的同步器，它有很多实现类，比如 CountdownLatch计数器(不可重用)、信号量(用于限流)、rentreentlock
它是一个显示锁，它是可重入的，需要手动释放锁，它能够加入条件队列 condition，能提供锁中断以及公平非公平锁
原理：1. AQS 的核心是一个被 volatile 修饰的state 变量，在不同的实现类有不同的语义，比如countdownlatch就是锁的个数，信号量就是还有几个信号
2. CLH 队列先进先出的队列
3. 非公平锁是抢占式的获取锁，而公平锁的是顺序的去获取，默认是非公平锁虽然性能快但是容易造成部分线程饥饿
condition 条件队列：
它是aqs 的内部类，提供了 wait、notify、notifyAll 的功能，它和AQS是不同的它内部是一个单向队列，当调用notify的时候会依次唤醒单向队列中的线程去获取锁，没获取到的则加入
aqs 双向队列中

⭐ AQS里的setState()是什么时候调用？
在ReentrantLock 一个线程重入了就调用 setStatus 增加该线程获取锁的次数
在CountDownLatch 中通过构造器传入count 的时候，会调用 setStatus 把count 设置成 Status

----

关于 synchronized
原理：Java 对象底层都关联一个monitor，使用synchronized 时 jvm 会根据环境找到对象的monitor，根据monitor的状态进行加解锁判断。如果成功加锁就成功monitor的持有者
，monitor 在释放前不能被其他线程获取
synchronized 在jvm编译后会产生 1个monitorenter 和2个monitorexit，两个exit 的是因为抛错的时候也要释放锁，
对于同步普通方法锁的是当前实例对象，对于静态同步方法锁的当前类的class对象，对于同步方法块锁的是synchronized括号里的

一般来讲对synchronized 的优化不会一上来就是互斥锁，会分为以下几个步骤之后才会升级到重量级锁
第一阶段：偏向锁：没有锁竞争
1. 根据对象头的mark word 是否属于偏向模式，不属于则进行轻量级锁判断
2. 判断目前请求锁的线程id 是否和偏向锁本身记录的线程id一致，如果不一致，直接跳转步骤4
3. 是否需要重偏向
4. cas 修改线程id，修改失败就是出现竞争升级 轻量级锁
第二阶段：轻量级锁
在轻量级锁中如果竞争过多则会升级成重量级锁
第三阶段：重量级锁

锁消除：编译器进行逃逸分析，如果没有出现逃逸分析则去掉锁

什么是逃逸分析：如果一个变量会被其他线程访问就是逃逸

锁粗化：加大锁的粒度，比如你在一个循环里不断的加锁，其实这是没有必要的，那么编译器会把锁扩展到 循环外部

自适应自旋

----

关于 volatile
保证可见性，禁止重排序，汇编指令会带有 lock前缀，相当于一个内存屏障
但是使用volatile 需要注意的是，它会造成伪共享，我们现在所使用的64位操作系统缓存行差不多在 64 个字节，差不多可以存 8个 long，
被volatile 修饰的变量会把周边的变量一起塞入缓存行这样的话填充整个cache line 有助于性能，也正是因为这个volatile失效是让整个缓存行失效的。
这样就会让其他无辜的变量一起失效影响性能
解决： @content 注解，实现原来在volatile变量周边插入 7个long 进行填充


----

关于 Hashmap

1. 为什么hashmap 的负载因子是0.75
   主要是空间和时间上的平衡
   因为hashmap 发生扩容阈值是负载因子与capacity(默认16) 的乘值来决定是否发生扩容，默认是12
   所以如果在 capacity 不变的情况下，如果把负载因子调高的话，那么扩容的阀值就会升高，这样会导致发生hash冲突几率变高，这样的话元素操作的时候就会增加；
   如果把负载因子调低，虽然hash冲突几率变小，但是扩容的几率就会升高

2. 如何解决hash冲突，为什么hashmap 中的链表需要转成红黑树
   jdk1.7 中是倒序头部插入
   jdk1.8 是正序尾部插入链表，当 capacity > 64 并且链表长度 > 8 会进行转换成红黑树
   这是因为数据量越大的时候则时间的复杂度也是呈线性增长的，而红黑树则是根据树的高度来决定，所以使用红黑树也是性能上的优化

3. hashmap 什么时候会发生扩容
   在达到扩容阈值的时候，就会发生扩容，扩容成当前capacity 的2倍，采用高低分位的方式进行扩容

4. jdk1.8 之前并发操作 hashmap 时为什么会有死循环操作
   1.7 插入元素的方式是倒序头部插入的在并发情况下假如有两个线程同时插入元素后发生扩容，线程一还没refresh  
   反转的时候，cpu进行了切换让线程二执行，然后线程二执行反转后，头部和尾部相连可就变成了死循环
   然后1.8 改为了正序尾部插入了，防止这个问题，但是在多线程情况下，还是会出现线程不安全问题

5. hashmap 扩容时每个 entry 需要在计算一次 hash 吗
   jdk1.7 是会计算的
   jdk1.8 是通过高位运算(hash & oldCapacity) 来判断是否需要移动

6.hashmap的数组长度为什么要保证是2的幂？
平均分布减少碰撞。因为在hash 的时候 是与数组长度进行取模的，如果不是偶数的话那就很容易发生碰撞。

总结：jdk1.7 和 jdk1.8 最主要的区别有两点：
1. 从元素的倒序头部插入变成正序尾部插入，避免死循环，但是还是会出现多线程竞争导致的hash冲突
2. 在发生扩容的时候，1.7 会重新计算每个元素的hash值，影响性能；而1.8 会通过高位运算 hash & oldCapacity 来判断是否需要移动

7. hashmap 如何保证key唯一
   需要重写hashcode 和 equals
   hashcode比较的是内存地址，内存地址可能不能确保一定准备，但是效率高
   equals 是object的方法，还是通过==来比较内存地址，当然可以重写
   hashmap 会先通过hashcode 来比较key 然后再是 equals 所以需要重写这两个方法来保证唯一

----

关于 concurrenthashmap
jdk1.7 是数组加链表，通过ReentrantLock 进行 segment(数据分段) 锁，来保证线程安全，锁定整个segment 锁的粒度很高
jdk1.8 是数组加链表加红黑树，大致逻辑是和HashMap 一致，取消了segment ，通过 cell(cas) 来锁定桶头 降低锁粒度
当元素个数>8 并且数据容量大于 64 时，转红黑树


----

关于 Spring

1. Spring 创建的一个bean 并没有被某些beanPostProcessor处理，是为啥
   被优先级更高的beanpostprocessor 依赖提前创建了

2. 动态代理和Cglib 的区别
   前者需要接口，通过反射
   后者通过继承回调的方式 fastclass 机制：在继承fastclass的动态类只呢个，根据方法签名直接获取方法索引，根据方法索引调用目标方法。
   而反射需要扫描最差结果是 0n

3. Spring 的多数据源怎么实现的，可以通过 Java 代码实现一下吗？
    1. 继承 AbstractRoutingDataSource
    2. 通过 threadlocal 把多个数据源塞进去
    3. 然后实现它的lookup 方法 去threadlocal 中获取


4. Spring IOC 和 AOP 的理解
   依赖注入和控制反转是同一个概念，当某个角色需要另外一个角色协助的时候，在传统的程序中，通常由调用者来创建被调用者实例。
   在spring 中不在由调用者来创建因此叫控制反转。
   创建被调用者的工作由spring来完成然后注入调用者这叫做依赖注入，一般是通过构造器注入但是会出现依赖循环可以用set注入解决

   aop其实就是一个动态代理，一般是由两种方式比如jdk动态代理，需要接口和反射，性能差，spring使用 cglib 继承回调 fastclass 的方式

5. SpringBean 的生命周期的整个流程

    1. 找到 BeanDefintion 定义，然后实例化bean
    2. 给他注入值，beanname 什么的，填充属性
    3. 调用 beanNameAware的 setBeanName setBeanFactory setApplicationContext
    4. 然后执行 beanpostprocessor before 预初始化方法
    5. 调用 initializingBean 的afterProperties set 方法
    6. 调用自定义初始化方法
    7. 调用 beanposetProcessor after 初始化后方法
    8. 基本上是这样，当要销毁的时候就调用destroy方法

6. Bean 的热更新是如何实现的

7. 如何写一个 spring 的 starter
   看下面第 9.
   在 spring.factories 配置文件的里类加上 EnableConfigurationPropertoes， 就可以自动的set 参数了
   然后@bean 注解 注入bean 就可以了

8. Spring 事务失效

> 1. 首先你的数据引擎要支持事务。比如 mylsam 引擎它就不支持事务。
> 2. 没有被 spring 管理
> 3. 方法不是public
> 4. 自身调用问题，因为它没有经过spring的代理类，解决方案就是在类中注入自己
> 5. 数据源没有配置事务管理器
> 6. 把异常吃了，或者异常抛出类型不对(要runtime 才有效)

9. spring 的自动装配和autowired
   什么是自动装配：通过注解或者一些简单的配置就能在 sb 的帮助下实现某块功能，扫描spring.factories 文件来进行加载
   实现原理：核心就是 EnableAutoConfiguration 扫描 spring.factories 文件来进行加载，然后进行加载

10. aware 接口的作用
    什么是：中文意识感知，就是让bean 感知到自身的一些信息，比如 applicationcontextaware 它就能获取application context

11. 三级缓存
    干什么用：用来解决 bean 的循环依赖问题
    singletionObjects：用于存放完全初始化好的 bean，从缓存中取出 bean 可以直接使用
    earlySingletonObjects：提前曝光单例对象的cache，存放原始的bean对象（未被填充属性）用于解决循环依赖
    singletonFactories：单例对象工厂的cache，存放bean工厂对象，用于解决循环依赖
    总结通过先把实例化的对象存进去，这样当出现对象在初始化循环依赖的时候找不到初始化好的对象，直接把实例化的引用进行注入

12. beanpostprocesser before 和 after
    其实就是在bean初始化的过程中可以自定义前置方法或者后置方法

13. spring中用到的设计模式有哪些
    工厂、单例(双重检查锁)、适配、装饰、代理、观察、策略、模板

14. BeanFactory 和 ApplicationContext 的区别
    applicationContext 是 BeanFactory 的子接口
    beanfactory：是最底层的接口，包含了各种bean 的定义、配置文档啥的、管理bean的加载、实例化
    一般来说 都是用ApplicationContext ，beanfactory 它不支持spring插件 aop web 等，它是懒加载，速度慢
    反之提前加载，浪费内存

15. spring 支持哪些作用域
    singleton(单个)、prototype(多个)、request(一个请求一个)、session(一个session 一个)、global-session(全局session)

----

⭐ 关于JVM

gc log 有哪些参数，metadataspace

主要就三块，新生代gc 前后 老年代gc前后 元空间前后 以及gc花费时间
jdk8中 元空间替换了永久代 和方法区是一起的放在了非堆内存，所有元空间大小也是需要注意的 类信息常量静态太多的话也会导致元空间full gc
metadataspace 默认是21m 建议要设置到200m以上
线程堆栈默认是1m 建议设置到500k
如果young gc 频繁那么把新生代设置大一点 因为默认比例是1:2  大对象也要设置大一点默认2m

JVM 内存模型
堆：线程共享，存放实例对象 gc主要管理的对象
方法区：线程共享，存放类信息，常量，静态变量，jit后的代码
方法栈：线程私有，局部变量、对象指针啥的
本地方法栈：线程私有，native 方法
程序计数器：线程私有，
非堆内存：元空间

一个线程报了OOM，整个应用还能继续运行吗
可以，除非都是守护线程

如何理解类文件结构布局？

热部署
什么是：在程序不重启的情况下更新应用
原理：通过自定义类加载器加载新的应用到内存，然后使用新的类，卸载就的类以及对象

3. 热部署与热替换有何区别，如何隔离类重提？

4. JVM 如何管理内存，有何内存淘汰机制?
5. JVM 执行引擎的工作机制是什么？
6. JVM 调优应该遵循什么原则，使用什么工具？
7. JPDA 架构是什么？ 如何应用代码热替换？
8. JVM 字节码增强技术有哪些？

1.类加载器是如何加载 Class 文件的？
①加载， 找到.class文件并把这个文件包含的字节码加载到内存中
②连接， 分别是字节码的验证保证类的正确性，静态变量的分配内存赋默认值，符号引用转换成直接引用
③初始化，为静态变量赋予正确的值，对类变量进行初始化。

2.你有遇到过内存泄露的问题吗? 举个例子?
ThreadLocal 内部通过一个ThreadLocalMap 来存储值，Map 中的 key 是 threadlocal 本身实例，是一个弱引用。
弱引用是在发生gc的时候就会被回收，那么在发生gc的时候如果Map中的 key 没有被强引用引用的话就会被gc掉，但是其value没有一起被回收
这样的话就造成了内存泄露。所以在使用ThreadLocal 的时候不使用的时候一定要进行remove。

3.讲一下JVM堆内存管理(对象的分配过程)？
①.依据逃逸分析，判断是否能在栈上进行分配。如果可以直接在栈上进行分配，用完出栈直接销毁。
②.如果不能在栈上进行分配，判断是否可以大对象，如果是大对象直接进入老年代。
③.如果不是大对象，判断是否可以在TLAB(Thread Local Allocator Buffer) 中进行分配，如果可以直接进行分配。

4.什么是逃避分析？
逃逸分析简单来讲就是可以分析新创建对象的使用范围，是否有竞争，并决定是否在Java堆上进行分配内存的一种技术
1)锁消除也是根据逃逸分析来实现的，如果没有线程竞争，直接会把锁去除
2)当对象没有发生逃逸的时候，就可以直接在栈上进行分配，使用完后直接出栈销毁，减少gc压力
3)标量替换，如果一个对象没有发生逃逸，那么不用在堆上进行分配内存，直接在栈上分配它的局部变量，节省内存空间，提升应用程序性能

5.多大的对象是大对象？
在新生代的一些垃圾收集器比如ParNew 上，是通过 -XX:PerTenureSizeThreshold 来设置。
在G1中，通过 Region大小 只要超过Region的50%以上，那么就判定为大对象。

6.什么是TLAB?为什么需要TLAB?
TLAB 是Thread Local Allocator Buffer，由于对象都是分配在堆上的，但是堆是线程共享的，那么为了保证分配对象的安全需要加锁来保持同步，
但是这样的话性能会很差，所以每个线程会预先分配一些内存放在TLAB中，对象要创建就预先在TLAB中分配。

7.什么情况下会发生栈内存溢出？
死循环，栈帧数量太多超过了整个栈的值。

8.G1两个region不是连续的，而且之间还有可达的引用，我现在要回收其中一个，另一个会被怎么处理？
不知道。

9.JVM中一次完整的GC流程是怎样的，对象如何晋升到老年代？

1. 如果有大对象是直接进入老年代的（3m）
2. 晋升次数是15次默认，超过则会进入老年代
3. 首先是通过gcroot 的 oopmap 集合扫描可达的对象引用，然后在新生代把eden区和 survivor0 的区别进行回收，剩下的对象查看它的晋升次数是否超过15次
4. 没超过的则放到 survivor1 然后替换 survivor0 和 1 的位置，并且增加这些对象的晋升次数

10.你知道哪几种垃圾收集器，各自的优缺点，重点讲下cms和G1，包括原理，流程，优缺点。
cms：
1.初始标记会造成stw：
通过gc root 标记直接关联的对象，但是通过gc root的话要全部扫描是否能直接关联；所以通过oopmap数据结构来完成
每当一个对象在类加载完成的时候就会把这个对象的一些引用放入 oopmap，这样的话在初始标记的时候就不用通过gc root
整个扫描，只需要扫描oopmap中就行。 会暂停用户线程

2.并发标记
由前面初始标记标记过的对象开始出发，所有可达的对象都会在这里进行标记，与用户线程并行

3.并发预清理其实做的还是标记，主要标记在并发标记阶段被修改的对象，新分配到老年代的对象，晋升到老年代的对象。
这步操作是为了减轻重新标记stw的时间。
有两个参数 CMSScheduleRemarkEdenSizeThreshold、CMSScheduleRemarkEdenPenetration，默认是2m和50%，也就是说
当eden 空间使用操作了2m就会触发可中断的并发预清理，直到 eden 空间使用率达到50%，进入remark阶段。
在进入可中断的并发预清理的时候可以通过CMSMaxAborttablePrecleanTime 默认5s 也就是在5s内等待发生minor gc，如果超过5s直接中断进入remark阶段。
但是如果在上面的CMSMaxAborttablePrecleanTime 时间内还没发生minor gc 可以通过CMSScanvengeBeforeRemark，在remark阶段强制minor gc

     在老年代中，有个叫卡表的数组把老年代空间分成512bytes 的块，每个元素对应一个块，因为在并发标记的时候是与用户线程并行的这样就会导致用户修改一些引用，
     通过三色标记，把这些被变掉的引用对应的卡表中的块置为dirty，然后在老年代引用新生代的也会记录在卡表中，这样在预清理发生minor 就可以直接扫描卡表，
     来重新扫描dirty的引用。

4.重标记会造成stw
暂停所有用户线程，重新扫描堆中的对象，进行可达性分析,标记活着的对象。
5.并发清理
用户线程被重新激活，同时清理那些无效的对象。
6.重置
CMS清除内部状态，为下次回收做准备。
缺点：由于是与用户进行并发处理的，这样就会产生浮动垃圾，只能在下次gc的时候在进行回收；
在CPU比较少的机器中，如果使用CMS会导致性能降低，因为CMS默认回收线程是(cpu个数 + 3) / 4

g1：g1 每一代都使用了 n个大小相同 不连续的Region，每个Region 占有一块连续的虚拟内存地址。
Young GC：选定所有年轻代的Region，控制年轻代的Region个数，来控制young gc的时间。

Mixed GC：通过全局并发标记来获取老年代的region，来选择收益较高的region 进行回收。
全局并发标记：1.初始标记，伴随ygc的时候进行。
2.并发标记，从gc root开始对堆对象进行标记。与用户线程并行，通过三色标记把变动的对象形成一个增量式的快照。
3.重新标记，标记那些在并发标记阶段发生变化的对象，将被回收
4.清理。
出现full gc的时机，mixed gc 无法跟上程序的分配速度，导致老年代满了无法进行mixed gc 这时候会使用 serial old gc 来进行full gc



11.垃圾回收算法的实现原理。
标记-清除：标记死亡对象，然后进行清除，并记录一个空闲列表，如果有对象需要创建则会从空闲列表中寻找内存，并划分给新建对象。
但是如果回收的数量很大，那么它执行效率都会随着对象的数量而增长，并且会造成内存碎片。(一般用于老年代)
标记-复制：将内存划分成两块内存，每次只使用一半内存，发生gc的时候把存活的对象移动到另外一块内存中，然后在把已使用的内存进行清除。
这种算法空间使用率很低，因为每次只使用一半内存。（一般用于新生代）
标记-整理：把存活对象聚集到区域的起始位置，能解决内存碎片化问题，但是有压缩的性能开销。

12.JVM内存模型的相关知识了解多少，比如重排序，内存屏障，happen-before，主内存，工作内存等。
重排序：重排序是为了提升性能，然后改变了代码的执行顺序，依靠as-if-serial的语义保证在单线程中结果都是一样的，但是在多线程中就会出现可见性和有序性问题。
内存屏障：就是来解决在多线程下的可见性和有序性问题，主要就是load store 之间的组合。
happen-before：JMM 提供了一些规则来实现有序性
1.在一个线程中，写在前面的操作优先于写入后面的操作，但是有可能会被重排序，但是会依据as-if-serial原则保证最终的结果与正常顺序的的结果一致。
2.一个lock 操作 优先于 unlock 操作
3.被volatile 修饰的变量，写操作优先于读操作
4.a优先于b ，b 优先于 c，那么a优先于c
5.thread对象的start操作优先于此线程的任何操作
6.线程的所有操作都优先于 interrupt
7.对象的初始化操作优先于finalize操作


13.简单说说你了解的类加载器，可以打破双亲委派么，怎么打破。
bootstrapclassloader：启动类加载器，主要负责一些lib目录下的。
extensionclassloader：扩展类加载器，用于加载lib\ext目录下的
applicationclassloader：应用类加载器，classpath目录下的。
每个类加载器都自己的命名空间，在同一个类加载器下不能有名字相同的类，但是在不同类加载器下能有名字相同的类。
双亲委托机制：
1）首先应用类加载器从已经加载器的类中，查询此类是否已经加载，如果已经加载则直接返回。如果没有执行第二步
2) 委托父级加载，则与1）相同，如果还有找到则执行第三步
   3）如果bootstrap 都没找到，则在由当前的类加载器进行加载。

为什么要优先选择父classloader 加载类？
共享功能，避免重复加载，父加载器加载过后，子类加载器就不需要加载了。
隔离功能，保证安全，避免用户自己编写的写动态替换Java的一些核心类。

怎么破坏双亲委托？为什么要破坏？
可以同thread.setclassloader 设置类加载器， 破坏双亲委托机制的原因比如开发一些自定义的plugin的时候，为了避免plugin之间不出现影响。
继承classloader 实现loadclass 的findclass 方法

14.你们线上应用的JVM参数有哪些。
见17题 -XX:+PrintGC：开启打印 gc 信息；
-XX:+PrintGCDetails：打印 gc 详细信息。

15.能作为GC Root 的对象有哪几种？
native方法、栈中引用的对象、静态变量、常量的引用等。

16.怎么打出线程栈信息。
jstack pid
cpu 排查  top -Hp pid 会显示进程号所有线程占cpu情况 然后 jstack pid｜grep 线程id

17.请解释如下jvm参数的含义：
-server -Xms512m -Xmx512m -Xss1024K -
XX:MaxTenuringThreshold=20XX:CMSInitiatingOccupancyFraction=80 -
XX:+UseCMSInitiatingOccupancyOnly
最小堆内存为 512m， 最大堆内存为 512m， 栈内存是1024k
在新生代存活20次后会晋升到老年代，cms 收集的比例
标志不使用运行时收集的数据进行gc 而是根据CMSInitiatingOccupancyFraction 设置的比例来进行gc
调整 -XX:NewRatio，新生代和老年代的比例 默认是1：2，但是大部分对象还是新生代的，会有大量的老年代空间被浪费，增加这个比例

18.对象访问的几种方式？
直接引用：直接存放对象的地址。hotspot采用此方式
句柄：堆中将可能会划分出一块内存来作为句柄池，句柄池中存放的对象实例数据和对象类型数据的具体地址信息

19. 什么情况下会直接在老年代分配
20. 大对象 xx：pretenureSizeThresold=2m 默认2m，此参数只作用于serial parnew
21. 长期存活的对象，对象头中谨慎次数达到15次 默认值
22. minor gc 后 新生代空间放不下
23. 动态年龄判断后

24. 动态年龄判断
    发生于minor gc 后，把对象的晋升次数从大到小进行排序，当内存空间超过s区的50%会把晋升次数大于10的都放入老年代

25. 内存担保机制
    在发生minor gc 之前 会判断老年代是否有足控容纳下所有新生代对象之和 并且判断
    XX:-HandlePromotionFailure参数是否设置，未设置的话会发生full gc 设置的话 会根据之前minor gc 平均晋升到老年代的内存比较，有足够的空间minorgc 反之 full gc

26. 对象分配的过程
    如果对象能够在eden区分配，那么就直接在eden区分配
    如果eden区域大小不够新对象存储的话，则会触发一次minor gc，
    如果minor gc后发现对象也无法在s区分配，则会直接在老年代中分配
    如果老年代也无法存储，则会触发full gc

----

关于 RPC
一次 RPC 请求的流程是什么

1. 服务消费方（client）调用以本地调用方式调用服务
2. client stub 接受到调用后负责将方法、参数等组装成网络传输消息体；
3. client stub 找到服务地址，并将消息进行发送
4. server stub 收到消息后进行解码
5. server stub 根据解码结果调用本地服务
6. 本地服务执行结果返回给 server stub
7. server stub 将返回结果打包并发送至消费方
8. client stub 接受到消息，并进行解码
9. 服务消费方得到最终结果
   ⭐ 以上的这些步骤中：1. 首先是需要一个注册中心用于服务的注册和服务的发现
   2. 需要动态代理，包装整个调用过程
   3. 对于一些需要经常访问的进行长连接，避免三次握手和四次挥手的开销
   4. 其实就是在服务端对本地服务的负载策略
   5. 异步调用

----

⭐ 关于设计模式

总原则：
- 开放封闭原则：对扩展开放，对修改关闭，在程序需要进行拓展的时候，不能人为去修改原有的代码，实现热插拔的效果
- 单一职责原则：一个类、接口或方法只负责一个职责，这样可以降低代码复杂度以及减少代码变更引起的风险
- 依赖倒置原则：针对接口编程，编程依赖于抽象类或接口而不依赖于具体实现类
- 接口隔离原则：将不同功能定义在不同接口中来实现接口隔离。
- 里氏替换原则：任何基类可以出现的地方，子类一定可以出现。
- 迪米特原则：每个模块对其他模块都要尽可能少地了解和依赖，降低代码耦合度。
- 合成复用原则：尽量使用组合( has-a )/聚合( contains-a )而不是继承( is-a )达到软件复用的目的。

单例
什么是：避免一个class 重复的实例化，只实例化一次
懒汉式：线程不安全
饿汉式：线程安全
双重检查锁定：线程安全需要加volatile
需要加 volatile 的原因是 在创建一个对象的时候，其实分了3个步骤
1. 分配对象的内存空间
2. 初始化对象
3. 设置instance 指向刚分配的内存地址
也就是因为一些jit编译重排序的问题，它可能把分配内存给提前到初始化前面了，那么有线程过来访问，就会访问到还未初始化的对象，所以要加volatile
懒汉式（登记式/静态内部类方式）        
public class Singletion {
private static SingletionHolder {
private static final Singletion INSTNACE = new Singletion();
}
private Singletion(){}
public static final Singleton getInstance(){
return SinlgetionHolder.Instance
}
}   
枚举：线程安全
class Resource {

}
public enum Something {
INSTANCE;
private Resource instance;
SomeThing(){
instance = new Resource();
}
public Resource getInstance(){
return instance;
}
}
首先你声明一个枚举类，在编译之后它是会自动的继承 enum类，同时被 final 关键字修饰，这个类不能被继承的，
它的所有成员变量都会被 static final 修饰也就是一个常量
那么这些被static 修饰的属性是会被初始化的时候直接创建，而且只会创建一次所有实例共享，放入运行时常量池里。
所以说是线程安全的
⭐ 枚举类型在序列化的时候Java仅仅是将枚举对象的name属性输出到结果中，反序列化的时候则是通过java.lang.Enum的valueOf方法来根据名字查找枚举对象

工厂
SPI
策略
过滤链
访问
装饰
1. 主要通过抽象类继承抽象类实现
2. 定义一个抽象基类，有很多实现类，然后在继承这个抽象基类，进行扩展(这些扩展里放这被扩展的基类就行)
代理
1. 有一个代理类，可以做一些前置操作或者后置操作
2. 代理类里有一个真正的实现类
观察
1. 我一般会结合 context 来一起实现
2. 在context 里会放一个共享的变量
3. 提供pub/sub 方法 达成解耦，通过 condition 来控制避免一直循环
状态
1. 定一个状态接口
2. 定义一个减库存的实现类实现该接口
3. 然后定一个context 里面放着该状态
4. 接下来就是调用这些实现类，就可以改变context 里面的状态了