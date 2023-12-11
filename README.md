# Voxel Vegetables

## Building and Running

### Windows, Linux, macOS

Install cargo, then `cargo run`

Note that WASM (web) is currently [not supported](https://github.com/bbarker/voxel-vegetables/issues/19)

## Development

### Releases

workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
   * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow (it may not auto-trigger,
      but you can still run from Actions -> release-flow)
   * WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more
      build minutes. You might want to consider running the workflow less often or removing some builds from it.i
      **For public repositories the builds are free!**
   * The `credits` directory should be kept up-to-date as it is included in the release workflow.
        
### Debugging

We can conditionally run the game with the [WorldInspectorPlugin](https://github.com/jakobhellermann/bevy-inspector-egui):

```
cargo run --features debug-inspector
```

## [Credits](./credits/CREDITS.md)


# Note: this game is unrelated to the [Voxel Vegetables](https://vox-fox.itch.io/voxel-vegetables) asset pack.

# License

This project is licensed under the [MPL v2](LICENSE.md) except some content of `assets` and the Bevy icons 
in the `build` directory (see [Credits](credits/CREDITS.md)). 

We feel that treating files as units of licensed tech (as in the MPL v2) is a little bit antiquated, and
wish to be more liberal - if you make improvements to any code component, we would like those to be open
source, but if you use only a part of the file, there's no need for you to open source every part of
the file. We'll endeavor to dual-license the project under a new license along these lines at some point.
But for now, know this is the intent.
