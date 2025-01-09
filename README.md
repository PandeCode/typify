###### Personal Site

Over engineered personal site written in the yew framework in rust. Using taildwindcss classes for styling.

On push, a github workflow is run to compile and bundle the tailwind, css, assets and lastly the rust-yew code into wasm.
The compiled and bundled output is then pushed to my github pages repo.

###### Running locally

```bash
trunk serve --open
```

```rust
    let raw_data = use_state(|| String::new());
	{
		let raw_data = raw_data.clone();

		use_effect(move || {
			wasm_bindgen_futures::spawn_local(async move {
				let raw_data = raw_data.clone();
				if let Some(rm) = request_get_cache(gh_readme).await {
					raw_data.set(rm);
				};
			});
		});
	}

```
