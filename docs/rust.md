# Rust

### use 키워드

use 키워드는 스코프 내에서 호출하고 싶어하는 함수의 모듈을 가져옴

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

### derive 키워드
