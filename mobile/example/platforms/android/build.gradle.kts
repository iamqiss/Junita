plugins {
    id("com.android.application") version "8.2.0" apply false
    id("org.jetbrains.kotlin.android") version "1.9.22" apply false
}

tasks.register("buildRust") {
    description = "Build Rust library for Android"
    group = "rust"

    doLast {
        exec {
            workingDir = file("../..")
            commandLine("cargo", "ndk", "-t", "arm64-v8a", "build", "--lib")
        }
    }
}
