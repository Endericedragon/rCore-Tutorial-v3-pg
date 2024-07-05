# rCore改造记录

## 从SNT的pallets到rCore

本课题的理想目标为：将SNT从Linux用户态迁移到rCore内核态。实现这项目标，首先需要分析SNT中各模块和各依赖项和rCore的兼容性如何，并重点关注那些兼容性不佳的模块和依赖项上。

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
frame-benchmarking = { version = "4.0.0-dev", default-features = false, optional = true, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
# rCore incompatible
frame-support = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
frame-system = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
```

本着兵来将挡水来土掩的原则，我们继续分析`frame-support`和`frame-system`，看看这二者为何会编译不通过。

### frame-support

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
sp-state-machine = { version = "0.28.0", default-features = false, optional = true, path = "../../primitives/state-machine" }
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

### frame-system

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

## 从SNT的runtime到rCore

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
frame-benchmarking = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" }
frame-system-benchmarking = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" }

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

观察上述输出不难得出以下结论：

| 依赖项命名前缀 | 和rCore的兼容情况 |
| --- | --- |
| `frame_*` | 大多无法兼容 |
| `pallet_*` | 完全无法兼容 |
| `sp_*` | 部分兼容，接近五五开 |