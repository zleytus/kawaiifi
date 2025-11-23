# kawaiifi

`kawaiifi` is a cross-platform Wi-Fi scanning and monitoring library.

## Example

``` rust
fn main() {
    if let Some(interface) = kawaiifi::default_interface() {
        if let Ok(scan_results) = interface.scan() {
            println!("{:#?}", scan_results);
        }
    }
}
```
