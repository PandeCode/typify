###### Personal Site

Over engineered typing with music site written in the yew framework in rust. Using taildwindcss classes for styling.

On push, a github workflow is run to compile and bundle the tailwind, css, assets and lastly the rust-yew code into wasm.
The compiled and bundled output is then pushed to gh-pages branch.

###### Running locally

```bash
trunk serve --open
```
