use crate::config::AndroidBuildTarget;

impl AndroidBuildTarget {
    /// Identifier used in the NDK to refer to the ABI
    pub fn android_abi(self) -> &'static str {
        match self {
            AndroidBuildTarget::ArmV7a => "armeabi-v7a",
            AndroidBuildTarget::Arm64V8a => "arm64-v8a",
            AndroidBuildTarget::X86 => "x86",
            AndroidBuildTarget::X86_64 => "x86_64",
        }
    }

    /// Returns the triple used by the rust build tools
    pub fn rust_triple(self) -> &'static str {
        match self {
            AndroidBuildTarget::ArmV7a => "armv7-linux-androideabi",
            AndroidBuildTarget::Arm64V8a => "aarch64-linux-android",
            AndroidBuildTarget::X86 => "i686-linux-android",
            AndroidBuildTarget::X86_64 => "x86_64-linux-android",
        }
    }

    // Returns the triple NDK provided LLVM
    pub fn ndk_llvm_triple(self) -> &'static str {
        match self {
            AndroidBuildTarget::ArmV7a => "armv7a-linux-androideabi",
            AndroidBuildTarget::Arm64V8a => "aarch64-linux-android",
            AndroidBuildTarget::X86 => "i686-linux-android",
            AndroidBuildTarget::X86_64 => "x86_64-linux-android",
        }
    }

    /// Returns the triple used by the non-LLVM parts of the NDK
    pub fn ndk_triple(self) -> &'static str {
        match self {
            AndroidBuildTarget::ArmV7a => "arm-linux-androideabi",
            AndroidBuildTarget::Arm64V8a => "aarch64-linux-android",
            AndroidBuildTarget::X86 => "i686-linux-android",
            AndroidBuildTarget::X86_64 => "x86_64-linux-android",
        }
    }

    /// Returns the double used by to identify arch specific library path of a platform
    pub fn platform_double(self) -> &'static str {
        match self {
            AndroidBuildTarget::ArmV7a => "arch-arm",
            AndroidBuildTarget::Arm64V8a => "arch-arm64",
            AndroidBuildTarget::X86 => "arch-x86",
            AndroidBuildTarget::X86_64 => "arch-x86_64",
        }
    }

    pub fn toolchain_triple(self) -> &'static str {
        match self {
            // aarch64-linux-android-4.9  arm-linux-androideabi-4.9  llvm  renderscript  x86-4.9  x86_64-4.9
            AndroidBuildTarget::ArmV7a => "arm-linux-androideabi",
            AndroidBuildTarget::Arm64V8a => "aarch64-linux-android",
            AndroidBuildTarget::X86 => "x86",
            AndroidBuildTarget::X86_64 => "x86_64",
        }
    }
}
