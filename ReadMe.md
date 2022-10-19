## 模块系统管理
随着模块的逐渐变大，单一的文件无法将整个crate中整个模块的内容包含其中，此时可以将每个模块的代码单独保存在文件/目录中，再通过在上级模块中声明下级模块，从而实现将模块内容分成不同文件的操作。
## 模块拆分规则
+ 如果foo模块没有子模块，将foo模块的代码放在foo.rs文件中。
+ 如果foo模块由子模块，有两种处理方式：
  + 将foo模块代码放在foo.rs文件中，并将其子模块所在文件存放在foo/目录。
  + 将foo模块的代码放在foo/mod.rs中，并将其子模块所在的文件存放在foo/目录。

```
|-phrases_lib/
    |-Cargo.toml				# package配置文件
    |-src/
        |-lib.rs				# library crate入口
        |						# 使用第一种方式管理模块
        |-chinese.rs            # > 该文件存放chinese模块的内容，并包含使用mod关键字挂载它的子模块的语句
        |-chinese/				# > 该目录下存放chinese模块下属的子模块的内容
            |-farewells.rs
            |-greetings.rs
        |						# 使用第二种方式管理模块
        |-english/              # > 该目录下存放english模块自身内容及子模块的内容
            |-mod.rs			# > 该目录下存放english模块自身的内容，并包含使用mod关键字挂载它的子模块的语句
            |-farewells.rs		# > 该文件存放english模块的子模块的内容
            |-greetings.rs		# > 该文件存放english模块的子模块的内容
```
+ 注意mod关键字和use关键字的功能的区别：
  + mod:*用于声明模块之间的结构关系，在一个模块的源文件中使用mod起到的作用是声明挂载子模块，当然如果子模块比较简单的时候也可以在模块中声明的同时定义子模块内部逻辑。*
  + use:*用于声明要引用其它模块的项的信息，引入一个项，从而可以在当前上下文中直接使用这个项。当然很多时候引用的项会是模块，struct,enum等。它实际上是简化路径的一种技术。*
## Cargo 工作空间
+ 前面有讲到，一个Package可以管理多个Crate，但是想要只使用一个Package管理一个大项目还是有所不妥。
+ Cargo工作空间(workspaces)的功能类似于Maven的Module，即将一个庞大的项目拆分成多个功能相对独立的Package，比如说将项目拆分为三Package：product_management,order_management,user_management。
+ 把整个大项目的每一个Package都放到一个工作空间中进行管理，使用工作空间后项目的结构大致如下：

```
my_web_shop/
    ├── Cargo.toml				# 工作空间的配置文件
    ├── Cargo.lock
    ├── product_management/		# 第一个Package
    │   ├── Cargo.toml
    │   └── src/
    │   	├── main.rs
    │       └── lib.rs
    ├── order_management/		# 第一个Package
    │   ├── Cargo.toml
    │   └── src
    │   	├── main.rs
    │       └── lib.rs
    ├── user_management			# 第二个Package
    │   ├── Cargo.toml
    │   └── src
    │   	├── main.rs
    │       └── lib.rs
    └── target/					# 编译输出

```
+ my-web-shop的Cargo.toml(包含工作空间的配置信息)如下：
```
[workspace]
members = [
    "product_management",
    "order_management",
    "user_management"
]
```
+ 目前使用cargo new指令无法之间创建工作空间，因此还是建议先新建一个目录，然后手动创建一个Cargo.toml文件，然后手动加上[workspace]以指定该目录是一个工作空间；
+ 当处于工作空间所在根目录下执行cargo new创建Package时，cargo并不会直接在Cargo.toml的workspace.members中增加新Package的信息(即不会把新创建的包纳入工作空间管理中)，终端只会返回信息当前目录是一个工作目录，此时需要手动设置一下workspace.members；

+ 工作空间在顶级目录有一个 target 目录，每一个Package内没有target目录，即使进入子Package中运行cargo build，生成的结果依然会保存在工作空间/target中，这样让所有的Package共享一个target目录，可以避免其他Package多余的重复构建。
+ 子Package之间互相依赖：默认情况下cargo不假定工作空间中的crate会互相依赖，所欲要显式声明crate之间的依赖关系。具体做法为，比如order_management依赖同级的user_management，则在order_management/Cargo.toml中需要声明：
```
[dependencies]
user_management = { path = "../user_management" }
```
+ 如果要在工作空间中运行指定的二进制crate，需要增加-p参数和包名称来指定：
```
cargo run -p product_management
```
+ **由此可见Cargo.toml可以作为Package的配置，也可以作为工作空间的配置。**

+ 工作空间中使用外部依赖：
  + 由于整个工作空间及子目录中只有根目录的一个Cargo.lock，因此工作空间根上的src/*.rs使用的依赖与每一个子Package的依赖的所有版本信息都交由工作空间根目录的Cargo.lock约束。
  + 一个子Package A依赖了某个外部库a，如果子Package B或者工作空间的根没有在对应的Cargo.toml中声明使用a，那么只有Package A能使用外部库a，但是约束信息还是会保存在根Cargo.lock。
  + 如果其它Package或者根也使用外部库a，则由于Cargo.lock的存在，会强制要求工作空间中任何地方都只能用相同版本的外部库a。

+ 工作空间中测试：
  + 运行所有测试：cargo test
  + 指定某个子crate测试：cargo test -p product_management

+ 工作空间中发布：如果需要单独发布每一个子Package，需要进入到对应的目录中执行cargo publish。

## 编译不同版本
### 安装Target
1.linux64位安装：
+ `rustup target add x86_64-unknown-linux-musl`
+ `rustup target list`

2.可支持的平台汇总：<https://doc.rust-lang.org/nightly/rustc/platform-support.html#platform-support>

### 交叉编译配置
路径：/.cargo/config.toml
内容：
```
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
rustflags = ["-C", "linker-flavor=ld.lld"]
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```
### 生成对应平台的可执行文件
项目目录执行：
1. windows
  + release: cargo build --release --target=x86_64-pc-windows-msvc
  + debug：cargo build --target=x86_64-pc-windows-msvc
2. linux
  + release: cargo build --release --target=x86_64-unknown-linux-musl
  + debug：cargo build --target=x86_64-unknown-linux-musl