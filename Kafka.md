⭐ Kafka 的架构
一些概念
- AR(ISR+OSR) 所有的分区副本， isr 可用的分区副本， osr 不可用的分区副本
- HW 高水位，consumer 只能消费到这里
- LEO 分区的最新的offset

为什么不像 Redis 一样支持读写分离，而是读写都由 Partition Leader 来负责
原因：
- 首先消息队列不像 Redis 一样是一个读多写少的组件，他写多于读
- 他本身的设计副本是异步同步的，那么必然会有延迟的，那么就无法做到 单调读

kafka 的网络层实现
就是一个典型的  reactor 模型，

1. 有一个 acceptor 线程用来接受客户端的请求，这个客户端并不单单是producer 还可以是 consumer 或者是其他的 broker，
2. 然后是一个 processor 线程池用来，这个processor 线程只是把这些请求塞到了请求队列中requestqueue，并不会真正的处理请求，
3. 然后由io线程从请求队列中获取请求并且执行，然后在塞回响应队列，这个响应队列就是在processor中并有processor 响应给客户端

⭐ 这里有一个思考，为什么requestqueue 是线程共享的，而响应队列则是线程私有的？
首先是因为 kafka 的processor 线程默认是3个 通过 num.network.threads 配置，而真正处理 request 的线程有8个 num.io.threads
所以让 processor 并发的来处理提升性能，而 io线程处理完了通过线程私有设计成 lock-free 可以避免 reace content

-----

SSD Kafka 架构设计
问题：当单机承载的 topic 和 partition 数量很大的时候，很容易出现不同的 partition 间竞争 pagecache 资源，互相影响，导致整个broker 的吞吐降低
主要出现在实际消费和一些延迟消费的consumer，延迟消费的consumer 占用了需要实时消费的pagecache，导致cache miss 去读hdd的时候 并发读导致性能急剧下降
解决方案：上 ssd

-----

kafka 发送消息的流程

1. Producer 发送一条消息首先会经过 Interceptors、Serializer、Partitioner，当需要自定义一些功能比如消息审计，消息追踪啥的都可以实现这些接口做到自定义
2. 然后消息会被放到寄存器 RecordAccumulate 中，里面是多个双端队列，队列里存放的是 producerbatch，放进去之后等待batch 满或者一定时间后会触发发送
3. sender 线程就是专门从 RecordAccumulate 中获取满足条件的 producerBatch 然后进行发送，它并不会马上发送出去
4. 而是会封装成 clientRequest 然后放入 inflightRequests 这个等待队列中
5. 由一个 select 的子线程进行发送，就是 java中的 nio

-----

关于 producer / consumer 如何管理 tcp 连接的
producer：是通过 sender 线程与 broker 进行的tcp线程，如果conntions.max.idle.ms 参数大于 0 则会自动关闭，反之不会关闭称为僵尸连接
consumer：在调用 kafkacomsumer.poll 方法时被创建的，如果9分钟没有获取到任何的消息的话，则会杀掉tcp连接

-----

⭐ 关于 consumer 的 rebalance
什么是：Consumer 的rebalance 机制就是当消费者出现不平衡的时候重新平衡

1. 当所有的消费者初始化的时候，每个 consumer 都会发送一个 joinGroup 的请求给 Coordinator(协调者)，然后协调者会选出一个 consumer leader，一般是第一个发送请求的consumer
2. 然后 Coordinator 会把消费组的订阅信息返回给 leader consumer
3. leader consumer 根据这些订阅信息然后进行分配方案在给发协调者
4. 然后Coordinator 把leader 分配的方案发送给 consumer 达成同步

rebalance 发生的时机

- 新的 consumer 加入或离开
- 订阅的 topic 发生改变
- 订阅的分区发生改变

消费组的状态

- Empty
- Dead
- PreparingRebalance
- CompletingRebalance
- Stable

commitFailedException 异常
这个异常是consumer 连续两次poll的间隔超过了 max.poll.interval.ms 参数，这个时候会把消费者下线 进行rebalance

- 增加 max.poll.interval.ms 的时间
- 减少 max.poll.records 参数值，poll方法一次性返回消息数量

-----

关于kafka 的底层索引文件设计
通过mmap 做内存映射，即 java 中 mappedBytedBuffer
Offset index 为8个字节 (offset 4个字节，物理磁盘位置 4个字节) 这里的 offset 是相对位移，减去了 baseoffset
Time index 为12个字节（时间戳 8个字节，物理磁盘 4个字节）
kafka 把一整个文件系统做了切分，每一个切分叫做 segment，这些segment 的映射存在内存中的 跳表中，方便范围查询
在2.5 版本中也对这个对了优化：对segment 进行的区分，热冷的区别，这样的话每次来获取segment 先去 热的部份进行查找

-----

⭐ 关于 unclean.leader.election.enable 参数
默认是false，它的意思是就是当leader 分区宕机之后重新选择新的 分区只能从 isr中，反之可以从 osr 中获取
这就是一个典型的 cp/ap 的切换

-----

关于生产者的分区机制
自定义的话实现 partitioner，否则就是默认的会使用 sticky partitioner 黏性分区
这个黏性分区主要是解决 key 消息分散到小 batch问题

- 主要思路单个分区发送所有无key消息，一旦一个分区的batch满了就会随机选择另一个分区尽可能使用完该分区
- 这样，一旦我们拉长整个运行时间，消息还是能均匀地发布到各个分区上，避免出现分区倾斜，同时Producer还能降低延时，因为这个分配过程中始终能确保形成较大的batch，而非小batch。

-----

如何确保kafka 一定不会丢消息

- retries > 0
- ack = -1，需要isr 全部都同步到
- unclean.leader.election.enable = false
- replication.factor >= 3
- min.insync.replicas > 1
- enable.auto.commit = fasle

-----

Kafka 为什么这么快

1. partition 的物理结构决定了顺序写，这和磁盘写的物理结构是切合的
2. 多个 partition 可以分布式写，增加的并发度
3. 追加写，时间复杂度为 O1
4. 批量的写入
5. 以及端到端的数据压缩
6. 零拷贝，跳过了「用户态缓冲区」的拷贝，通过mmap 建立一个磁盘空间和内存的直接映射，数据不再复制到 「用户态缓冲区」
7. PageCache

-----

⭐ Kafka 如何保证消息的顺序？顺序包含了写的顺序和读的顺序？
写：
Kafka 分布式的单位是 partition，partition 是一个追加写的队列结构
发送1条消息的时候，可以指定（topic,partition,key）三个参数，partition和 key 是可选的，可以自定义选择partition，目前的默认的策略是黏性partition
所以想要写顺序的话，那么就可以根据给消息塞入特定的 key 然后继承 Partitioner 接口进而实现 该 key 的消息都往特定的 partition 发送，就可以保证写顺序
读：在多个 partition 情况下，是无法做到顺序读的，因为这些消费者是并发的消费，所以无法保证。如果应用场景对数据有序性要求很高，那么设置一个分区。

----

关于 ack 机制
什么是：当客户端发送一条消息给服务端的时候，服务端返回响应，这个就是ack，kafka 的ack 机制默认有以下三种：
1：默认值，至少一次，会重复发送
0：最多一次
-1：精确一次，它需要 kafka 的所有 isr 中的 partition 都接受到才可以算成功

----

⭐ 关于幂等的实现
什么是：幂等性是指发送同样的请求，对系统资源的影响是一样的，这个需要客户端和服务端相互配合
Producer：这里有两个问题是
- 向服务端发送消息，但是此时连接就断开了，发送的消息经过网络传输时，被丢失了，服务端没有接受到消息
- 服务端接受到消息了，但是在响应客户端的时候失败了。
这两种情况下都会导致producer 重发，所以到 broker 中之后需要再一次去重，
所以需要打开enable.idempotence 配置项，该配置项需要 ack 设置为-1，通过这两个配置可以满足发送消息一定被broker 接受到
该配置项会为消息分配一个唯一的id，但是只支持单个partition
Consumer：不能自动ack，需要手动的ack，因为你在消费消息执行逻辑的时候可能出错，所以需要手动ack

-----

关于消息堆积
登录Kafka Manager，找出消息堆积的消费组，观察消费组是否有消费者在消费，如果有，让业务方加快消费效率，如果没有，让客户酌情删掉不使用的消费组。

-----

关于突然消费不到消息怎么排查


-----

Kafka 分区过多引发的弊端
内存开销：
客户端 producer 有个参数 batch.size 默认是 16kb，这个 batch 存在 RecordAccumulator 中，它会为每个分区缓存消息，一旦满了就会发送，这个设计一般来说是为了提升性能批量发送，
但是这个参数是分区级别的参数，如果分区数越多，这部分缓存所需的内存占用也会越多。
> 假设有10000个分区，按照默认配置，这部分缓存就会占用 157mb 的内存，而对于消费者来说，
> 一般消费者的数量是要和分区数量一致的，所以说消费者配置10000个线程，在线程上下文切换的开销就很大
> 而且在 controller、FetcherManager 中都会维护这些分区的缓存，所以分区越多，缓存的成本越多
文件句柄开销：
每个 parition 在文件系统上会对应一个目录， .log .index .timeindex，kafka 会一直保持这些文件句柄，所以如果partition 越多，
那么文件句柄也会月来与的多，会很快突破单台 broker 的 ulimit-n 的上限
链路延迟：
kafka 的链路延迟也就是 producer 端发布消息到 consumer 端接受消息所需要的时间
kafka 只有在消息提交后，才会将消息暴露给消费者，期间消息需要在 in-sync 副本中完成同步复制，这是耗时的最要部分，默认情况下，每个 broker 从其他 broker 节点进行数据副本同步时，该节点会为此分配一个线程，该线程需要完成该 broker 上所有 partition 数据的复制。
> 一般将 1000 个 partition 从一个 broker 到另一个 broker 所需要的时间延迟为 20ms
SLA：kafka 是通过 replica 机制来提供高可用的，来保证 SLA，每个 partition 都会有多个副本，每个副本分别在于不同的 broker 上，所以的数据副本中，有一个副本
会被选举为leader，负责处理 producer 和 consumer 的读写，其他为 follower，由 kafka controller 负责保证与 leader 的同步。
当 leader 不可用时，会从 follower 中选举新的 leader，这中间会有短暂的不可用时间，大部分是毫秒级的。假设一个partition 恢复时间为5ms，那么1000个就是5s

-----

kafka 一条消息最大是多少？
默认是 1MB， 可以通过 message.max.bytes 进行设置


⭐ ----     
关于 kafka controller
是什么：是kafka broker 的协调者，主要用于选leader 分区、rebalance 分区、以及追踪 broker 的一些信息，它会把元数据存入zk中
但是在新版的 kafka 中，已经把 zk 移除了，加入了 raft 协议
controller 它通过堵塞队列做到与其他 broker 通信的解耦，它会被为每个 broker 都创建一个 RequestSendThread 线程，这些线程不断的从堵塞队列中获取请求发送

controller broker 的选举：在我看的源码版本中是通过zk的 /controller 节点进行抢占的
1. 会通过 ControllerChangeHandler 来监听 zk /controller 节点
2. 如果节点没有则进行抢占，有则直接返回
3. 一旦有broker 抢占成功则它就是 controller，它会进行注册各种zk的监听器、 删除日志路径变更和 ISR 副本变更通知事件、启动 controller 通道管理器以及启动副本状态机
发生网络分区后，kafka 会怎么样
1. 首先要查看是否出现脑裂，即同时有多个 controller 组件
2. 由于 controller 会给 broker 发送 3类请求即 LeaderAndIsrRequest、StopReplicaRequest、UpdateMetadataRequest，因此，一旦网络分区这些
请求将不能顺利达到 broker 整个集群会出现僵死的状态
分区 Leader 总是 -1 怎么办

        删除 zk 节点/controller，触发 Controller 重选举，controller 重选举能够为所有主题重刷分区状态，可以有效解决因不一致导致的 Leader 不可用问题
----

⭐ 关于partition leader 的选举策略
什么是 分区leader 选举：所有的分区 leader 选举都是由 broker controller 进行选举的，分别有以下几种策略：

1. OfflinePartition Leader:
   - 每当有分区上线时，就需要执行 Leader 选举。所谓的分区上线，可能是创建了新分区，也可能是之前的下线分区重新上线。这是最常见的分区 Leader 选举场景。
2. ReassignPartition Leader:  
   - 当你手动运行 kafka-reassign-partitions 命令，或者是调用 Admin 的 alterPartitionReassignments 方法执行分区副本重分配时，可能触发此类选举。假设原来的 AR 是[
   1，2，3]，Leader 是 1，当执行副本重分配后，副本集合 AR 被设置成[4，5，6]，显然，Leader 必须要变更，此时会发生 Reassign Partition Leader 选举。
3. PreferredReplicaPartition Leader:
   - 当你手动运行 kafka-preferred-replica-election 命令，或自动触发了 Preferred Leader 选举时，该类策略被激活。所谓的 Preferred Leader，指的是 AR
   中的第一个副本。比如 AR 是[3，2，1]，那么，Preferred Leader 就是 3。
4. ControlledShutdownPartition Leader:
   - Broker 正常关闭时，该 Broker 上的所有 Leader 副本都会下线，因此，需要为受影响的分区执行相应的 Leader 选举

这四种分区策略，基本都是从 AR 集合中获取第一个 分区 来作为 分区leader

----

⭐ kafka 如何调优

- producer 增加存储器每个 batch的大小，非必要的话关闭重试，压缩
- broker 增加同步线程数
- consumer 增加每次拉取的字节数

----

⭐ Consumer 为什么采用单线程来消费
consumer 虽然是单线程消费，但是它是双线程的设计。一个主线程是用户线程用来获取消息；另一个线程是心跳线程，负责向 kafka 汇报消费者信息
这样的设计是为了把两个线程分割开来互补影响；对于获取消息的线程来讲，便于实现异步非堵塞

----

⭐ Follower 副本同步消息的完整流程

1. follower 分区 发送 fetch 请求给 leader 分区，leader 分区会读取底层日志文件中的消息数据
2. 在更新它内存中 follower 副本的 leo 值，更新为 fetch 请求中 fetchOffset 值
3. 最后尝试更新分区高水位，（因为只有当 follower 副本都接受到之后才会把hw 向后移动）
4. follower 接受到 fetch 响应后，会把消息写入到底层日志，接着更新 leo 和 hw 的值

----

⭐  Topic 的删除过程

1. 删除 zk 上所有相关的 znode
2. controller 元数据，并且要同步到其他的 broker
3. 磁盘日志

----

⭐ Group.instance.id 参数
什么是：每个消费组唯一的，设置了该值，该消费者就被称为 静态成员，它能避免不必要的消费组rebalance
有什么用：比如有消费组的逻辑要变更，那么我们重启可能只需要1-2分钟，这个时候如果不配置静态成员则一定会发生rebalance，rebalance 会发生 stw所以越少越好
那么我们可以配置 session.timeout.ms 和这个 group.instance.id 就可以避免无必要的rebalance 和重分区

----

⭐ 时间轮
什么是：如果我们原生使用 DelayQueue 作为延迟队列的话，它的插入和删除的时间复杂度都是 On ，在数据量很大的情况下性能很差
kafka 是怎么运用的：在kafka 中，比如发送一条消息到broker，leader partition 是不能马上的修改自己的 hw 的，它需要让其他的 follower partition 进行同步后才能更新 hw
这是需要一个等待时间的，这个时候就把这个请求放入时间轮，kafka 称作为 炼狱；消费者也是它避免频繁的io 请求，它会有超时时间，因为消费者去fetch 的时候
可以设置它fetch的size，如果不满足则也会放入这个时间轮进行等待
kafka 的设计：每层是 20个刻度，比如一格是延迟10s 那这一层的总延迟时间就是 200s，但是如果我们需要延迟的时间比较长，那么它会向上创建新的时间轮，下一层的时间轮会有上一层的引用
新创建的时间轮的每一格的延迟时间是下一层的总延迟时间。
这每一层时间轮其实就是一个 TimerTaskList 是通过 DelayQueue(PriorityQueue 实现的)存放的是 TimerTask 真正的延迟任务
然后通过 TimingWheel 这个类做包装
所以的话因为大小堆获取对头的时间复杂度是 O1，性能较高

-----

⭐ 消费者和消费组的关系

- RangeAssigner 策略
  是什么：以主题为单位，以数据顺序排列可用分区，以字典顺序排列消费者，将 topic 分区数除以消费者总数，以确定分配给每个消费者的分区书；如果没有平均分配，那么前几个消费者将拥有一个额外的分区

假设 n = 分区数/消费者数量，m = 分区数%消费者数量，那么前 m 个消费者每个分配 n+1 个分区，后面的（消费者数量- m）个消费者每个分配 n 个分区。

比如有 topic1 分区 topic1，2，3 ；topic2 分区 topic1，2，3
consumer1：topic1p1,topic1p2,topic2p1,topic2p2
consumer2：topic1p3,topic2p3

- RoundRobinAssignor 策略：默认 轮询分配

如果同一个消费组内的消费者订阅的信息是不相同的，那么在执行分区分配的时候就不是完全的轮询分配，有可能导致分区分配得不均匀。如果某个消费者没有订阅消费组内的某个主题，那么在分配分区的时候此消费者将分配不到这个主题的任何分区。

举个例子，假设消费组内有3个消费者（C0、C1 和 C2），它们共订阅了3个主题（t0、t1、t2），这3个主题分别有1、2、3个分区，即整个消费组订阅了 t0p0、t1p0、t1p1、t2p0、t2p1、t2p2 这6个分区。具体而言，消费者 C0 订阅的是主题 t0，消费者 C1 订阅的是主题 t0 和 t1，消费者 C2 订阅的是主题 t0、t1 和 t2，那么最终的分配结果为：

消费者C0：t0p0
消费者C1：t1p0
消费者C2：t1p1、t2p0、t2p1、t2p2
可以看 到 RoundRobinAssignor 策略也不是十分完美，这样分配其实并不是最优解，因为完全可以将分区 t1p1 分配给消费者 C1。

消费者C0：t0p0、t0p2、t1p1
消费者C1：t0p1、t1p0、t1p2

- StickyAssignor 策略：尽量保持现有分配，将已经终止的消费者所分配的分区移动到另一个消费者，避免全部分区重新平衡

