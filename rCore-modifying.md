# rCore改造记录

## SNT兼容性测试

本课题的理想目标为：将SNT从Linux用户态迁移到rCore内核态。实现这项目标，首先需要分析SNT中各模块和各依赖项和rCore的兼容性如何，并重点关注那些兼容性不佳的模块和依赖项上。

### 从SNT的pallets到rCore

根据之前在开题报告中分析的各模块依赖关系，发现拓扑排序的第一位就是`pallets`项目。这个项目中可以包含很多子crate，但此时里面只有`template`一个crate，作为唯一一个外部自定义pallet添加入SNT的runtime中。因此，有必要对`template`进行分析。

> 测试依赖项和rCore兼容性的方法如下：将该依赖项加入rCore的`user/Cargo.toml`中，即测试其在用户态下的表现如何。若编译通过则认为兼容，否则认为不兼容。

经过分析，`template`的依赖项对rCore的兼容性情况如下所示：

```toml
[dependencies]
# rCore compatible
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = [
	"derive",
] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-benchmarking = { version = "4.0.0-dev", default-features = false, optional = true, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" } # optional救命
# rCore incompatible
frame-support = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-system = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
```

### 从SNT的runtime到rCore

SNT模块依赖拓扑排序的第二个就是`runtime`项目。上文提及的`pallets`即被它所依赖。其依赖项与rCore的兼容性如下所示：

```toml
[dependencies]
# rCore compatible
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-try-runtime = { version = "0.10.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" }
sp-inherents = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-std = { version = "8.0.0", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# Used for runtime benchmarking
frame-benchmarking = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" } # optional救命
frame-system-benchmarking = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" } # optional救命

# rCore incompatible
pallet-aura = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-balances = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-support = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-grandpa = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-sudo = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-system = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-timestamp = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-transaction-payment = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-executive = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-api = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-block-builder = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-consensus-aura = { version = "0.10.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-consensus-grandpa = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-core = { version = "21.0.0", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-offchain = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-runtime = { version = "24.0.0", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-session = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-transaction-pool = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-version = { version = "22.0.0", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# Used for the node template's RPCs
frame-system-rpc-runtime-api = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-transaction-payment-rpc-runtime-api = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# Local Dependencies
pallet-template = { version = "4.0.0-dev", default-features = false, path = "../pallets/template" }
```

### 从SNT的node到rCore

采用和`runtime`和`pallets/template`相同的测试方法获得结果如下：

```toml
[dependencies]
# rCore compatible
sp-inherents = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# CLI-specific dependencies
try-runtime-cli = { version = "0.10.0-dev", optional = true, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# rCore incompatible
clap = { version = "4.4.2", features = ["derive"] }
futures = { version = "0.3.21", features = ["thread-pool"]}
sc-cli = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-core = { version = "21.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-executor = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-network = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-service = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-telemetry = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-transaction-pool = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-transaction-pool-api = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-offchain = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-statement-store = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-consensus-aura = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-consensus-aura = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-consensus = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-consensus-grandpa = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-consensus-grandpa = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-client-api = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-runtime = { version = "24.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-io = { version = "23.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-timestamp = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-keyring = { version = "24.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-system = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-transaction-payment = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# These dependencies are used for the node template's RPCs
jsonrpsee = { version = "0.16.2", features = ["server"] }
sp-api = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-rpc-api = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-blockchain = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sp-block-builder = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
sc-basic-authorship = { version = "0.10.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
substrate-frame-rpc-system = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
pallet-transaction-payment-rpc = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# These dependencies are used for runtime benchmarking
frame-benchmarking = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-benchmarking-cli = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# Local Dependencies
node-template-runtime = { version = "4.0.0-dev", path = "../runtime" }
```

值得注意的问题在于，`frame-benchmarking`及其相关的依赖实际上是**rCore不兼容**的。在之前的测试中之所以能过测，原因在于它们都有`optional = true`的加持。经过测试，SNT中所有带有`optional = true`的依赖，在rCore中都无法在去掉这个标注之后过测。

### 测试结论

观察上述测试结果，可得出下列结论。

首先，随依赖层级逐渐上升（`pallets/template --> runtime --> node`），rCore和SNT的兼容性越来越差（无法通过编译之依赖项的占比越来越大）。

其次，依赖项的命名前缀和兼容性存在关联，如下表所示。

| 依赖项命名前缀 | 和rCore的兼容情况    |
| -------------- | -------------------- |
| `frame_*`      | 大多无法兼容         |
| `pallet_*`     | 完全无法兼容         |
| `sp_*`         | 部分兼容，接近五五开 |
| `sc_*`         | 完全无法兼容         |

除此以外，我们还发现编译失败的报错存在一些共性。虽然在测试阶段没啥用，但可能对以后我们修复它有用。常见的错误包括：

1. 找不到`Ok, Err, Result`等常见结构体的问题。
2. 找不到`derive`等常见过程宏的问题。

`rustc`为上述错误贴心地给出了修改意见，即用`core`中不依赖具体操作系统的结构体替换掉默认的`std`中的结构体。只有先解决这些共性问题，才能继续深入尝试解决每个依赖项的共性问题。

## 诊断frame-support

根据`cargo metadata`的说法，`frame-support`的`Cargo.toml`位于`~/.cargo/git/checkouts/substrate-7e08433d4c370a21/948fbd2/frame/support/Cargo.toml`。将`~/.cargo/git/checkouts/substrate-7e08433d4c370a21/948fbd2`整个复制到`./dependencies`目录中备用。

经过与`pallets/template`相同的测试方法，`frame-support`的依赖项和rCore的兼容性情况如下：

```toml
[dependencies]
# rCore compatible
serde = { version = "1.0.163", default-features = false, features = ["alloc", "derive"] }
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive", "max-encoded-len"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-metadata = { version = "16.0.0", default-features = false, features = ["current"] }
tt-call = "1.0.8"
macro_magic = "0.4.1"
sp-std = { version = "8.0.0", default-features = false, path = "../../primitives/std" }
paste = "1.0"
bitflags = "1.3"
impl-trait-for-tuples = "0.2.2"
smallvec = "1.8.0"
log = { version = "0.4.17", default-features = false }
frame-support-procedural = { version = "4.0.0-dev", default-features = false, path = "./procedural" }
sp-core-hashing-proc-macro = { version = "9.0.0", path = "../../primitives/core/hashing/proc-macro" }
sp-state-machine = { version = "0.28.0", default-features = false, optional = true, path = "../../primitives/state-machine" } # optional救命
k256 = { version = "0.13.0", default-features = false, features = ["ecdsa"] }
environmental = { version = "1.1.4", default-features = false }
sp-debug-derive = { default-features = false, path = "../../primitives/debug-derive" }
sp-inherents = { version = "4.0.0-dev", default-features = false, path = "../../primitives/inherents" }
sp-arithmetic = { version = "16.0.0", default-features = false, path = "../../primitives/arithmetic" }
sp-tracing = { version = "10.0.0", default-features = false, path = "../../primitives/tracing" }

# rCore incompatibility
sp-api = { version = "4.0.0-dev", default-features = false, path = "../../primitives/api", features = [ "frame-metadata" ] }
sp-io = { version = "23.0.0", default-features = false, path = "../../primitives/io" }
sp-weights = { version = "20.0.0", default-features = false, path = "../../primitives/weights" }
sp-staking = { version = "4.0.0-dev", default-features = false, path = "../../primitives/staking" }
sp-core = { version = "21.0.0", default-features = false, path = "../../primitives/core" }
sp-runtime = { version = "24.0.0", default-features = false, path = "../../primitives/runtime" }
```

可见只有6个`sp_*`依赖和rCore不兼容。接下来就应该深挖这些依赖，看看如何将它们变得兼容。

## frame-system

根据`cargo metadata`的说法，`frame-system`的`Cargo.toml`位于`~/.cargo/git/checkouts/substrate-7e08433d4c370a21/948fbd2/frame/system/Cargo.toml`。采用和`frame-support`类似的办法，将其依赖一个一个加入rCore的`Cargo.toml`中进行编译测试，获得如下结果：

```toml
[dependencies]
# rCore compatible
cfg-if = "1.0"
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
log = { version = "0.4.17", default-features = false }
scale-info = { version = "2.5.0", default-features = false, features = ["derive", "serde"] }
serde = { version = "1.0.163", default-features = false, features = ["derive", "alloc"] }
sp-std = { version = "8.0.0", default-features = false, path = "../../primitives/std" }

# rCore incompatible
frame-support = { version = "4.0.0-dev", default-features = false, path = "../support" }
sp-core = { version = "21.0.0", default-features = false, path = "../../primitives/core", features = ["serde"] }
sp-io = { version = "23.0.0", default-features = false, path = "../../primitives/io" }
sp-runtime = { version = "24.0.0", default-features = false, path = "../../primitives/runtime", features = ["serde"] }
sp-weights = { version = "20.0.0", default-features = false, path = "../../primitives/weights", features = ["serde"] }
sp-version = { version = "22.0.0", default-features = false, path = "../../primitives/version", features = ["serde"] }
```

和`frame-support`相比，`frame-system`的依赖项中和rCore不兼容的情况占比更高，且存在重合。