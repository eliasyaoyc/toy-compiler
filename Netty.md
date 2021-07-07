-------

关于 Netty
https://zhuanlan.zhihu.com/p/85140317

netty的线程模型，netty如何基于reactor模型上实现的。
>有两个线程一个是： mainReactor 和 subReactor
1. main线程 主要用来accept 各种事件
2. 这些事件被塞入 subReactor 中
3. 然后交给线程池来处理

什么是TCP粘包，拆包。解决方式是什么。
>tcp 是基于流传输的，那么很可能C 发送了2个报文，S 就用一次接受，那么就出现了粘包
>netty 提供了多种的解析器
1. 固定长度
2. 在消息头指定消息长度
3. 换行
4. 标识符
5. 可以自定义

netty 的内存池实现
> 什么是：主要将内存预分配管理起来避免jvm 分配，有效减小内存碎片避免内存浪费，同时也能减少频繁gc带来的性能影响
>
>其实就是一个 jemalloc 内存分配方式，有很多 arena(堆和堆外)，arena 里面是 chunklist，里面装的是 chunk(16kb)，在里面就是page， page是8k，在里面是 subpage(0-512B) 内存分配单元
这个page 是完全二叉树维护的
也就是说，会因为你需要分配的大小，已经arena 已使用的程度来选择是佩芬chunk 还是page 还是subpage
1. 对需要分配的内存进行填充，比如只能是 16b、512b、1k、2k、4k、8k 的倍数，不满足的则会按最接近的进行填充
2. 然后对填充后内存进行判断是需要分配 subpage、page 还是 chunk
3. 先从缓存中进行分配，这个就是类似 jvm中 TLAB 预存在它的  fastthreadlocal 中
4. 超过16m 的内存会直接堆外分配

netty 的 unsafe 类主要做什么
>Unsafe 接口实际上是 Channel 接口的辅助接口，它不应该被用户代码直接调用，实际上的 IO 操作都是有 Unsafe 接口负责完成的
比如 localAddress、 Register(注册channel 到多路复用上) 等

netty 对象池的实现
>

netty 的内存泄露检查
>其实就是通过对 ByteBuf 虚引用的 refcount 来判断的，对于如果应用层没有用非池化的 ByteRef，虽然会被gc掉，但是它内部引用的数据内存还是会被池数据结构引用不能被释放这个需要我们手动释放

使用过Netty遇到过什么问题？
>把一个ChannelHandler 放入多个 ChannelPipeline 中，会出现并发问题

Netty 的执行流程
1. 客户端连接之后是到 channel 中由 mainreactor 线程accept 然后交给 subreactor
2. 然后由 processor 线程获取执行整个 pipeline ，最终返回（pipeline 里面就是你的handler 业务逻辑）

⭐ 零拷贝
>什么是：就是避免了把数据从内核态和用户态来回拷贝的一个过程
> 
>原本需要四个步骤
>1. 数据从磁盘读到内核态read buffer
>2. 数据从内核态read buffer 拷贝到用户态的buffer 中
>3. 在用用户态的buffer 拷贝到 内核态的 socketbuffer 中
>4. 然后从内核态的 socketbuffer 拷贝到 NIC buffer 中
   零拷贝就是省略了 2,3步骤，通过DMA 直接把buffer 怼到 NIC buffer 里

sendfile 和 mmap
- mmap：通过内存映射，将文件映射到内核缓冲区，同时用户空间可以共享内核数据，减少内核和用户空间的拷贝次数
    1. 用户态缓冲区，直接和 内核态缓冲区做共享
    2. 内科态缓冲区直接与 socket 缓冲区做复制
    3. 上下文切换不会变少，拷贝次数从4次变成了3次
- sendfile：
    1. 直接从内核态 DMA 到协议栈
    2. 只要2次拷贝，3次上下文切换
       mmap 适合小数据量读写， sendFile 适合大文件传输
       mmap 可以直接操作数据更加方便
       https://www.cnblogs.com/ericli-ericli/articles/12923420.html

Netty的fashwheeltimer的用法，实现原理，是否出现过调用不够准时，怎么解决。
> 

Netty的心跳处理在弱网下怎么办。
>netty 是通过 idleStateHandler 进行心跳检测的，一般就是我们常规的做法，客户端发心跳过来，然后服务端更新，如果超时就关闭联机诶
可以延长它的超时时间

Netty 如何实现重连
>当我们进行连接后会返回一个 channelfuture
>我们只需要监听这个 channelfuture，断开连接之后会调用channelfuture 的 doConnect 方法，加入我们的业务逻辑就行

Netty的通讯协议是什么样的。
>一般是自定义通讯协议的

说说 Netty 如何实现高性能？
1. 线程模型 reactor
2. 堆外内存，池化
3. 降低锁、队列优化、并发优化

Netty 的高可靠如何体现？
1. 心跳检测，idlestatehandler
2. 内存保护
3. 优雅停机

原生的 NIO 存在 Epoll Bug 是什么？Netty 是怎么解决的？
>原生NIO：SELECT 空轮询，导致cpu 100%
> 
>正常流程就是epoll 会调用 epoll_wait 然后返回事件集合，然后进行轮询
> 
>但是在 一些socket 突然中断之后，在event 集合中会出现 EPOLLERR，则event 集合发生了变化就会触发select，但是这个事件集合是空的，就触发空轮询的
> 
>Netty 解决办法：引用计数，对这种情况进行计数操作，超过512次 重新创建selector

Netty 实现长连接
>什么是：长连接就是在请求头里面加入 keep-alive，就不会断开连接了
> 
>难点
>- 更多的连接: 这个其实很好解决，Netty 就是多路复用的不会为了每个连接创建一个请求，所以很好实现。只要在相对应部署的机器上打开文件描述符的上限就行
>- 更高的QPS: Jprofile 代码优化，jvm 优化
