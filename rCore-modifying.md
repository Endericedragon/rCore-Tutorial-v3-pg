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

# rCore incompatibility
sp-api = { version = "4.0.0-dev", default-features = false, path = "../../primitives/api", features = [ "frame-metadata" ] }
sp-io = { version = "23.0.0", default-features = false, path = "../../primitives/io" }
sp-weights = { version = "20.0.0", default-features = false, path = "../../primitives/weights" }
sp-staking = { version = "4.0.0-dev", default-features = false, path = "../../primitives/staking" }

# rCore compatibility unknown
sp-runtime = { version = "24.0.0", default-features = false, path = "../../primitives/runtime" }
sp-tracing = { version = "10.0.0", default-features = false, path = "../../primitives/tracing" }
sp-core = { version = "21.0.0", default-features = false, path = "../../primitives/core" }
sp-arithmetic = { version = "16.0.0", default-features = false, path = "../../primitives/arithmetic" }
sp-inherents = { version = "4.0.0-dev", default-features = false, path = "../../primitives/inherents" }
 ```
