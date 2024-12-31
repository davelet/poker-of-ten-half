# “推十点半”纸牌游戏

这是我小时候玩的一种纸牌游戏。主要的规则有：
- 从A到9的牌分别代表1到9点，其余牌（包括JQK和大小王）都是半点；
- 玩家轮流要牌，最多要5张牌；
- 没超过十点半且不足5张也可以停止要牌，留对手一直拿牌到停止；

### 胜负规则
- 双方比较手里的点数，超过10点半的直接输掉本轮，都超过则庄家胜；
- 双方都没有超过10点半则比较手里的牌数量，有达到5张牌的直接获胜；
- 都没有到5张牌，则点数大的玩家获胜，点数相同庄家获胜。

## 轮次
每轮结束手中的牌都弃入废牌堆，继续用剩下的牌游戏。

# 运行
```bash
cargo run
```

# WASM
```bash
cargo install wasm-server-runner

export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner

cargo run --target wasm32-unknown-unknown

```


但是使用`wasm-bindgen`搭配`trunk`会不成功，加载不了静态资源。
```bash
rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --out-dir ./out --target web target/wasm32-unknown-unknown/release/poker-of-ten-half.wasm

trunk serve
```