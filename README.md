# 很懒可能不会有多余时间更新


# 第一次尝试 Rust写个小程序

# 踩坑
#### Js参数使用驼峰命名调用 (找了1个小时😂😂😂)
```
Rust
`pub fn projects(app_config_path: &str)`

Js Call
`invoke('projects',{appConfigPath:''})`
```

#### Rust vec<u8> 不能直接转 JS Uint8Array类型 (天真以为能直接转，结果找了3个小时😂😂😂)
```
Rust
`
fn read_file(file_path: std::path::PathBuf) -> Rt<Vec<u8>>
`
JS 错误🙅
`
RustCallResult<Uint8Array> = await invoke("read_file");
`
JS 正确🙆
`
RustCallResult<number[]> = await invoke("read_file");
Uint8Array::form(number[])
`
```

##  [tarui图标生成](https://tauri.app/v1/guides/features/icons/#creating-the-icons-manually)

# 突然不能打包了 (未获得授权将Apple事件发送给Finder) 可能我装的totalfinder有问题
`
tccutil reset AppleEvents com.binaryage.totalfinder.agent
`
https://totalfinder.binaryage.com/injection-troubles

# 鸣谢
[📚 Tauri Tutorial (系列教程 - 打造属于自己的跨端应用)](https://github.com/lencx/tauri-tutorial)


# 免责声明
### 该软件仅用于个人学习使用
- 禁止商用或者非法用途.
- 禁止商用或者非法用途.
- 禁止商用或者非法用途.
