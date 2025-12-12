# Migration Guide: v1.0.4 → v1.0.5

## Critical Bug Fix

Version 1.0.5 fixes a **critical bug** where the library was showing its own package name ("clap-version-flag") instead of the user's application name.

## What Changed?

### 1. Removed `from_cargo()` Method ❌

The `ColorfulVersion::from_cargo()` method has been **removed** because it was incorrectly using the library's own package information instead of the caller's.

### 2. Macro Now Works Correctly ✅

The `colorful_version!()` macro now correctly uses the caller's package information through proper macro expansion.

## How to Migrate

### If You're Using the Macro (Recommended) ✅

**Good news!** If you're already using the macro, you just need to update your dependency version. No code changes required!

```toml
[dependencies]
clap-version-flag = "1.0.5"  # Update from 1.0.4
```

Your code will continue to work:

```rust
use clap_version_flag::colorful_version;

fn main() {
    let version = colorful_version!();
    version.print(); // Now shows YOUR app name, not "clap-version-flag"!
}
```

### If You're Using `from_cargo()` ⚠️

You need to change your code from:

```rust
// ❌ OLD (v1.0.4) - REMOVED IN v1.0.5
use clap_version_flag::ColorfulVersion;

let version = ColorfulVersion::from_cargo();
```

To:

```rust
// ✅ NEW (v1.0.5) - Use the macro instead
use clap_version_flag::colorful_version;

let version = colorful_version!();
```

**Why?** The macro expands `env!()` at your code's location, correctly reading from your `Cargo.toml`, not the library's.

### Manual Creation Still Works

If you need full control, you can still create versions manually:

```rust
use clap_version_flag::ColorfulVersion;

// Option 1: Use env! directly (expands at your location)
let version = ColorfulVersion::new(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_AUTHORS")
);

// Option 2: Use hardcoded values
let version = ColorfulVersion::new("myapp", "1.0.0", "Your Name");
```

## Technical Details

### Why Did This Bug Happen?

In v1.0.4, the `from_cargo()` method was defined like this:

```rust
// ❌ WRONG - env! expands at DEFINITION location
impl ColorfulVersion {
    pub fn from_cargo() -> Self {
        Self {
            package_name: env!("CARGO_PKG_NAME"),  // Gets "clap-version-flag"
            version: env!("CARGO_PKG_VERSION"),    // Gets library's version
            author: env!("CARGO_PKG_AUTHORS"),     // Gets library's author
            colors: Colors::default(),
        }
    }
}
```

When `env!()` is used in a function, it expands at the **definition location** (inside the library), so it always read the library's own `Cargo.toml`.

### How Is It Fixed?

In v1.0.5, we use a macro instead:

```rust
// ✅ CORRECT - env! expands at CALLER location
#[macro_export]
macro_rules! colorful_version {
    () => {
        $crate::ColorfulVersion::new(
            env!("CARGO_PKG_NAME"),    // Expands in YOUR code
            env!("CARGO_PKG_VERSION"), // Reads from YOUR Cargo.toml
            env!("CARGO_PKG_AUTHORS")  // Gets YOUR author info
        )
    };
}
```

Macros expand at the **call site**, so `env!()` reads from the caller's `Cargo.toml` instead.

## Verification

After migrating, verify the fix works:

```rust
use clap_version_flag::colorful_version;

fn main() {
    let version = colorful_version!();
    
    // This should print YOUR app name, not "clap-version-flag"
    println!("Package: {}", version.package_name());
    
    // Or see it in color:
    version.print();
}
```

## Breaking Changes Summary

| What Changed | v1.0.4 | v1.0.5 | Migration |
|-------------|---------|---------|-----------|
| `colorful_version!()` macro | ❌ Wrong package name | ✅ Correct package name | No change needed |
| `ColorfulVersion::from_cargo()` | ❌ Buggy method | ❌ Removed | Use `colorful_version!()` |
| `ColorfulVersion::new()` | ✅ Works | ✅ Enhanced (accepts `impl Into<String>`) | No change needed |
| Manual creation with `env!()` | ✅ Works | ✅ Works | No change needed |

## FAQ

### Q: Why not just fix `from_cargo()` instead of removing it?

**A:** It's impossible to fix it as a method. Rust's `env!()` macro always expands at the location where it's written. Since `from_cargo()` is written inside the library, `env!()` will always read the library's `Cargo.toml`, not the caller's.

The only solution is to use a macro (which expands at the call site) or have users manually pass the values.

### Q: Will my code break?

**A:** Only if you were using `ColorfulVersion::from_cargo()`. If you were using the `colorful_version!()` macro, your code will continue to work AND the bug will be fixed!

### Q: Do I need to change my tests?

**A:** Yes, if your tests were using `from_cargo()`. Update them to use `colorful_version!()` instead.

### Q: Can I still use custom colors?

**A:** Yes! All color customization methods still work exactly the same:

```rust
// With macro + custom colors
let version = colorful_version!("#FF0000", "#0000FF", "#00FF00", "#FFFF00");

// With manual creation + custom colors
let version = ColorfulVersion::new("app", "1.0", "author")
    .with_hex_colors("#FF0000", "#0000FF", "#00FF00", "#FFFF00")?;

// With RGB colors
let version = ColorfulVersion::new("app", "1.0", "author")
    .with_rgb_colors((255, 0, 0), (0, 0, 255), (0, 255, 0), (255, 255, 0));
```

### Q: What if I need dynamic values at runtime?

**A:** Use `ColorfulVersion::new()` with runtime values:

```rust
let app_name = get_app_name_from_config();
let version = ColorfulVersion::new(app_name, "1.0.0", "Author");
```

The `env!()` approach (via macro) is only for compile-time values from `Cargo.toml`.

## Need Help?

If you encounter any issues during migration:

1. Check the [README.md](README.md) for examples
2. Look at the [examples/](examples/) directory
3. Open an issue on [GitHub](https://github.com/cumulus13/clap-version-flag/issues)

## Deprecation Timeline

- **v1.0.4**: `from_cargo()` exists but is buggy
- **v1.0.5**: `from_cargo()` is removed (current version)
- **Future**: No plans to bring it back (impossible to fix)

Always use `colorful_version!()` macro for best results! 🚀